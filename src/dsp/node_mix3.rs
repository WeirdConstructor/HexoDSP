// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};

/// A 3 channel signal mixer
#[derive(Debug, Clone)]
pub struct Mix3 {
}

impl Mix3 {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const ch1 : &'static str =
        "Mix3 ch1\nChannel 1 Signal input\nRange: (-1..1)\n";
    pub const ch2 : &'static str =
        "Mix3 ch2\nChannel 2 Signal input\nRange: (-1..1)\n";
    pub const ch3 : &'static str =
        "Mix3 ch3\nChannel 3 Signal input\nRange: (-1..1)\n";
    pub const gain1 : &'static str =
        "Mix3 gain1\nChannel 1 gain\nRange: (0..1)";
    pub const gain2 : &'static str =
        "Mix3 gain2\nChannel 2 gain\nRange: (0..1)";
    pub const gain3 : &'static str =
        "Mix3 gain3\nChannel 3 gain\nRange: (0..1)";
    pub const ogain : &'static str =
        "Mix3 ogain\nOutput gain of the sum\nRange: (0..1)";
    pub const sig : &'static str =
        "Mix3 sig\nMixed signal output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"3 Ch. Signal Mixer

A very simple 3 channel signal mixer.
You can mix anything, from audio signals to control signals.
"#;
    pub const HELP : &'static str =
r#"Mix3 - 3 Channel Signal Mixer

Just a small 3 channel mixer to create a sum of multiple signals.
You can mix anything, from audio signals to control signals.

There is even a convenient output gain knob,
to turn down the output.
"#;

}

impl DspNode for Mix3 {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm};

        let inp1 = inp::Mix3::ch1(inputs);
        let inp2 = inp::Mix3::ch2(inputs);
        let inp3 = inp::Mix3::ch3(inputs);
        let g1   = inp::Mix3::gain1(inputs);
        let g2   = inp::Mix3::gain2(inputs);
        let g3   = inp::Mix3::gain3(inputs);
        let og   = inp::Mix3::ogain(inputs);
        let out  = out::Mix3::sig(outputs);

        for frame in 0..ctx.nframes() {
            let sum =
                inp1.read(frame) * denorm::Mix3::gain1(g1, frame)
              + inp2.read(frame) * denorm::Mix3::gain2(g2, frame)
              + inp3.read(frame) * denorm::Mix3::gain3(g3, frame);
            out.write(frame, sum * denorm::Mix3::ogain(og, frame));
        }

        ctx_vals[0].set(
            out.read(ctx.nframes() - 1));
    }
}
