// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers::{Trigger};

#[macro_export]
macro_rules! fa_quant { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    write!($formatter, "?")
} } }

/// A 9 channel signal multiplexer
#[derive(Debug, Clone)]
pub struct Quant {
}

impl Quant {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const inp : &'static str =
        "Quant inp\n\nRange: (0..1)";
    pub const oct : &'static str =
        "Quant oct\n\nRange: (-1..1)";
    pub const sig : &'static str =
        "Quant sig\n\nRange: (-1..1)";
    pub const keys : &'static str =
        "Quant keys\n";
    pub const DESC : &'static str =
r#"Pitch/Note Quantizer

"#;
    pub const HELP : &'static str =
r#"Quant - A pitch quantizer

"#;

}

impl DspNode for Quant {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{at, out, inp, denorm};

        let inp = inp::Quant::inp(inputs);
        let oct = inp::Quant::oct(inputs);
        let out = out::Quant::sig(outputs);
        let max = at::Quant::keys(atoms);

        for frame in 0..ctx.nframes() {
//            self.idx =
//                (max as f32 * denorm::Mux9::slct(slct, frame))
//                .floor() as u8
//                % max;
//
//            out.write(frame, match self.idx {
//                0 => denorm::Mux9::in_1(in_1, frame),
//                1 => denorm::Mux9::in_2(in_2, frame),
//                2 => denorm::Mux9::in_3(in_3, frame),
//                3 => denorm::Mux9::in_4(in_4, frame),
//                4 => denorm::Mux9::in_5(in_5, frame),
//                5 => denorm::Mux9::in_6(in_6, frame),
//                6 => denorm::Mux9::in_7(in_7, frame),
//                7 => denorm::Mux9::in_8(in_8, frame),
//                _ => denorm::Mux9::in_9(in_9, frame),
//            });
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
        ctx_vals[1].set(0.5);
    }
}
