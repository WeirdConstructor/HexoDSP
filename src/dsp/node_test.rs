// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, GraphFun, GraphAtomData, DspNode, LedPhaseVals};
use crate::dsp::helpers::{TrigSignal};

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
    trig_sig:   TrigSignal,
    trigger:    bool,
}

impl Test {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            trigger:    false,
            trig_sig:   TrigSignal::new(),
        }
    }

    pub const f : &'static str = "F Test";
    pub const p : &'static str = "Test p\nAn unsmoothed parameter for automated tests.";
    pub const trig: &'static str = "Test trig\nA trigger input, that will create a short pulse on the 'tsig' output.\nRange: (-1..1)";
    pub const sig : &'static str = "Test sig\nThe output of p as signal";
    pub const tsig : &'static str = "Test tsig\nA short trigger pulse will be generated when the 'trig' input is triggered.";

    pub const DESC : &'static str = r#""#;
    pub const HELP : &'static str = r#""#;

}

impl DspNode for Test {
    fn outputs() -> usize { 2 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.trig_sig.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.trig_sig.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], _inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], _led: LedPhaseVals)
    {
        use crate::dsp::{out_idx, at};

        let p    = at::Test::p(atoms);
        let trig = at::Test::trig(atoms);
        let tsig = out_idx::Test::tsig();

        let (out, tsig) = outputs.split_at_mut(tsig);
        let out  = &mut out[0];
        let tsig = &mut tsig[0];

        let mut trigger = trig.i();
        if !self.trigger && trigger > 0 {
            self.trigger = true;

        } else if !self.trigger && trigger == 0 {
            self.trigger = false;

        } else if self.trigger {
            trigger = 0;
        }

        for frame in 0..ctx.nframes() {
            if trigger > 0 {
                self.trig_sig.trigger();
                trigger = 0;
            }

            out.write(frame, p.f());
            let t = self.trig_sig.next();
            tsig.write(frame, t);
        }
    }

    fn graph_fun() -> Option<GraphFun> {
        Some(Box::new(|_gd: &dyn GraphAtomData, _init: bool, x: f32| -> f32 {
            x
        }))
    }
}
