// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    denorm_offs, inp, out, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
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
    pub const freq: &'static str = "Sin freq\nFrequency of the oscillator.\n\nRange: (-1..1)\n";
    pub const det: &'static str = "Sin det\nDetune the oscillator in semitones and cents. \
         the input of this value is rounded to semitones on coarse input. \
         Fine input lets you detune in cents (rounded). \
         A signal sent to this port is not rounded.\n\
         Note: The signal input allows detune +-10 octaves.\
         \nRange: (Knob -0.2 .. 0.2) / (Signal -1.0 .. 1.0)\n";
    pub const sig: &'static str = "Sin sig\nOscillator signal output.\n\nRange: (-1..1)\n";

    pub const DESC: &'static str = r#"Sine Oscillator

This is a very simple oscillator that generates a sine wave.
"#;

    pub const HELP: &'static str = r#"Sin - A Sine Oscillator

This is a very simple oscillator that generates a sine wave.
The 'freq' paramter specifies the frequency, and the 'det' parameter
allows you to detune the oscillator easily.

You can send any signal to these input ports. The 'det' parameter takes
the same signal range as 'freq', which means, that a value of 0.1 detunes
by one octave. And a value 1.0 detunes by 10 octaves. This means that
for 'det' to be usefully modulated you need to attenuate the modulation input.

You can do FM with this node, but for easy FM synthesis there are other
nodes available.
"#;
}

impl DspNode for Sin {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate;
    }

    fn reset(&mut self) {
        self.phase = self.init_phase;
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
        ctx_vals: LedPhaseVals,
    ) {
        let o = out::Sin::sig(outputs);
        let freq = inp::Sin::freq(inputs);
        let det = inp::Sin::det(inputs);
        let isr = 1.0 / self.srate;

        let mut last_val = 0.0;
        for frame in 0..ctx.nframes() {
            let freq = denorm_offs::Sin::freq(freq, det.read(frame), frame);

            last_val = fast_sin(self.phase * TWOPI);
            o.write(frame, last_val);

            self.phase += freq * isr;
            self.phase = self.phase.fract();
        }

        ctx_vals[0].set(last_val);
    }
}
