// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::helpers::{Rng, Trigger, SlewValue};
use crate::dsp::{
    NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext,
    GraphAtomData, GraphFun,
};

/// A triggered random walker
#[derive(Debug, Clone)]
pub struct RndWk {
    rng:        Rng,
    slew_val:   SlewValue<f32>,
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
            trig:     Trigger::new(),
            slew_val: SlewValue::new(),
        }
    }

    pub const trig : &'static str =
        "RndWk trig\nThis trigger generates a new random number within \
        the current 'min'/'max' range.\nRange: (-1..1)";
    pub const step : &'static str =
        "RndWk step\nThis is the maximum possible step size of the \
        random number drawn upon 'trig'. Setting this to 0.0 will disable \
        the randomness.\nThe minimum step size can be defined \
        by the 'offs' parameter.\nRange: (0..1)";
    pub const offs : &'static str =
        "RndWk offs\nThe minimum step size and direction that is done on each 'trig'.\
        Depending on the size of the 'offs' and the 'min'/'max' range, \
        this might result in the output value being close to the limits \
        of that range.\nRange: (-1..1)";
    pub const min : &'static str =
        "RndWk min\nThe minimum of the new target value. If a value is drawn \
        that is outside of this range, it will be reflected back into it.\
        \nRange: (0..1)";
    pub const max : &'static str =
        "RndWk max\nThe maximum of the new target value. If a value is drawn \
        that is outside of this range, it will be reflected back into it.\
        \nRange: (0..1)";
    pub const slewt : &'static str =
        "RndWk slewt\nThe slew time, the time it takes to reach the \
        new target value. This can be used to smooth off rough transitions and \
        clicky noises.\nRange: (0..1)";
    pub const sig : &'static str =
        "RndWk sig\nOscillator output\nRange: (-1..1)\n";
    pub const DESC : &'static str =
r#"Random Walker

This modulator generates a random number by walking a pre defined maximum random 'step' width. For smoother transitions a slew time is integrated.
"#;
    pub const HELP : &'static str =
r#"RndWk - Random Walker

This modulator generates a random number by walking a pre defined
maximum random 'step' width. The newly generated target value will always
be folded within the defined 'min'/'max' range. The 'offs' parameter defines a
minimal step width each 'trig' has to change the target value.

For smoother transitions, if you want to modulate an audio signal with this,
a slew time ('slewt') is integrated.

You can disable all randomness by setting 'step' to 0.0.

Tip: Interesting and smooth results can be achieved if you set 'slewt'
to a longer time than the interval in that you trigger 'trig'. It will smooth
off the step widths and the overall motion even more.
"#;

}

impl DspNode for RndWk {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.slew_val.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.slew_val.reset();
        self.trig.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom], inputs: &[ProcBuf],
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

                let target =
                    self.slew_val.value()
                    + ((self.rng.next() * 2.0 * step) - step)
                    + offs;
                let target = ((target - min) % delta).abs() + min;

                let slew_time_ms = denorm::RndWk::slewt(slewt, frame);

                self.slew_val.set_target(target, slew_time_ms);
            }

            out.write(frame, self.slew_val.next());
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
