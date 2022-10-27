// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{Rng, SlewValue, Trigger};

/// A triggered random walker
#[derive(Debug, Clone)]
pub struct RndWk {
    rng: Rng,
    slew_val: SlewValue<f64>,
    trig: Trigger,
    target: f64,
}

impl RndWk {
    pub fn new(nid: &NodeId, _node_global: &NodeGlobalRef) -> Self {
        let mut rng = Rng::new();
        rng.seed((0x193a67f4a8a6d769_u64).wrapping_add(0x262829 * (nid.instance() as u64 + 1)));

        Self { rng, trig: Trigger::new(), slew_val: SlewValue::new(), target: 0.0 }
    }

    pub const trig: &'static str = "This trigger generates a new random number within \
        the current ~~min~~/~~max~~ range.";
    pub const step: &'static str = "This is the maximum possible step size of the \
        random number drawn upon ~~trig~~. Setting this to **0.0** will disable \
        the randomness.\nThe minimum step size can be defined \
        by the ~~offs~~ parameter.";
    pub const offs: &'static str =
        "The minimum step size and direction that is done on each ~~trig~~.\
        Depending on the size of the ~~offs~~ and the ~~min~~/~~max~~ range, \
        this might result in the output value being close to the limits \
        of that range.";
    pub const min: &'static str = "The minimum of the new target value. If a value is drawn \
        that is outside of this range, it will be reflected back into it.\
        ";
    pub const max: &'static str = "The maximum of the new target value. If a value is drawn \
        that is outside of this range, it will be reflected back into it.\
        ";
    pub const slew: &'static str = "The slew rate limiting time. Thats the time it takes to \
        get to **1.0** from **0.0**. Useful for smoothing modulation of audio signals. \
        The higher the time, the smoother/slower the transition to new \
        target values will be.";
    pub const sig: &'static str = "Oscillator output";
    pub const DESC: &'static str = r#"Random Walker

This modulator generates a random number by walking a pre defined maximum random ~~step~~ width.
For smoother transitions a slew rate limiter is integrated.
"#;
    pub const HELP: &'static str = r#"Random Walker

This modulator generates a random number by walking a pre defined
maximum random ~~step~~ width. The newly generated target value will always
be folded within the defined ~~min~~/~~max~~ range. The ~~offs~~ parameter defines a
minimal step width each ~~trig~~ has to change the target value.

For smoother transitions, if you want to modulate an audio signal with this,
a slew rate limiter (~~slew~~) is integrated.

You can disable all randomness by setting ~~step~~ to **0.0**.

Tip: Interesting and smooth results can be achieved if you set ~~slew~~
to a (way) longer time than the ~~trig~~ interval. It will smooth
off the step widths and the overall motion even more.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for RndWk {
    fn set_sample_rate(&mut self, srate: f32) {
        self.slew_val.set_sample_rate(srate as f64);
    }

    fn reset(&mut self) {
        self.slew_val.reset();
        self.trig.reset();
        self.target = 0.0;
    }

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{denorm, inp, out};

        let trig = inp::RndWk::trig(inputs);
        let step = inp::RndWk::step(inputs);
        let offs = inp::RndWk::offs(inputs);
        let min = inp::RndWk::min(inputs);
        let max = inp::RndWk::max(inputs);
        let slew = inp::RndWk::slew(inputs);
        let out = out::RndWk::sig(outputs);

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

                let mut target =
                    self.slew_val.value() as f32 + ((self.rng.next() * 2.0 * step) - step) + offs;

                // println!("{:8.6} {:8.6} {:8.6}", min, max, target);
                // clamp target into a range we can reflect
                target = target.clamp(min - (delta * 0.99), max + (delta * 0.99));
                // reflect back the overshoots:
                if target > max {
                    target = max - (max - target).abs();
                }
                if target < min {
                    target = min + (min - target).abs();
                }

                self.target = target as f64;
            }

            let slew_time_ms = denorm::RndWk::slew(slew, frame);

            out.write(frame, self.slew_val.next(self.target, slew_time_ms as f64) as f32);
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }

    //    pub fn graph_fun() -> Option<GraphFun> {
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
