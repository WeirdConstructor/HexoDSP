// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::helpers::Rng;
use crate::dsp::{
    NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext,
    GraphAtomData, GraphFun,
};

/// A triggered random walker
#[derive(Debug, Clone)]
pub struct RndWlk {
    israte_ms:  f64,
    rng:        Rng,
    target:     f64,
    target_inc: f64,
    current:    f64,
}

impl RndWlk {
    pub fn new(nid: &NodeId) -> Self {
        let mut rng = Rng::new();
        rng.seed(
            (0x193a67f4a8a6d769_u64).wrapping_add(
                0x262829 * (nid.instance() as u64 + 1)));

        Self {
            rng,
            israte_ms:  1.0 / 44100.0,
            target:     0.0,
            target_inc: 0.0,
            current:    0.0,
        }
    }

    pub const trig : &'static str =
        "RndWlk trig\n\n\nRange: (-1..1)";
    pub const step : &'static str =
        "RndWlk step\n\nRange: (-1..1)";
    pub const offs : &'static str =
        "RndWlk offs\n\nRange: (-1..1)";
    pub const min : &'static str =
        "RndWlk min\n\nRange: (0..1)";
    pub const max : &'static str =
        "RndWlk max\n\nRange: (0..1)";
    pub const slewt : &'static str =
        "RndWlk slewt\n\nRange: (0..1)";
    pub const sig : &'static str =
        "RndWlk sig\nOscillator output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Random Walker
"#;
    pub const HELP : &'static str =
r#"RndWlk - Random Walker
"#;

}

impl DspNode for RndWlk {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.israte_ms = (1.0 / (srate as f64)) / 1000.0;
    }

    fn reset(&mut self) {
        self.target  = 0.0;
        self.current = 0.0;
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm, denorm_offs, at};

        // if trigger
        //    - initialize target
        //
        // let min = min.clamp(0.0, 1.0);
        // let max = max.clamp(0.0, 1.0);
        // if min > max {
        //      std::mem::swap(&mut min, &mut max);
        // }
        // let delta = (max - min).clamp(0.0001, 1.0);
        //
        // if self.trigger.check... {
        //     let step = step.clamp(-1.0, 1.0);
        //     let offs = offs.clamp(-1.0, 1.0);
        //     self.target = self.rng.next() as f64 * step + offs;
        //
        //     if slew_time_ms < 0.01 {
        //          self.current = self.target;
        //          self.current = ((self.current - min) % delta).abs() + min;
        //     } else {
        //          let slew_samples = slew_time_ms * self.israte_ms;
        //          self.target_inc = self.target / slew_samples;
        //     }
        // }
        //
        // if self.target_inc > 0.0 {
        //     self.current += self.target_inc;
        //     if (self.current - self.target).abs() < 0.00001 {
        //         self.target_inc = 0.0;
        //         self.current    = self.target;
        //     }
        //     self.current = ((self.current - min) % delta).abs() + min;
        // }

//        let freq    = inp::RndWlk::freq(inputs);
//        let det     = inp::RndWlk::det(inputs);
//        let d       = inp::RndWlk::d(inputs);
//        let v       = inp::RndWlk::v(inputs);
//        let vs      = inp::RndWlk::vs(inputs);
//        let damt    = inp::RndWlk::damt(inputs);
        let out     = out::RndWlk::sig(outputs);
//        let ovrsmpl = at::RndWlk::ovrsmpl(atoms);
//        let dist    = at::RndWlk::dist(atoms);
//
//        let israte_ms = self.israte;
//
//        let dist       = dist.i() as u8;
//        let oversample = ovrsmpl.i() == 1;
//
//        let mut osc = &mut self.osc;
//
//        if oversample {
//            for frame in 0..ctx.nframes() {
//                let freq = denorm_offs::RndWlk::freq(freq, det.read(frame), frame);
//                let v    = denorm::RndWlk::v(v, frame).clamp(0.0, 1.0);
//                let d    = denorm::RndWlk::d(d, frame).clamp(0.0, 1.0);
//                let vs   = denorm::RndWlk::vs(vs, frame).clamp(0.0, 20.0);
//                let damt = denorm::RndWlk::damt(damt, frame).clamp(0.0, 1.0);
//
//                let v = VPSOscillator::limit_v(d, v + vs);
//
//                let overbuf = self.oversampling.resample_buffer();
//                for b in overbuf {
//                    let s = osc.next(freq, israte, d, v);
//                    *b = apply_distortion(s, damt, dist);
//                }
//
//                out.write(frame, self.oversampling.downsample());
//            }
//
//        } else {
//            for frame in 0..ctx.nframes() {
//                let freq = denorm_offs::RndWlk::freq(freq, det.read(frame), frame);
//                let v    = denorm::RndWlk::v(v, frame).clamp(0.0, 1.0);
//                let d    = denorm::RndWlk::d(d, frame).clamp(0.0, 1.0);
//                let vs   = denorm::RndWlk::vs(vs, frame).clamp(0.0, 20.0);
//                let damt = denorm::RndWlk::damt(damt, frame).clamp(0.0, 1.0);
//
//                let v = VPSOscillator::limit_v(d, v + vs);
//                let s = osc.next(freq, israte * (OVERSAMPLING as f32), d, v);
//                let s = apply_distortion(s, damt, dist);
//
//                out.write(frame, s);
//            }
//        }

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
//            let v     = NodeId::RndWlk(0).inp_param("v").unwrap().inp();
//            let vs    = NodeId::RndWlk(0).inp_param("vs").unwrap().inp();
//            let d     = NodeId::RndWlk(0).inp_param("d").unwrap().inp();
//            let damt  = NodeId::RndWlk(0).inp_param("damt").unwrap().inp();
//            let dist  = NodeId::RndWlk(0).inp_param("dist").unwrap().inp();
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
