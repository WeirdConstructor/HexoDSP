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
            let formant_freq = denorm::Formant::freq(formant_freq, frame);
            let attack_freq = denorm::Formant::freq(attack_freq, frame);
            let decay_freq = denorm::Formant::freq(decay_freq, frame);

            // where the two decays meet
            let carrier_center = decay_freq / (attack_freq + decay_freq);

            // where they meet in amplitude
            let carrier_lowest_amplitude =
                (-std::f32::consts::TAU * base_freq * carrier_center * decay_freq).exp();

            // turn it into a triangle wave
            let carrier_attack = (1.0 - self.phase) / carrier_center;
            let carrier_decay = self.phase / (1.0 - carrier_center);

            // actual triangle wave
            let carrier_base = 1.0 - carrier_attack.min(carrier_decay);

            // smoothstep
            let carrier =
                carrier_base * carrier_base * (3.0 - 2.0 * carrier_base) * carrier_lowest_amplitude
                    + (1.0 - carrier_lowest_amplitude);

            // multiple of the frequency the modulators are at
            let multiple = formant_freq / base_freq;

            // round them to the closest integer of the formant freq
            let freq_a = multiple.floor();
            let freq_b = freq_a + 1.0;

            // and how much to lerp between them
            let blend = multiple.fract();

            // get the true modulator
            let modulator = (1.0 - blend) * (std::f32::consts::TAU * self.phase * freq_a).cos()
                + blend * (std::f32::consts::TAU * self.phase * freq_b).cos();

            // entire wave
            let wave = carrier * modulator;

            // increment phase (very imporant)
            self.phase += base_freq * self.inv_sample_rate;
            self.phase = self.phase.fract();

            out.write(frame, wave);
        }
    }
}
