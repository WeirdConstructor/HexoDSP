// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{sqrt4_to_pow4, TrigSignal, Trigger};

#[macro_export]
macro_rules! fa_ad_mult {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "x1",
            1 => "x10",
            2 => "x100",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Ad {
    inc: f64,
    stage: u8,
    samples_ms: f64,
    value: f64,
    last_time: f32,
    trig: Trigger,
    trig_sig: TrigSignal,
}

impl Ad {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            inc: 0.0,
            stage: 0,
            samples_ms: 44.1,
            value: 0.0,
            last_time: -1.0,
            trig: Trigger::new(),
            trig_sig: TrigSignal::new(),
        }
    }
    pub const inp: &'static str =
        "Signal input. If you don't connect this, and set this to **1.0** \
        this will act as envelope signal generator. But you can also just \
        route a signal directly through this of course.";
    pub const trig: &'static str = "Trigger input that starts the attack phase.";
    pub const atk: &'static str = "Attack time of the envelope. You can extend the maximum \
        range of this with the ~~mult~~ setting.";
    pub const dcy: &'static str = "Decay time of the envelope. \
        You can extend the maximum range of this with the ~~mult~~ setting.\
        ";
    pub const ashp: &'static str = "Attack shape. This allows you to change the shape \
        of the attack stage from a logarithmic, to a linear and to an \
        exponential shape.";
    pub const dshp: &'static str = "Decay shape. This allows you to change the shape \
        of the decay stage from a logarithmic, to a linear and to an \
        exponential shape.";
    pub const mult: &'static str = "Attack and Decay time range multiplier. \
        This will extend the maximum range of the ~~atk~~ and ~~dcy~~ parameters.";
    pub const sig: &'static str = "Envelope signal output. If a signal is sent to the 'inp' port, \
        you will receive an attenuated signal here. If you set 'inp' to a \
        fixed value (**for instance 1.0**), this will output an envelope signal \
        in the range 0.0 to 'inp' (**1.0**).";
    pub const eoet: &'static str = "End of envelope trigger. This output sends a trigger once \
        the end of the decay stage has been reached.";
    pub const DESC: &'static str = r#"Attack-Decay Envelope

This is a simple envelope offering an attack time and decay time with a shape parameter.
You can use it as envelope generator to modulate other inputs or process a signal with it directly.
"#;
    pub const HELP: &'static str = r#"Attack-Decay Envelope

This simple two stage envelope with attack and decay offers shape parameters
for each stage. The attack and decay times can be extended using the ~~mult~~
setting.

The ~~inp~~ can either be used to process a signal, or set the target output
value of the envelope. In the latter case this node is just a simple
envelope generator, with which you can generate control signals to modulate
other inputs.

With the ~~eoet~~ output you can either trigger other envelopes or via
`FbWr`/`FbRd` retrigger the envelope.
"#;
}

impl DspNode for Ad {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.samples_ms = srate as f64 / 1000.0;
        self.trig_sig.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.stage = 0;
        self.value = 0.0;
        self.inc = 0.0;
        self.last_time = -1.0;
        self.trig_sig.reset();
        self.trig.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp, out};

        let inp = inp::Ad::inp(inputs);
        let trig = inp::Ad::trig(inputs);
        let atk = inp::Ad::atk(inputs);
        let dcy = inp::Ad::dcy(inputs);
        let atk_shape = inp::Ad::ashp(inputs);
        let dcy_shape = inp::Ad::dshp(inputs);
        let mult = at::Ad::mult(atoms);

        // block start:
        let (mut shape_src, mut inc_time_src, mut target, mut delta) = match self.stage {
            1 => (atk_shape, atk, 1.0, 1.0),
            2 => (dcy_shape, dcy, 0.0, -1.0),
            _ => (atk_shape, atk, 0.0, 0.0),
        };
        let mult: f64 = match mult.i() {
            1 => 10.0,
            2 => 100.0,
            _ => 1.0,
        };

        //        let mut cnt : usize = 0;

        for frame in 0..ctx.nframes() {
            if self.trig.check_trigger(denorm::Ad::trig(trig, frame)) {
                //d// println!("RETRIGGER!");
                self.stage = 1;
                self.last_time = -1.0;
                target = 1.0;
                delta = 1.0;
                shape_src = atk_shape;
                inc_time_src = atk;
            }

            let cur_time = denorm::Ad::atk(inc_time_src, frame);
            if self.last_time != cur_time {
                self.inc = if cur_time <= 0.0001 {
                    delta
                } else {
                    delta / ((cur_time as f64) * mult * self.samples_ms)
                };
                self.last_time = cur_time;

                //                if cnt % 32 == 0 {
                //                    println!("** v={:8.3}, inc={:8.3}, tar={:8.3}, time={:8.3}",
                //                             self.value, self.inc, target, self.last_time);
                //                }
            }

            self.value += self.inc;
            let shape = denorm::Ad::ashp(shape_src, frame).clamp(0.0, 1.0);

            //            if cnt % 32 == 0 {
            //                println!("v={:8.3}, inc={:8.3}, tar={:8.3}, time={:8.3}",
            //                         self.value, self.inc, target, self.last_time);
            //            }
            //            cnt += 1;

            match self.stage {
                1 => {
                    if self.value >= (target - 0.0001) {
                        self.stage = 2;
                        self.last_time = -1.0;
                        self.value = target;
                        target = 0.0;
                        delta = -1.0;
                        shape_src = dcy_shape;
                        inc_time_src = dcy;
                    }
                }
                2 => {
                    if self.value <= (target + 0.0001) {
                        self.stage = 0;
                        self.last_time = -1.0;
                        self.value = target;
                        target = 0.0;
                        delta = 0.0;
                        self.trig_sig.trigger();
                    }
                }
                _ => {}
            }

            let in_val = denorm::Ad::inp(inp, frame);
            let out = out::Ad::sig(outputs);
            //d// println!("VAL in={}, val={} shp: {}=>{}", in_val, self.value, shape,
            //d//     sqrt4_to_pow4(1.0, shape));
            out.write(frame, in_val * sqrt4_to_pow4(self.value.clamp(0.0, 1.0) as f32, shape));

            let eoet = out::Ad::eoet(outputs);
            eoet.write(frame, self.trig_sig.next());
        }

        ctx_vals[0].set(self.value as f32);
        //        ctx_vals[1].set(self.phase / self. + self.stage * );
    }

    fn graph_fun() -> Option<GraphFun> {
        Some(Box::new(|gd: &dyn GraphAtomData, _init: bool, x: f32, xn: f32| -> f32 {
            let atk_idx = NodeId::Ad(0).inp_param("atk").unwrap().inp();
            let dcy_idx = NodeId::Ad(0).inp_param("dcy").unwrap().inp();
            let ashp_idx = NodeId::Ad(0).inp_param("ashp").unwrap().inp();
            let dshp_idx = NodeId::Ad(0).inp_param("dshp").unwrap().inp();

            let atk = gd.get_norm(atk_idx as u32);
            let dcy = gd.get_norm(dcy_idx as u32);
            let ashp = gd.get_denorm(ashp_idx as u32);
            let dshp = gd.get_denorm(dshp_idx as u32);

            let a = atk * 0.5;
            let d = dcy * 0.5;
            if x <= a {
                if xn > a {
                    1.0
                } else if a < 0.0001 {
                    0.0
                } else {
                    let delta = 1.0 - ((a - x) / a);
                    sqrt4_to_pow4(delta, ashp)
                }
            } else if (x - a) <= d {
                if d < 0.0001 {
                    0.0
                } else {
                    let x = x - a;
                    let delta = (d - x) / d;
                    sqrt4_to_pow4(delta, dshp)
                }
            } else {
                0.0
            }
        }))
    }
}
