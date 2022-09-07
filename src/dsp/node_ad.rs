// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{env_target_stage, sqrt4_to_pow4, EnvState, TrigSignal, Trigger};

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
    state: EnvState,
    trig: Trigger,
    trig_sig: TrigSignal,
}

impl Ad {
    pub fn new(_nid: &NodeId) -> Self {
        Self { state: EnvState::new(), trig: Trigger::new(), trig_sig: TrigSignal::new() }
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
        self.state.set_sample_rate(srate);
        self.trig_sig.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.state.reset();
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

        let mult: f32 = match mult.i() {
            1 => 10.0,
            2 => 100.0,
            _ => 1.0,
        };

        for frame in 0..ctx.nframes() {
            if self.trig.check_trigger(denorm::Ad::trig(trig, frame)) {
                self.state.trigger();
            }

            let atk_ms = mult * denorm::Ad::atk(atk, frame);
            let ashp = denorm::Ad::ashp(atk_shape, frame).clamp(0.0, 1.0);

            if self.state.is_running() {
                env_target_stage_lin_time_adj!(
                    self.state,
                    0,
                    atk_ms,
                    0.0,
                    1.0,
                    |x: f32| sqrt4_to_pow4(x.clamp(0.0, 1.0), ashp),
                    {
                        let dcy_ms = mult * denorm::Ad::dcy(dcy, frame);
                        let dshp = 1.0 - denorm::Ad::dshp(dcy_shape, frame).clamp(0.0, 1.0);

                        env_target_stage!(
                            self.state,
                            2,
                            dcy_ms,
                            0.0,
                            |x: f32| sqrt4_to_pow4(x.clamp(0.0, 1.0), dshp),
                            {
                                self.trig_sig.trigger();
                                self.state.stop_immediately();
                            }
                        );
                    }
                );
            }


            let in_val = denorm::Ad::inp(inp, frame);
            let out = out::Ad::sig(outputs);
            out.write(frame, in_val * self.state.current);

            let eoet = out::Ad::eoet(outputs);
            eoet.write(frame, self.trig_sig.next());
        }

        ctx_vals[0].set(self.state.current as f32);
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
