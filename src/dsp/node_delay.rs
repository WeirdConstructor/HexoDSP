// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, GraphFun, GraphAtomData, DspNode, LedPhaseVals};

#[macro_export]
macro_rules! fa_dly_s { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
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
pub struct Delay {
}

impl Delay {
    pub const DESC : &'static str = r#""#;
    pub const HELP : &'static str = r#""#;

    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const f : &'static str = "F Delay";
    pub const p : &'static str = "Delay p\nJust an unsmoothed parameter for tests.";
    pub const sig : &'static str = "Delay sig\nThe output of p as signal";
}

impl DspNode for Delay {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], _inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], _led: LedPhaseVals)
    {
        use crate::dsp::{out, at};

        let p    = at::Delay::p(atoms);
        let out  = out::Delay::sig(outputs);
        for frame in 0..ctx.nframes() {
            out.write(frame, p.f());
        }
    }

    fn graph_fun() -> Option<GraphFun> {
        Some(Box::new(|_gd: &dyn GraphAtomData, _init: bool, x: f32| -> f32 {
            x
        }))
    }
}
