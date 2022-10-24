// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{EnvRetrigAD, sqrt4_to_pow4};

pub trait DynamicNode: Send {
    fn set_sample_rate(&mut self, sample_rate: f32) { }
    fn reset(&mut self) { }
    fn process(&mut self, input: &[f32], output: &mut [f32]);
}

struct RustDummyNode {
}

impl DynamicNode for RustDummyNode {
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        for o in output.iter_mut() {
            *o = 0.0;
        }
    }
}

/// A native Rust code node that uses trait objects for dispatch
pub struct Rust1x1 {
    node: Box<dyn DynamicNode>,
}

impl Clone for Rust1x1 {
    fn clone(&self) -> Self {
        Self {
            node: Box::new(RustDummyNode { }),
        }
    }
}

impl std::fmt::Debug for Rust1x1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rust1x1()")
    }
}

impl Rust1x1 {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            node: Box::new(RustDummyNode {}),
        }
    }

    fn swap_node(&mut self, node: &mut Box<dyn DynamicNode>) {
        std::mem::swap(&mut self.node, node);
    }

    pub const inp: &'static str =
        "Signal input. If you don't connect this, and set this to **1.0** \
        this will act as envelope signal generator. But you can also just \
        route a signal directly through this of course.";
    pub const alpha: &'static str = "";
    pub const beta: &'static str = "";
    pub const gamma: &'static str = "";
    pub const delta: &'static str = "";
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
    pub const DESC: &'static str = r#"Rust Code Node

"#;
    pub const HELP: &'static str = r#"Rust Code Node

"#;
}

impl DspNode for Rust1x1 {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.node.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.node.reset();
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

        let inp = inp::Rust1x1::inp(inputs);
        let out = out::Rust1x1::sig(outputs);

        self.node.process(inp.slice(ctx.nframes()), out.slice(ctx.nframes()));
//        for frame in 0..ctx.nframes() {
//            let trigger_sig = denorm::Ad::trig(trig, frame);
//            let atk_ms = mult * denorm::Ad::atk(atk, frame);
//            let ashp = denorm::Ad::ashp(atk_shape, frame).clamp(0.0, 1.0);
//            let dcy_ms = mult * denorm::Ad::dcy(dcy, frame);
//            let dshp = 1.0 - denorm::Ad::dshp(dcy_shape, frame).clamp(0.0, 1.0);
//
//            let (value, retrig_sig) = self.env.tick(trigger_sig, atk_ms, ashp, dcy_ms, dshp);
//
//            let in_val = denorm::Ad::inp(inp, frame);
//            let out = out::Ad::sig(outputs);
//            out.write(frame, in_val * value);
//
//            let eoet = out::Ad::eoet(outputs);
//            eoet.write(frame, retrig_sig);
//        }
//
//        let last_frame = ctx.nframes() - 1;
//        ctx_vals[0].set(out.read(last_frame));
    }
}
