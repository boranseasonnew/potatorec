use potato_core::{
    config::{Filter, FilterKind},
    dsp::Dynamics,
};
pub(crate) enum Node {
    Dynamics(Dynamics),
    Rnn {
        state: Box<nnnoiseless::DenoiseState<'static>>,
        input: Box<[f32; 480]>,
        output: Box<[f32; 480]>,
        dry: Box<[f32; 480]>,
        index: usize,
        strength: f32,
    },
}
impl Node {
    pub(crate) fn new(f: Filter) -> Self {
        if f.kind == FilterKind::Suppression {
            Self::Rnn {
                state: nnnoiseless::DenoiseState::new(),
                input: Box::new([0.; 480]),
                output: Box::new([0.; 480]),
                dry: Box::new([0.; 480]),
                index: 0,
                strength: f.strength,
            }
        } else {
            Self::Dynamics(Dynamics::new(f))
        }
    }
    pub(crate) fn process(&mut self, samples: &mut [f32]) {
        match self {
            Self::Dynamics(d) => d.process(samples, 48000.),
            Self::Rnn {
                state,
                input,
                output,
                dry,
                index,
                strength,
            } => {
                for x in samples {
                    let value = *x;
                    *x = output[*index] / 32768. * *strength + dry[*index] * (1. - *strength);
                    input[*index] = value * 32768.;
                    dry[*index] = value;
                    *index += 1;
                    if *index == 480 {
                        state.process_frame(output.as_mut_slice(), input.as_slice());
                        *index = 0;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rnnoise_stream_handles_arbitrary_packet_sizes() {
        let f = Filter {
            kind: FilterKind::Suppression,
            enabled: true,
            ..Filter::default()
        };
        let mut n = Node::new(f);
        for len in [32, 467, 480, 1024, 819] {
            let mut samples = vec![0.; len];
            n.process(&mut samples);
            assert!(samples.iter().all(|s| s.is_finite() && s.abs() <= 1.));
        }
    }
}
