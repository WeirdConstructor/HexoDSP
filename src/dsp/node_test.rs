// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::TrigSignal;

#[macro_export]
macro_rules! fa_test_s {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Zero",
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eigth",
            9 => "Nine",
            10 => "Ten",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Test {
    trig_sig: TrigSignal,
    trigger: bool,
}

impl Test {
    pub fn new(_nid: &NodeId) -> Self {
        Self { trigger: false, trig_sig: TrigSignal::new() }
    }

    pub const f: &'static str = "F Test";
    pub const p: &'static str = "An unsmoothed parameter for automated tests.";
    pub const trig: &'static str =
        "A trigger input, that will create a short pulse on the ~~tsig~~ output.";
    pub const sig: &'static str = "The output of p as signal";
    pub const tsig: &'static str =
        "A short trigger pulse will be generated when the ~~trig~~ input is triggered.";
    pub const out2: &'static str =
        "A test output that will emit **1.0** if output ~~sig~~ is connected.";
    pub const out3: &'static str =
        "A test output that will emit **1.0** if input ~~f~~ is connected.";
    pub const out4: &'static str = "";
    pub const outc: &'static str =
        "Emits a number that defines the out_connected bitmask. Used only for testing!";

    pub const DESC: &'static str = r#""#;
    pub const HELP: &'static str = r#""#;

    fn graph_fun() -> Option<GraphFun> {
        Some(Box::new(|_gd: &dyn GraphAtomData, _init: bool, x: f32, _xn: f32| -> f32 { x }))
    }
}

impl DspNode for Test {
    fn set_sample_rate(&mut self, srate: f32) {
        self.trig_sig.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.trig_sig.reset();
    }

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        atoms: &[SAtom],
        _inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        _led: LedPhaseVals,
    ) {
        use crate::dsp::{at, is_in_con, is_out_con, out_buf, out_idx};

        let p = at::Test::p(atoms);
        let trig = at::Test::trig(atoms);
        let tsig = out_idx::Test::tsig();

        let mut out2 = out_buf::Test::out2(outputs);
        let mut out3 = out_buf::Test::out3(outputs);
        let mut outc = out_buf::Test::outc(outputs);

        let (out, tsig) = outputs.split_at_mut(tsig);
        let out = &mut out[0];
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

            out2.write(frame, if is_out_con::Test::sig(nctx) { 1.0 } else { 0.0 });
            out3.write(frame, if is_in_con::Test::f(nctx) { 1.0 } else { 0.0 });
            outc.write(frame, nctx.out_connected as f32);
        }
    }
}
