// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::helpers::{Rng, Trigger};
use crate::dsp::{
    NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext,
    GraphAtomData, GraphFun,
};

/// A triggered random walker
#[derive(Debug, Clone)]
pub struct RndWk {
    sr_ms:      f32,
    rng:        Rng,
    target:     f32,
    target_inc: f32,
    slew_count: u64,
    current:    f32,
    trig:       Trigger,
}

impl RndWk {
    pub fn new(nid: &NodeId) -> Self {
        let mut rng = Rng::new();
        rng.seed(
            (0x193a67f4a8a6d769_u64).wrapping_add(
                0x262829 * (nid.instance() as u64 + 1)));

        Self {
            rng,
            sr_ms:      44100.0 / 1000.0,
            target:     0.0,
            target_inc: 0.0,
            slew_count: 0,
            current:    0.0,
            trig:       Trigger::new(),
        }
    }

    pub const trig : &'static str =
        "RndWk trig\n\n\nRange: (-1..1)";
    pub const step : &'static str =
        "RndWk step\n\nRange: (-1..1)";
    pub const offs : &'static str =
        "RndWk offs\n\nRange: (-1..1)";
    pub const min : &'static str =
        "RndWk min\n\nRange: (0..1)";
    pub const max : &'static str =
        "RndWk max\n\nRange: (0..1)";
    pub const slewt : &'static str =
        "RndWk slewt\n\nRange: (0..1)";
    pub const sig : &'static str =
        "RndWk sig\nOscillator output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Random Walker
"#;
    pub const HELP : &'static str =
r#"RndWk - Random Walker
"#;

}

impl DspNode for RndWk {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.sr_ms = srate / 1000.0;
    }

    fn reset(&mut self) {
        self.target     = 0.0;
        self.current    = 0.0;
        self.target_inc = 0.0;
        self.slew_count = 0;
        self.trig.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, denorm_offs, at};

        let trig    = inp::RndWk::trig(inputs);
        let step    = inp::RndWk::step(inputs);
        let offs    = inp::RndWk::offs(inputs);
        let min     = inp::RndWk::min(inputs);
        let max     = inp::RndWk::max(inputs);
        let slewt   = inp::RndWk::slewt(inputs);
        let out     = out::RndWk::sig(outputs);

        for frame in 0..ctx.nframes() {
            if self.trig.check_trigger(denorm::RndWk::trig(trig, frame)) {
                let mut min = denorm::RndWk::min(min, frame).clamp(0.0, 1.0);
                let mut max = denorm::RndWk::max(max, frame).clamp(0.0, 1.0);
                if min > max {
                     std::mem::swap(&mut min, &mut max);
                }
                let delta = (max - min).clamp(0.0001, 1.0);

                let step = denorm::RndWk::step(step, frame).clamp(-1.0, 1.0);
                let offs = denorm::RndWk::offs(offs, frame).clamp(-1.0, 1.0);

                self.target =
                    self.current
                    + ((self.rng.next() * 2.0 * step) - step)
                    + offs;
                self.target = ((self.target - min) % delta).abs() + min;

                let slew_time_ms = denorm::RndWk::slewt(slewt, frame);

                if slew_time_ms < 0.01 {
                    self.current    = self.target;
                    self.slew_count = 0;

                } else {
                    let slew_samples = slew_time_ms * self.sr_ms;
                    self.slew_count = slew_samples as u64;
                    self.target_inc = (self.target - self.current) / slew_samples;
                }
            }

            if self.slew_count > 0 {
                self.current += self.target_inc;
                self.slew_count -= 1;
            } else {
                self.target_inc = 0.0;
                self.current    = self.target;
            }

            out.write(frame, self.current);
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }

//    fn graph_fun() -> Option<GraphFun> {
//        let mut osc = VPSOscillator::new(0.0);
//        let israte = 1.0 / 128.0;
//
//        Some(Box::new(move |gd: &dyn GraphAtomData, init: bool, _x: f32, _xn: f32| -> f32 {
//            if init {
//                osc.reset();
//            }
//
//            let v     = NodeId::RndWk(0).inp_param("v").unwrap().inp();
//            let vs    = NodeId::RndWk(0).inp_param("vs").unwrap().inp();
//            let d     = NodeId::RndWk(0).inp_param("d").unwrap().inp();
//            let damt  = NodeId::RndWk(0).inp_param("damt").unwrap().inp();
//            let dist  = NodeId::RndWk(0).inp_param("dist").unwrap().inp();
//
//            let v     = gd.get_denorm(v as u32).clamp(0.0, 1.0);
//            let d     = gd.get_denorm(d as u32).clamp(0.0, 1.0);
//            let vs    = gd.get_denorm(vs as u32).clamp(0.0, 20.0);
//            let damt  = gd.get_denorm(damt as u32);
//            let dist  = gd.get(dist as u32).map(|a| a.i()).unwrap_or(0);
//
//            let v = VPSOscillator::limit_v(d, v + vs);
//            let s = osc.next(1.0, israte, d, v);
//            let s = apply_distortion(s, damt, dist as u8);
//
//            (s + 1.0) * 0.5
//        }))
//    }
}
