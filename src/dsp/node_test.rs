// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, GraphFun, GraphAtomData, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_test_s { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
        let s =
            match ($v.round() as usize) {
                0  => "Zero",
                1  => "One",
                2  => "Two",
                3  => "Three",
                4  => "Four",
                5  => "Five",
                6  => "Six",
                7  => "Seven",
                8  => "Eigth",
                9  => "Nine",
                10 => "Ten",
                _  => "?",
            };
        write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Test {
}

impl Test {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const f : &'static str = "F Test";
    pub const s : &'static str = "S Test";
//  pub const gain : &'static str =
//      "Amp gain\nGain input\nRange: (0..1)\n";
//  pub const sig : &'static str =
//      "Amp sig\nAmplified signal output\nRange: (-1..1)\n";
}

impl DspNode for Test {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, _ctx: &mut T, _ectx: &mut NodeExecContext,
        _atoms: &[SAtom], _params: &[ProcBuf], _inputs: &[ProcBuf],
        _outputs: &mut [ProcBuf], _led: LedPhaseVals)
    {
//        use crate::dsp::out;
//        use crate::dsp::inp;
//        use crate::dsp::denorm;
//
//        let gain = inp::Test::gain(inputs);
//        let inp  = inp::Test::inp(inputs);
//        let out  = out::Test::sig(outputs);
//        for frame in 0..ctx.nframes() {
//            out.write(frame, inp.read(frame) * denorm::Test::gain(gain, frame));
//        }
    }

    fn graph_fun() -> Option<GraphFun> {
        Some(Box::new(|_gd: &dyn GraphAtomData, _init: bool, x: f32| -> f32 {
            x
        }))
    }
}
