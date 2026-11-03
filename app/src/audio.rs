//! WASAPI event-driven capture. No microphone is opened until Record or Test is pressed.
use crate::{audio_dsp::Node, platform};
use anyhow::{Context, Result};
use potato_core::config::{AudioSource, Filter, FilterKind};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    },
    thread::JoinHandle,
};
use windows::{
    Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Foundation::*,
        Media::Audio::*,
        System::{
            Com::{StructuredStorage::*, *},
            Threading::*,
        },
    },
    core::PCWSTR,
};

#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub render: bool,
}
pub fn devices_for(_: &std::path::Path) -> Result<Vec<Device>> {
    devices()
}
pub fn devices() -> Result<Vec<Device>> {
    let _com = platform::Com::new()?;
    unsafe {
        let e: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let mut out = Vec::new();
        for (flow, render) in [(eRender, true), (eCapture, false)] {
            let collection = e.EnumAudioEndpoints(flow, DEVICE_STATE_ACTIVE)?;
            for i in 0..collection.GetCount()? {
                let device = collection.Item(i)?;
                let id = device.GetId()?;
                let id_string = id.to_string()?;
                CoTaskMemFree(Some(id.0.cast()));
                let properties = device.OpenPropertyStore(STGM_READ)?;
                let mut value = properties.GetValue(&PKEY_Device_FriendlyName)?;
                let name_ptr = PropVariantToStringAlloc(&value)?;
                let name = name_ptr.to_string()?;
                CoTaskMemFree(Some(name_ptr.0.cast()));
                let _ = PropVariantClear(&mut value);
                out.push(Device {
                    id: id_string,
                    name,
                    render,
                });
            }
        }
        Ok(out)
    }
}

#[derive(Default)]
pub struct AudioMetrics {
    pub discontinuities: AtomicU64,
    pub dropped_packets: AtomicU64,
    pub peak: AtomicU32,
    pub frames: AtomicU64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stem {
    pub name: String,
    pub file: PathBuf,
    pub mixed: bool,
    pub isolated: bool,
    pub channels: u16,
    pub rnnoise_delay_samples: u32,
}
struct Packet {
    frame: u64,
    samples: Vec<f32>,
}
pub struct AudioCapture {
    stop: Arc<AtomicBool>,
    capture: Option<JoinHandle<Result<()>>>,
    writer: Option<JoinHandle<Result<()>>>,
    pub metrics: Arc<AudioMetrics>,
    pub stem: Stem,
}
struct Handle(HANDLE);
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}
struct Stream(IAudioClient);
impl Drop for Stream {
    fn drop(&mut self) {
        unsafe {
            let _ = self.0.Stop();
        }
    }
}

impl AudioCapture {
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        name: &str,
        settings: AudioSource,
        loopback: bool,
        filters: Vec<Filter>,
        folder: PathBuf,
        epoch: u64,
        pool: Arc<rayon::ThreadPool>,
        muted: Arc<AtomicBool>,
        _ffmpeg: PathBuf,
    ) -> Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let metrics = Arc::new(AudioMetrics::default());
        let (tx, rx) = crossbeam_channel::bounded::<Packet>(32); // <=320 ms at typical 10 ms WASAPI packets
        let (ready_tx, ready_rx) = crossbeam_channel::bounded(1);
        let file = folder.join(format!("{name}.f32le"));
        let output_file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file)
            .context("Cannot create audio recovery file")?;
        let stem = Stem {
            name: name.into(),
            file: file.clone(),
            mixed: settings.mixed,
            isolated: settings.isolated,
            channels: 2,
            rnnoise_delay_samples: if filters
                .iter()
                .any(|f| f.enabled && f.kind == FilterKind::Suppression)
            {
                960
            } else {
                0
            },
        };
        let s = stop.clone();
        let m = metrics.clone();
        let capture_settings = settings.clone();
        let capture = std::thread::Builder::new()
            .name(format!("wasapi-{name}"))
            .spawn(move || -> Result<()> {
                let _com = platform::Com::new()?;
                let init = || -> Result<(IAudioClient, IAudioCaptureClient, Handle)> {
                    unsafe {
                        let e: IMMDeviceEnumerator =
                            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
                        let device = if capture_settings.device_id.is_empty() {
                            e.GetDefaultAudioEndpoint(
                                if loopback { eRender } else { eCapture },
                                eConsole,
                            )?
                        } else {
                            let id = platform::wide(&capture_settings.device_id);
                            e.GetDevice(PCWSTR(id.as_ptr()))?
                        };
                        let client: IAudioClient = device.Activate(CLSCTX_ALL, None)?;
                        // Windows shared audio engine resamples to 48 kHz, including Bluetooth endpoints.
                        let format = WAVEFORMATEX {
                            wFormatTag: 3,
                            nChannels: 2,
                            nSamplesPerSec: 48000,
                            nAvgBytesPerSec: 384000,
                            nBlockAlign: 8,
                            wBitsPerSample: 32,
                            cbSize: 0,
                        };
                        let flags = AUDCLNT_STREAMFLAGS_EVENTCALLBACK
                            | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM
                            | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY
                            | if loopback {
                                AUDCLNT_STREAMFLAGS_LOOPBACK
                            } else {
                                0
                            };
                        client.Initialize(
                            AUDCLNT_SHAREMODE_SHARED,
                            flags,
                            1_000_000,
                            0,
                            &format,
                            None,
                        )?;
                        let event = Handle(CreateEventW(None, false, false, None)?);
                        client.SetEventHandle(event.0)?;
                        let capture: IAudioCaptureClient = client.GetService()?;
                        client.Start()?;
                        Ok((client, capture, event))
                    }
                };
                let (client, capture, event) = match init() {
                    Ok(v) => {
                        let _ = ready_tx.send(Ok(()));
                        v
                    }
                    Err(e) => {
                        let _ = ready_tx.send(Err(format!("{e:#}")));
                        return Err(e);
                    }
                };
                let _stream = Stream(client);
                while !s.load(Ordering::Relaxed) {
                    unsafe {
                        let _ = WaitForSingleObject(event.0, 100);
                    }
                    while unsafe { capture.GetNextPacketSize()? } > 0 {
                        let mut data = std::ptr::null_mut();
                        let mut frames = 0;
                        let mut flags = 0;
                        let mut device_pos = 0;
                        let mut qpc = 0;
                        unsafe {
                            capture.GetBuffer(
                                &mut data,
                                &mut frames,
                                &mut flags,
                                Some(&mut device_pos),
                                Some(&mut qpc),
                            )?;
                        }
                        // SILENT packets are allowed to return a null data pointer. Never dereference it.
                        let samples = if flags & AUDCLNT_BUFFERFLAGS_SILENT.0 as u32 != 0
                            || data.is_null()
                        {
                            vec![0.; frames as usize * 2]
                        } else {
                            unsafe {
                                std::slice::from_raw_parts(data.cast::<f32>(), frames as usize * 2)
                                    .to_vec()
                            }
                        };
                        unsafe {
                            capture.ReleaseBuffer(frames)?;
                        }
                        if flags
                            & (AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY.0 as u32
                                | AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR.0 as u32)
                            != 0
                        {
                            m.discontinuities.fetch_add(1, Ordering::Relaxed);
                        }
                        let stamp = if qpc == 0
                            || flags & AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR.0 as u32 != 0
                        {
                            platform::qpc_100ns().saturating_sub(frames as u64 * 10_000_000 / 48000)
                        } else {
                            qpc
                        };
                        let frame =
                            (stamp.saturating_sub(epoch) as u128 * 48000 / 10_000_000) as u64;
                        if tx.try_send(Packet { frame, samples }).is_err() {
                            m.dropped_packets.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                Ok(())
            })?;
        match ready_rx.recv_timeout(std::time::Duration::from_secs(5)) {
            Ok(Ok(())) => {}
            result => {
                stop.store(true, Ordering::Relaxed);
                let _ = capture.join();
                anyhow::bail!("Audio initialization failed: {result:?}");
            }
        }
        let m = metrics.clone();
        let writer = std::thread::Builder::new()
            .name(format!("audio-writer-{name}"))
            .spawn(move || -> Result<()> {
                let mut writer = BufWriter::with_capacity(128 * 1024, output_file);
                let mut cursor = 0u64;
                let mut chains: [Vec<Node>; 2] = std::array::from_fn(|_| {
                    filters
                        .iter()
                        .filter(|f| f.enabled)
                        .map(|f| Node::new(f.clone()))
                        .collect()
                });
                let mut planar = [Vec::<f32>::new(), Vec::<f32>::new()];
                let mut bytes = Vec::new();
                let zeros = [0u8; 8192];
                while let Ok(packet) = rx.recv() {
                    let count = packet.samples.len() / 2;
                    // QPC-derived positions preserve gaps and device clock drift. Missing packets become silence.
                    let gap = packet.frame.saturating_sub(cursor);
                    if gap > 48000 * 10 {
                        anyhow::bail!(
                            "Audio clock jumped more than 10 seconds; original PCM preserved"
                        );
                    }
                    let mut padding = gap as usize * 8;
                    while padding > 0 {
                        let n = padding.min(zeros.len());
                        writer.write_all(&zeros[..n])?;
                        padding -= n;
                    }
                    cursor += gap;
                    let skip = cursor.saturating_sub(packet.frame).min(count as u64) as usize;
                    for channel in &mut planar {
                        channel.clear();
                        channel.reserve(count);
                    }
                    let gain = 10f32.powf(settings.gain_db / 20.);
                    for pair in packet.samples.as_chunks::<2>().0 {
                        let pair = if settings.mono {
                            [(pair[0] + pair[1]) * 0.5; 2]
                        } else {
                            [pair[0], pair[1]]
                        };
                        for c in 0..2 {
                            planar[c].push(if settings.mute || muted.load(Ordering::Relaxed) {
                                0.
                            } else {
                                (pair[c] * gain).clamp(-1., 1.)
                            });
                        }
                    }
                    if !filters.is_empty() {
                        pool.install(|| {
                            chains.par_iter_mut().zip(planar.par_iter_mut()).for_each(
                                |(chain, samples)| {
                                    for node in chain {
                                        node.process(samples);
                                    }
                                },
                            );
                        });
                    }
                    let mut peak = 0f32;
                    bytes.clear();
                    bytes.reserve((count - skip) * 8);
                    for i in skip..count {
                        for channel in &planar {
                            let x = if channel[i].is_finite() {
                                channel[i]
                            } else {
                                0.
                            };
                            peak = peak.max(x.abs());
                            bytes.extend_from_slice(&x.to_le_bytes());
                        }
                    }
                    writer.write_all(&bytes)?;
                    cursor += (count - skip) as u64;
                    m.peak.store(peak.to_bits(), Ordering::Relaxed);
                    m.frames.store(cursor, Ordering::Relaxed);
                }
                writer.flush()?;
                writer.get_ref().sync_data()?;
                Ok(())
            })?;
        Ok(Self {
            stop,
            capture: Some(capture),
            writer: Some(writer),
            metrics,
            stem,
        })
    }
    pub fn stop(&mut self) -> Result<()> {
        self.stop.store(true, Ordering::Relaxed);
        let capture = if let Some(h) = self.capture.take() {
            h.join()
                .map_err(|_| anyhow::anyhow!("WASAPI thread panicked"))?
        } else {
            Ok(())
        };
        let writer = if let Some(h) = self.writer.take() {
            h.join()
                .map_err(|_| anyhow::anyhow!("Audio processing thread panicked"))?
        } else {
            Ok(())
        };
        capture.context("Audio device disconnected or capture failed")?;
        writer
    }
    pub fn ended(&self) -> bool {
        self.capture.as_ref().is_some_and(|h| h.is_finished())
            || self.writer.as_ref().is_some_and(|h| h.is_finished())
    }
}
impl Drop for AudioCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
