use anyhow::{Context, Result};
use potato_core::config::{Hotkeys, Language, Priority, Source};
use std::{
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
    time::Instant,
};
use windows::{
    Win32::{
        Foundation::*,
        Globalization::*,
        Graphics::Gdi::*,
        Storage::FileSystem::*,
        System::{Com::*, Performance::*, ProcessStatus::*, Threading::*},
        UI::{Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
    core::{BOOL, PCWSTR, w},
};

pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
pub fn command(exe: impl AsRef<std::ffi::OsStr>) -> Command {
    use std::os::windows::process::CommandExt;
    let mut c = Command::new(exe);
    c.creation_flags(0x0800_0000);
    c
}
pub struct Com;
impl Com {
    pub fn new() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        }
        Ok(Self)
    }
}
impl Drop for Com {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
pub fn qpc_100ns() -> u64 {
    unsafe {
        let mut counter = 0;
        let mut freq = 0;
        let _ = QueryPerformanceCounter(&mut counter);
        let _ = QueryPerformanceFrequency(&mut freq);
        if freq <= 0 {
            0
        } else {
            (counter as u128 * 10_000_000 / freq as u128) as u64
        }
    }
}
pub fn os_language() -> potato_core::config::Language {
    let mut name = [0u16; 85];
    let count = unsafe { GetUserDefaultLocaleName(&mut name) };
    if count > 1 {
        potato_core::config::Language::from_code(&String::from_utf16_lossy(
            &name[..count as usize - 1],
        ))
    } else {
        potato_core::config::Language::English
    }
}
pub fn free_space(path: &Path) -> Result<u64> {
    let s = wide(&path.to_string_lossy());
    let mut free = 0;
    unsafe {
        GetDiskFreeSpaceExW(PCWSTR(s.as_ptr()), Some(&mut free), None, None)?;
    }
    Ok(free)
}

pub fn priority_handle(handle: HANDLE, p: Priority) -> Result<()> {
    let class = match p {
        Priority::Low => IDLE_PRIORITY_CLASS,
        Priority::BelowNormal => BELOW_NORMAL_PRIORITY_CLASS,
        Priority::Normal => NORMAL_PRIORITY_CLASS,
        Priority::AboveNormal => ABOVE_NORMAL_PRIORITY_CLASS,
        Priority::High => HIGH_PRIORITY_CLASS,
    };
    unsafe {
        SetPriorityClass(handle, class)?;
    }
    Ok(())
}
pub fn priority(p: Priority) -> Result<()> {
    priority_handle(unsafe { GetCurrentProcess() }, p)
}
pub fn child_priority(pid: u32, p: Priority) -> Result<()> {
    unsafe {
        let h = OpenProcess(PROCESS_SET_INFORMATION, false, pid)?;
        let r = priority_handle(h, p);
        let _ = CloseHandle(h);
        r
    }
}

unsafe extern "system" fn enum_window(hwnd: HWND, data: LPARAM) -> BOOL {
    unsafe {
        if !IsWindowVisible(hwnd).as_bool() || GetWindowTextLengthW(hwnd) == 0 {
            return BOOL(1);
        }
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == std::process::id() {
            if std::env::var_os("POTATO_ALLOW_SELF_CAPTURE").is_none()
                && !std::env::args().any(|a| a == "--ui-test")
            {
                let _ = SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE);
            }
            return BOOL(1);
        }
        if GetWindow(hwnd, GW_OWNER).is_ok_and(|h| !h.is_invalid()) {
            return BOOL(1);
        }
        let mut title = [0u16; 512];
        let n = GetWindowTextW(hwnd, &mut title);
        if n > 0 {
            let list = &mut *(data.0 as *mut Vec<Source>);
            list.push(Source::Window {
                hwnd: hwnd.0 as u64,
                title: String::from_utf16_lossy(&title[..n as usize]),
                pid,
            });
        }
        BOOL(1)
    }
}
unsafe extern "system" fn enum_monitor(m: HMONITOR, _: HDC, _: *mut RECT, data: LPARAM) -> BOOL {
    unsafe {
        let list = &mut *(data.0 as *mut Vec<Source>);
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        let _ = GetMonitorInfoW(m, &mut info);
        let i = list.len();
        list.push(Source::Monitor {
            index: i as u32,
            name: format!(
                "Display {} · {} × {}{}",
                i + 1,
                info.rcMonitor.right - info.rcMonitor.left,
                info.rcMonitor.bottom - info.rcMonitor.top,
                if info.dwFlags == 1 { " · Primary" } else { "" }
            ),
        });
        BOOL(1)
    }
}
pub fn sources() -> Vec<Source> {
    let mut result = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            None,
            None,
            Some(enum_monitor),
            LPARAM(&mut result as *mut _ as isize),
        );
        let _ = EnumWindows(Some(enum_window), LPARAM(&mut result as *mut _ as isize));
    }
    result
}
pub fn minimized(source: &Source) -> bool {
    match source {
        Source::Window { hwnd, .. } => unsafe { IsIconic(HWND(*hwnd as *mut _)).as_bool() },
        _ => false,
    }
}
pub fn open(path: &Path) -> Result<()> {
    let p = wide(&path.to_string_lossy());
    let r = unsafe {
        ShellExecuteW(
            None,
            w!("open"),
            PCWSTR(p.as_ptr()),
            None,
            None,
            SW_SHOWNORMAL,
        )
    };
    anyhow::ensure!(
        r.0 as isize > 32,
        "Windows could not open {}",
        path.display()
    );
    Ok(())
}

#[derive(Debug, Clone)]
pub enum NativeEvent {
    Record,
    Pause,
    Mute,
    Show,
    OpenFolder,
    Exit,
    Replay,
    Error(String),
}
pub struct NativeBridge {
    pub rx: crossbeam_channel::Receiver<NativeEvent>,
    hwnd: Arc<Mutex<isize>>,
    join: Option<std::thread::JoinHandle<()>>,
    shutdown: Arc<std::sync::atomic::AtomicBool>,
}
impl NativeBridge {
    pub fn start(keys: Hotkeys, language: Language, ctx: eframe::egui::Context) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let hwnd = Arc::new(Mutex::new(0isize));
        let store = hwnd.clone();
        let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let closing = shutdown.clone();
        let join = std::thread::spawn(move || {
            let run = || -> Result<()> {
                unsafe {
                    let h = CreateWindowExW(
                        WS_EX_TOOLWINDOW,
                        w!("STATIC"),
                        w!("Potato Recorder Controls"),
                        WS_OVERLAPPED,
                        0,
                        0,
                        0,
                        0,
                        None,
                        None,
                        None,
                        None,
                    )?;
                    *store.lock().unwrap() = h.0 as isize;
                    if closing.load(std::sync::atomic::Ordering::Acquire) {
                        let _ = DestroyWindow(h);
                        return Ok(());
                    }
                    let mut icon = NOTIFYICONDATAW {
                        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                        hWnd: h,
                        uID: 1,
                        uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
                        uCallbackMessage: WM_APP + 1,
                        hIcon: LoadIconW(None, IDI_APPLICATION)?,
                        ..Default::default()
                    };
                    let tip = wide(&format!(
                        "Potato Recorder · F{} {}",
                        keys.record - 0x6f,
                        crate::localization::tr(language, "Start / stop")
                    ));
                    let n = tip.len().min(icon.szTip.len() - 1);
                    icon.szTip[..n].copy_from_slice(&tip[..n]);
                    if !Shell_NotifyIconW(NIM_ADD, &icon).as_bool() {
                        let _ = tx.send(NativeEvent::Error(
                            "System tray icon unavailable; window stays visible".into(),
                        ));
                    }
                    for (i, key) in [
                        keys.record,
                        keys.pause,
                        keys.mute,
                        keys.visibility,
                        keys.replay,
                    ]
                    .iter()
                    .enumerate()
                    {
                        if let Err(e) = RegisterHotKey(Some(h), i as i32 + 1, MOD_NOREPEAT, *key) {
                            let _ = tx.send(NativeEvent::Error(crate::localization::fill(
                                language,
                                "Shortcut F{0} is unavailable: {1}",
                                &[(key - 0x6f).to_string(), e.to_string()],
                            )));
                        }
                    }
                    let menu = CreatePopupMenu()?;
                    for (id, label) in [
                        (1, "Start / stop"),
                        (2, "Pause / resume"),
                        (3, "Mute microphone"),
                        (4, "Show Potato Recorder"),
                        (5, "Open recordings"),
                        (6, "Exit safely"),
                    ] {
                        let text = wide(crate::localization::tr(language, label));
                        AppendMenuW(menu, MF_STRING, id, PCWSTR(text.as_ptr()))?;
                    }
                    let mut msg = MSG::default();
                    loop {
                        let r = GetMessageW(&mut msg, Some(h), 0, 0);
                        if r.0 <= 0 || msg.message == WM_CLOSE {
                            break;
                        }
                        let event = if msg.message == WM_HOTKEY {
                            match msg.wParam.0 {
                                1 => Some(NativeEvent::Record),
                                2 => Some(NativeEvent::Pause),
                                3 => Some(NativeEvent::Mute),
                                4 => Some(NativeEvent::Show),
                                5 => Some(NativeEvent::Replay),
                                _ => None,
                            }
                        } else if msg.message == WM_APP + 1 {
                            if msg.lParam.0 as u32 == WM_LBUTTONDBLCLK {
                                Some(NativeEvent::Show)
                            } else if msg.lParam.0 as u32 == WM_RBUTTONUP {
                                let mut p = POINT::default();
                                let _ = GetCursorPos(&mut p);
                                let _ = SetForegroundWindow(h);
                                let choice = TrackPopupMenu(
                                    menu,
                                    TPM_RETURNCMD | TPM_NONOTIFY,
                                    p.x,
                                    p.y,
                                    Some(0),
                                    h,
                                    None,
                                )
                                .0;
                                match choice {
                                    1 => Some(NativeEvent::Record),
                                    2 => Some(NativeEvent::Pause),
                                    3 => Some(NativeEvent::Mute),
                                    4 => Some(NativeEvent::Show),
                                    5 => Some(NativeEvent::OpenFolder),
                                    6 => Some(NativeEvent::Exit),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        if let Some(event) = event {
                            let _ = tx.send(event);
                            ctx.request_repaint();
                        }
                        let _ = TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                    for i in 1..=5 {
                        let _ = UnregisterHotKey(Some(h), i);
                    }
                    let _ = Shell_NotifyIconW(NIM_DELETE, &icon);
                    let _ = DestroyMenu(menu);
                    let _ = DestroyWindow(h);
                    Ok(())
                }
            };
            if let Err(e) = run() {
                let _ = tx.send(NativeEvent::Error(format!("Native controls: {e:#}")));
                ctx.request_repaint();
            }
        });
        Self {
            rx,
            hwnd,
            join: Some(join),
            shutdown,
        }
    }
}
impl Drop for NativeBridge {
    fn drop(&mut self) {
        self.shutdown
            .store(true, std::sync::atomic::Ordering::Release);
        let h = *self.hwnd.lock().unwrap();
        if h != 0 {
            unsafe {
                let _ = PostMessageW(Some(HWND(h as *mut _)), WM_CLOSE, WPARAM(0), LPARAM(0));
            }
        }
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

pub fn validate_source(source: &Source) -> Result<()> {
    if let Source::Window { hwnd, pid, .. } = source {
        let h = HWND(*hwnd as *mut _);
        let mut current_pid = 0;
        unsafe {
            anyhow::ensure!(
                IsWindow(Some(h)).as_bool(),
                "Selected window no longer exists; refresh capture sources"
            );
            GetWindowThreadProcessId(h, Some(&mut current_pid));
        }
        anyhow::ensure!(
            current_pid == *pid,
            "Selected window handle belongs to a different process; select the source again"
        );
    }
    Ok(())
}

pub struct ProcessSampler {
    last: Instant,
    cpu: u64,
}
impl Default for ProcessSampler {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, Default)]
pub struct Resources {
    pub cpu_percent: f64,
    pub ram_mb: f64,
}
impl ProcessSampler {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
            cpu: 0,
        }
    }
    pub fn sample(&mut self, pid: Option<u32>) -> Resources {
        unsafe {
            let h = if let Some(pid) = pid {
                match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
                    Ok(h) => h,
                    Err(_) => return Resources::default(),
                }
            } else {
                GetCurrentProcess()
            };
            let mut c = FILETIME::default();
            let mut e = c;
            let mut k = c;
            let mut u = c;
            let mut mem = PROCESS_MEMORY_COUNTERS {
                cb: std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
                ..Default::default()
            };
            let _ = GetProcessMemoryInfo(
                h,
                &mut mem,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            );
            let cpu = if GetProcessTimes(h, &mut c, &mut e, &mut k, &mut u).is_ok() {
                ((k.dwHighDateTime as u64) << 32 | k.dwLowDateTime as u64)
                    + ((u.dwHighDateTime as u64) << 32 | u.dwLowDateTime as u64)
            } else {
                0
            };
            let elapsed = self.last.elapsed().as_secs_f64();
            let percent = if self.cpu == 0 || elapsed == 0. {
                0.
            } else {
                cpu.saturating_sub(self.cpu) as f64
                    / 10_000_000.
                    / elapsed
                    / std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(1) as f64
                    * 100.
            };
            self.cpu = cpu;
            self.last = Instant::now();
            if pid.is_some() {
                let _ = CloseHandle(h);
            }
            Resources {
                cpu_percent: percent,
                ram_mb: mem.WorkingSetSize as f64 / 1048576.,
            }
        }
    }
}

pub fn config_path() -> std::path::PathBuf {
    if std::env::args().any(|a| a == "--ui-test") {
        return std::env::current_dir()
            .unwrap_or_else(|_| ".".into())
            .join("test-output")
            .join("ui-test-settings.json");
    }
    std::env::var_os("LOCALAPPDATA")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| ".".into())
        .join("Potato Recorder")
        .join("config.json")
}
pub fn ensure_output(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path).with_context(|| format!("Cannot create {}", path.display()))?;
    anyhow::ensure!(
        free_space(path)? > 512 * 1024 * 1024,
        "Less than 512 MiB free; choose another recording folder"
    );
    Ok(())
}

pub fn validate_source_for(source: &Source, _: &Path) -> Result<()> {
    validate_source(source)
}
pub fn sources_for(_: &Path) -> Vec<Source> {
    sources()
}
pub fn prepare_config(_: &mut potato_core::config::Config) -> Result<()> {
    Ok(())
}
pub fn backends() -> &'static [potato_core::config::Backend] {
    use potato_core::config::Backend;
    &[Backend::Auto, Backend::Wgc, Backend::Dxgi]
}
pub fn priorities() -> &'static [Priority] {
    Priority::ALL
}
pub fn has_tray() -> bool {
    true
}
pub fn system_audio_supported() -> bool {
    true
}
pub fn note() -> &'static str {
    "Windows native capture and audio"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_controls_can_be_dropped_during_startup() {
        for _ in 0..20 {
            let bridge = NativeBridge::start(
                Hotkeys::default(),
                Language::English,
                eframe::egui::Context::default(),
            );
            drop(bridge);
        }
    }
    #[test]
    fn missing_window_is_rejected_before_capture() {
        let source = Source::Window {
            hwnd: 0,
            title: "Closed fixture".into(),
            pid: u32::MAX,
        };
        assert!(validate_source(&source).is_err());
    }
}
