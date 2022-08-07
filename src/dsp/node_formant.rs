use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Formant {
    inv_sample_rate: f32,
    phase: f32,
}

impl Formant {
    pub fn new(_nid: &NodeId) -> Self {
        Self { inv_sample_rate: 1.0 / 44100.0, phase: 0.0 }
    }
    pub const freq: &'static str = "Formant freq\nBase frequency to oscilate at\n";
    pub const form: &'static str = "Formant form\nFrequency of the formant\n";
    pub const atk: &'static str =
        "Formant atk\nFormant attack bandwidth, controls the general bandwidth";
    pub const dcy: &'static str =
        "Formant dcy\nFormant decay bandwidth, controls the peak bandwidth";
    pub const sig: &'static str = "Formant sig\nGenerated formant signal";
    pub const DESC: &'static str = r#"A direct formant synthesizer

This generates a single formant from a given frequency, formant frequency, as well as attack and decay frequencies.
The attack and decay frequencies both control the bandwidth of the formant, decay the peak of the bandwidth, attack peak.
"#;
    pub const HELP: &'static str = r#"Formant - Single formant synthesizer
This is a formant synthesizer that directly generates the audio, no filters needed.
"#;
}

impl DspNode for Formant {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.inv_sample_rate = 1.0 / srate;
    }

    fn reset(&mut self) {
        self.phase = 0.0;
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        _ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{denorm, inp, out};

        let base_freq = inp::Formant::freq(inputs);
        let formant_freq = inp::Formant::form(inputs);
        let attack_freq = inp::Formant::atk(inputs);
        let decay_freq = inp::Formant::dcy(inputs);
        let out = out::Formant::sig(outputs);

        for frame in 0..ctx.nframes() {
            // get the inputs
            let base_freq = denorm::Formant::freq(base_freq, frame);
            let formant_freq = denorm::Formant::form(formant_freq, frame);
            let attack_freq = denorm::Formant::atk(attack_freq, frame);
            let decay_freq = denorm::Formant::dcy(decay_freq, frame);

            // where the two decays meet
            // clamp to avoid division by 0
            let carrier_center =
                (attack_freq / (attack_freq + decay_freq)).max(1e-6).min(1.0 - 1e-6);

            // where they meet in amplitude
            let carrier_lowest_amplitude =
                if carrier_center * decay_freq > base_freq * 2.0 || base_freq == 0.0 {
                    0.0
                } else {
                    (-(std::f32::consts::PI * carrier_center * decay_freq) / base_freq).exp()
                };

            // make a triangle wave, with the peak at carrier center
            let carrier_base =
                (self.phase / carrier_center).min((1.0 - self.phase) / (1.0 - carrier_center));

            // smoothstep
            let carrier = 1.0
                - ((1.0 - carrier_lowest_amplitude)
                    * (carrier_base * carrier_base * (3.0 - 2.0 * carrier_base)));

            // multiple of the frequency the modulators are at
            let multiple = formant_freq / base_freq.max(1e-6);

            // round them to the closest integer of the formant freq
            let freq_a = multiple.floor();
            let freq_b = freq_a + 1.0;

            // and how much to lerp between them
            let blend = multiple.fract();

            // get the true modulator
            let modulator = (1.0 - blend)
                * if multiple < 1.0 {
                    0.0
                } else {
                    (std::f32::consts::TAU * self.phase * freq_a).cos()
                }
                + blend * (std::f32::consts::TAU * self.phase * freq_b).cos();

            // entire wave
            let wave = carrier * modulator;

            // increment phase (very imporant)
            self.phase += base_freq * self.inv_sample_rate;

            // wrap around
            self.phase = self.phase.fract();

            out.write(frame, wave);
        }
    }
}