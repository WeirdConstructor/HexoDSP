// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{sqrt4_to_pow4, EnvADSRParams, EnvRetrigADSR};

#[macro_export]
macro_rules! fa_adsr_mult {
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
pub struct Adsr {
    env: EnvRetrigADSR,
}

impl Adsr {
    pub fn new(_nid: &NodeId) -> Self {
        Self { env: EnvRetrigADSR::new() }
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
    pub const rel: &'static str = "Release time of the envelope. \
        You can extend the maximum range of this with the ~~mult~~ setting.\
        ";
    pub const sus: &'static str = "Sustain value. \
        "; // TODO
    pub const ashp: &'static str = "Attack shape. This allows you to change the shape \
        of the attack stage from a logarithmic, to a linear and to an \
        exponential shape.";
    pub const dshp: &'static str = "Decay shape. This allows you to change the shape \
        of the decay stage from a logarithmic, to a linear and to an \
        exponential shape.";
    pub const rshp: &'static str = "Release shape. This allows you to change the shape \
        of the release stage from a logarithmic, to a linear and to an \
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

impl DspNode for Adsr {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.env.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.env.reset();
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

        let inp = inp::Adsr::inp(inputs);
        let trig = inp::Adsr::trig(inputs);
        let atk = inp::Adsr::atk(inputs);
        let dcy = inp::Adsr::dcy(inputs);
        let sus = inp::Adsr::sus(inputs);
        let rel = inp::Adsr::rel(inputs);
        let atk_shape = inp::Adsr::ashp(inputs);
        let dcy_shape = inp::Adsr::dshp(inputs);
        let rel_shape = inp::Adsr::rshp(inputs);
        let mult = at::Adsr::mult(atoms);

        let mult: f32 = match mult.i() {
            1 => 10.0,
            2 => 100.0,
            _ => 1.0,
        };

        let mut params = EnvADSRParams::default();

        for frame in 0..ctx.nframes() {
            let trigger_sig = denorm::Adsr::trig(trig, frame);
            params.attack_ms = mult * denorm::Adsr::atk(atk, frame);
            params.attack_shape = denorm::Adsr::ashp(atk_shape, frame).clamp(0.0, 1.0);
            params.decay_ms = mult * denorm::Adsr::dcy(dcy, frame);
            params.decay_shape = 1.0 - denorm::Adsr::dshp(dcy_shape, frame).clamp(0.0, 1.0);
            params.release_ms = mult * denorm::Adsr::rel(rel, frame);
            params.release_shape = 1.0 - denorm::Adsr::rshp(rel_shape, frame).clamp(0.0, 1.0);
            params.sustain = denorm::Adsr::sus(sus, frame).clamp(0.0, 1.0);

            let (value, retrig_sig) = self.env.tick(trigger_sig, &mut params);

            let in_val = denorm::Adsr::inp(inp, frame);
            let out = out::Adsr::sig(outputs);
            out.write(frame, in_val * value);

            let eoet = out::Adsr::eoet(outputs);
            eoet.write(frame, retrig_sig);
        }

        let last_frame = ctx.nframes() - 1;
        let out = out::Adsr::sig(outputs);
        ctx_vals[0].set(out.read(last_frame));
    }

    fn graph_fun() -> Option<GraphFun> {
        Some(Box::new(|gd: &dyn GraphAtomData, _init: bool, x: f32, xn: f32| -> f32 {
            let atk_idx = NodeId::Adsr(0).inp_param("atk").unwrap().inp();
            let dcy_idx = NodeId::Adsr(0).inp_param("dcy").unwrap().inp();
            let ashp_idx = NodeId::Adsr(0).inp_param("ashp").unwrap().inp();
            let dshp_idx = NodeId::Adsr(0).inp_param("dshp").unwrap().inp();

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
