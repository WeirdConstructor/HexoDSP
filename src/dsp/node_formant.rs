use super::helpers::{sqrt4_to_pow4, TrigSignal, Trigger};
use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
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
    pub const inp: &'static str =
        "Ad inp\nSignal input. If you don't connect this, and set this to 1.0 \
        this will act as envelope signal generator. But you can also just \
        route a signal directly through this of course.\nRange: (-1..1)\n";
    pub const trig: &'static str =
        "Ad trig\nTrigger input that starts the attack phase.\nRange: (0..1)\n";
    pub const atk: &'static str =
        "Ad atk\nAttack time of the envelope. You can extend the maximum \
        range of this with the 'mult' setting.\nRange: (0..1)\n";
    pub const dcy: &'static str = "Ad atk\nDecay time of the envelope. You can extend the maximum \
        range of this with the 'mult' setting.\nRange: (0..1)\n";
    pub const ashp: &'static str = "Ad ashp\nAttack shape. This allows you to change the shape \
        of the attack stage from a logarithmic, to a linear and to an \
        exponential shape.\nRange: (0..1)\n";
    pub const dshp: &'static str = "Ad dshp\nDecay shape. This allows you to change the shape \
        of the decay stage from a logarithmic, to a linear and to an \
        exponential shape.\nRange: (0..1)\n";
    pub const mult: &'static str = "Ad mult\nAttack and Decay time range multiplier. \
        This will extend the maximum range of the 'atk' and 'dcy' parameters.";
    pub const sig: &'static str =
        "Ad sig\nEnvelope signal output. If a signal is sent to the 'inp' port, \
        you will receive an attenuated signal here. If you set 'inp' to a \
        fixed value (for instance 1.0), this will output an envelope signal \
        in the range 0.0 to 'inp' (1.0).\nRange: (-1..1)\n";
    pub const eoet: &'static str =
        "Ad eoet\nEnd of envelope trigger. This output sends a trigger once \
        the end of the decay stage has been reached.\nRange: (0..1)";
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
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp, out};

        let base_freq = inp::Formant::freq(inputs);
        let formant_freq = inp::Formant::fmt(inputs);
        let attack_freq = inp::Formant::atk(inputs);
        let decay_freq = inp::Formant::dcy(inputs);
        let out = out::Formant::sig(outputs);

        for frame in 0..ctx.nframes() {
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

            out.write(frame, wave);
        }
    }
}
