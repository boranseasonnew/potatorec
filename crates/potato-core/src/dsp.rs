use crate::config::{Filter, FilterKind};

/// One state per channel and filter. Coefficients use the actual audio sample rate.
pub struct Dynamics {
    filter: Filter,
    envelope: f32,
    gain: f32,
    gate_open: bool,
    hold: u32,
}
fn db(x: f32) -> f32 {
    20. * x.max(1e-9).log10()
}
fn linear(x: f32) -> f32 {
    10f32.powf(x / 20.)
}
impl Dynamics {
    pub fn new(filter: Filter) -> Self {
        Self {
            filter,
            envelope: 0.,
            gain: 1.,
            gate_open: false,
            hold: 0,
        }
    }
    pub fn process(&mut self, samples: &mut [f32], rate: f32) {
        let f = &self.filter;
        if !f.enabled || f.kind == FilterKind::Suppression {
            return;
        }
        let attack = (-1. / (f.attack_ms * 0.001 * rate).max(1.)).exp();
        let release = (-1. / (f.release_ms * 0.001 * rate).max(1.)).exp();
        for x in samples {
            if !x.is_finite() {
                *x = 0.;
            }
            let peak = x.abs();
            let coeff = if peak > self.envelope {
                attack
            } else {
                release
            };
            self.envelope = coeff * self.envelope + (1. - coeff) * peak;
            let level = db(self.envelope);
            let target = match f.kind {
                FilterKind::Gain => linear(f.gain_db),
                FilterKind::Compressor => linear(if level > f.threshold_db {
                    (f.threshold_db + (level - f.threshold_db) / f.ratio) - level + f.gain_db
                } else {
                    f.gain_db
                }),
                FilterKind::Expander => linear(if level < f.threshold_db {
                    (level - f.threshold_db) * (f.ratio - 1.)
                } else {
                    0.
                }),
                FilterKind::Limiter => (linear(f.threshold_db) / peak.max(1e-9)).min(1.),
                FilterKind::Gate => {
                    if peak >= linear(f.threshold_db) {
                        self.gate_open = true;
                        self.hold = (f.hold_ms * rate / 1000.) as u32;
                    } else if peak < linear(f.close_db) {
                        if self.hold > 0 {
                            self.hold -= 1;
                        } else {
                            self.gate_open = false;
                        }
                    }
                    if self.gate_open { 1. } else { 0. }
                }
                _ => 1.,
            };
            if f.kind == FilterKind::Limiter {
                self.gain = if target < self.gain {
                    target
                } else {
                    release * self.gain + (1. - release) * target
                };
            } else {
                let c = if target > self.gain { attack } else { release };
                self.gain = c * self.gain + (1. - c) * target;
            }
            *x = (*x * self.gain).clamp(-1., 1.);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn limiter_stops_transient_clipping() {
        let f = Filter {
            enabled: true,
            kind: FilterKind::Limiter,
            threshold_db: -6.,
            ..Filter::default()
        };
        let mut d = Dynamics::new(f);
        let mut s = [0., 0., 1., -1., 0.1];
        d.process(&mut s, 48000.);
        assert!(s.iter().all(|x| x.abs() <= 0.5012));
    }
    #[test]
    fn gate_closes_on_silence() {
        let f = Filter {
            enabled: true,
            kind: FilterKind::Gate,
            ..Filter::default()
        };
        let mut d = Dynamics::new(f);
        let mut s = vec![0.001; 48000];
        d.process(&mut s, 48000.);
        assert!(s.last().unwrap().abs() < 1e-6);
    }
    #[test]
    fn disabled_filter_is_transparent() {
        let mut d = Dynamics::new(Filter::default());
        let mut s = [0.1, -0.2, 0.9];
        let orig = s;
        d.process(&mut s, 48000.);
        assert_eq!(s, orig);
    }
}
