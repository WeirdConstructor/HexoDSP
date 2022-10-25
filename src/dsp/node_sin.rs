// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    denorm, denorm_offs, inp, out, DspNode, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf,
    SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::fast_sin;

/// A sine oscillator
#[derive(Debug, Clone)]
pub struct Sin {
    /// Sample rate
    srate: f32,
    /// Oscillator phase
    phase: f32,
    /// Initial phase offset
    init_phase: f32,
}

const TWOPI: f32 = 2.0 * std::f32::consts::PI;

impl Sin {
    pub fn new(nid: &NodeId) -> Self {
        let init_phase = nid.init_phase();

        Self { srate: 44100.0, phase: init_phase, init_phase }
    }
    pub const freq: &'static str = "Frequency of the oscillator.\n";
    pub const det: &'static str = "Detune the oscillator in semitones and cents. \
         the input of this value is rounded to semitones on coarse input. \
         Fine input lets you detune in cents (rounded). \
         A signal sent to this port is not rounded.\n\
         Note: The signal input allows detune +-10 octaves.\
         ";
    pub const pm: &'static str =
        "Phase modulation input or phase offset. Use this for linear FM/PM modulation.\n";
    pub const sig: &'static str = "Oscillator signal output.\n";

    pub const DESC: &'static str = r#"Sine Oscillator

This is a very simple oscillator that generates a sine wave.
"#;

    pub const HELP: &'static str = r#"A Sine Oscillator

This is a very simple oscillator that generates a sine wave.
The ~~freq~~ parameter specifies the frequency, and the ~~det~~ parameter
allows you to detune the oscillator easily.

You can send any signal to these input ports. The ~~det~~ parameter takes
the same signal range as ~~freq~~, which means, that a value of 0.1 detunes
by one octave. And a value 1.0 detunes by 10 octaves. This means that
for ~~det~~ to be usefully modulated you need to attenuate the modulation input.

For linear FM, you can use the ~~pm~~ input. It allows you to modulate the phase
of the oscillator linearly. It does so *through zero* which means that the pitch
should not detune by the amount of modulation in low frequencies.

You can do exponential FM with this node using the ~~det~~ or ~~freq~~ input,
but for easy exponential FM synthesis there might be other nodes available.
"#;

    fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for Sin {
    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate;
    }

    fn reset(&mut self) {
        self.phase = self.init_phase;
    }

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        let o = out::Sin::sig(outputs);
        let freq = inp::Sin::freq(inputs);
        let det = inp::Sin::det(inputs);
        let pm = inp::Sin::pm(inputs);
        let isr = 1.0 / self.srate;

        let mut last_val = 0.0;
        for frame in 0..ctx.nframes() {
            let freq = denorm_offs::Sin::freq(freq, det.read(frame), frame);

            let mut phase = self.phase + denorm::Sin::pm(pm, frame);
            while phase < 0.0 {
                phase += 1.0
            }
            last_val = fast_sin(phase.fract() * TWOPI);
            o.write(frame, last_val);

            self.phase += freq * isr;
            self.phase = self.phase.fract();
        }

        ctx_vals[0].set(last_val);
    }
}
