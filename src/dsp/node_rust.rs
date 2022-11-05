// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use std::sync::Arc;
use synfx_dsp::AtomicFloat;
use triple_buffer::{TripleBuffer, Input, Output};


/// A context structure for supporting the [DynamicNode1x1::process] function.
///
/// It provides access to the input slices of the `alpha`, `beta`, `gamma` and `delta`
/// values. And to the LED and phase values, which are basically two [AtomicFloat]
/// values that you can read out in the frontend thread using the [crate::SynthConstructor]
/// or [crate::NodeConfigurator] API.
pub struct DynNode1x1Context {
    nframes: usize,
    alpha: ProcBuf,
    beta: ProcBuf,
    gamma: ProcBuf,
    delta: ProcBuf,
    led_value: Arc<AtomicFloat>,
    phase_value: Arc<AtomicFloat>,
}

impl DynNode1x1Context {
    pub fn alpha_slice(&self) -> &[f32] {
        self.alpha.slice(self.nframes)
    }

    pub fn beta_slice(&self) -> &[f32] {
        self.beta.slice(self.nframes)
    }

    pub fn gamma_slice(&self) -> &[f32] {
        self.gamma.slice(self.nframes)
    }

    pub fn delta_slice(&self) -> &[f32] {
        self.delta.slice(self.nframes)
    }

    pub fn led_value(&self) -> &Arc<AtomicFloat> {
        &self.led_value
    }
    pub fn phase_value(&self) -> &Arc<AtomicFloat> {
        &self.phase_value
    }
}

/// This trait allows you to write custom HexoDSP nodes in Rust and provide them
/// at runtime using [crate::NodeConfigurator::set_dynamic_node1x1] or [crate::SynthConstructor::set_dynamic_node1x1].
///
/// The 1x1 means there is one dedicated input signal and one dedicated output signal.
/// The input signal is accompanied with 4 auxiliary signals using the [DynNode1x1Context]
/// structure.
///
/// There is a trait implementation for `FnMut(&[f32], &mut [f32], &DynNode1x1Context)` functions,
/// which means you don't have to implement a full structure yourself and can just pass in
/// closures to [crate::SynthConstructor::set_dynamic_node1x1]:
///
///```
/// use hexodsp::{SynthConstructor, DynamicNode1x1, DynNode1x1Context};
/// use hexodsp::build::*;
///
/// let mut sc = SynthConstructor::new();
///
/// sc.set_dynamic_node1x1(0, Box::new(|inp: &[f32], out: &mut [f32], ctx: &DynNode1x1Context| {
///     // Your code here!
/// }));
///
/// sc.upload(&out(0).input().ch1(&rust1x1(0).output().sig()));
///```
/// See also: [crate::SynthConstructor::set_dynamic_node1x1] for a more detailed example.
pub trait DynamicNode1x1: Send {
    /// The sample rate function sets the sample rate the DSP graph is currently running at.
    fn set_sample_rate(&mut self, _sample_rate: f32) {}
    /// This is called whenever the DSP in the audio thread is resetted.
    fn reset(&mut self) {}
    /// You implement this method with your own custom DSP code.
    fn process(&mut self, _input: &[f32], _output: &mut [f32], _ctx: &DynNode1x1Context);
}

impl<T> crate::dsp::DynamicNode1x1 for T
where
    T: FnMut(&[f32], &mut [f32], &DynNode1x1Context) + Send,
{
    fn process(&mut self, input: &[f32], output: &mut [f32], ctx: &DynNode1x1Context) {
        (self)(input, output, ctx)
    }
}

struct RustDummyNode {}

pub fn new_dummy_dynamic_node1x1() -> Box<dyn DynamicNode1x1> {
    Box::new(RustDummyNode {})
}

impl DynamicNode1x1 for RustDummyNode {
    fn process(&mut self, _input: &[f32], output: &mut [f32], _ctx: &DynNode1x1Context) {
        for o in output.iter_mut() {
            *o = 0.0;
        }
    }
}

pub struct DynNodeBuffer<T: Send> {
    output: Output<T>,
}

impl<T> DynNodeBuffer<T> where T: Send {
    #[inline]
    pub fn update(&mut self) {
        self.output.update();
    }

    #[inline]
    pub fn access(&mut self) -> &mut T {
        self.output.output_buffer()
    }
}

struct DynNode1x1Handle {
    input: Input<Box<dyn DynamicNode1x1>>,
    output: Option<Output<Box<dyn DynamicNode1x1>>>,
}

impl Default for Box<dyn DynamicNode1x1> {
    fn default() -> Self {
        new_dummy_dynamic_node1x1()
    }
}

impl DynNode1x1Handle {
    pub fn new() -> Self {
        let (input, output) = TripleBuffer::default().split();
        Self {
            input,
            output: Some(output),
        }
    }

    pub fn write(&mut self, node: Box<dyn DynamicNode1x1>) {
        self.input.write(node);
    }

    pub fn get_output_buffer(&mut self) -> DynNodeBuffer<Box<dyn DynamicNode1x1>> {
        let output =
            if let Some(output) = self.output.take() {
                output
            } else {
                let (input, output) = TripleBuffer::default().split();
                self.input = input;
                output
            };
        DynNodeBuffer { output }
    }
}

/// A native Rust code node that uses trait objects for dispatch
#[derive(Debug, Clone)]
pub struct Rust1x1 {
    index: usize,
}

impl Rust1x1 {
    pub fn new(nid: &NodeId, _node_global: &NodeGlobalRef) -> Self {
        Self { index: nid.instance() as usize }
    }

    pub const inp: &'static str =
        "Signal input. Signal input to the dynamically dispatched Rust node.";
    pub const alpha: &'static str = "Alpha parameter for the dynamically dispatched Rust node.";
    pub const beta: &'static str = "Beta parameter for the dynamically dispatched Rust node.";
    pub const gamma: &'static str = "Gamma parameter for the dynamically dispatched Rust node.";
    pub const delta: &'static str = "Delta parameter for the dynamically dispatched Rust node.";
    pub const sig: &'static str =
        "Signal output. Signal output of the dynamically dispatched Rust node.";
    pub const DESC: &'static str = r#"Rust Code Node

This node does provide the user of HexoDSP or the SynthConstructor with an API
to code custom DSP node implementations in pure Rust at compile time.
It does not have any relevance for HexoSynth.
See also [crate::SynthConstructor] and [crate::DynamicNode1x1].
"#;
    pub const HELP: &'static str = r#"Rust Code Node

This node does provide the user of HexoDSP or the SynthConstructor with an API
to code custom DSP node implementations in pure Rust at compile time.

Treat this node as plugin API into the HexoDSP DSP graph.

This node does nothing in HexoSynth.

See also [crate::SynthConstructor] and [crate::DynamicNode1x1].
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for Rust1x1 {
    fn set_sample_rate(&mut self, _srate: f32) {}

    fn reset(&mut self) {}

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{inp, inp_buf, out};

        let inp = inp::Rust1x1::inp(inputs);
        let out = out::Rust1x1::sig(outputs);

        let n1x1ctx = DynNode1x1Context {
            nframes: ctx.nframes(),
            alpha: inp_buf::Rust1x1::alpha(inputs),
            beta: inp_buf::Rust1x1::beta(inputs),
            gamma: inp_buf::Rust1x1::gamma(inputs),
            delta: inp_buf::Rust1x1::delta(inputs),
            led_value: ctx_vals[0].clone(),
            phase_value: ctx_vals[1].clone(),
        };

        ectx.dynamic_nodes1x1[self.index].process(
            inp.slice(ctx.nframes()),
            out.slice_mut(ctx.nframes()),
            &n1x1ctx,
        );
    }
}
