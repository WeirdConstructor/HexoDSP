// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::{SharedFeedback, SharedFeedbackReader, SharedFeedbackWriter};

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FbWr {
    fb_wr: Box<SharedFeedbackWriter>,
}

impl FbWr {
    pub fn new(nid: &NodeId, node_global: &NodeGlobalRef) -> Self {
        let fb_wr = if let Ok(mut node_global) = node_global.lock() {
            node_global.get_feedback_writer(nid.instance() as usize)
        } else {
            // If we can't get the lock, other issues are active and I would
            // rather not crash, so I just make a dummy feedback buffer:
            let sfb = SharedFeedback::new(44100.0);
            Box::new(SharedFeedbackWriter::new(&sfb))
        };
        Self { fb_wr }
    }
    pub const inp: &'static str = "Signal input";

    pub const DESC: &'static str = "Feedback Delay Writer\n\n\
HexoSynth does not allow direct feedback cycles in it's graph.\n\
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.\n\
This node allows you to write a signal into the corresponsing signal delay buffer.\n\
Use `FbRd` for using the signal. The delay is **3.14ms**.";
    pub const HELP: &'static str = r#"Feedback Delay Writer

HexoSynth does not allow direct feedback cycles in it's graph.
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.
This node allows you to send a signal into the corresponding `FbWr` signal
delay.

The instance id of the node defines which `FbWr` and `FbRd` are connected.
That means `FbRd 0` is connected to the corresponding `FbWr 0`. You can use
the signal multiple times by connecting the `FbRd 0` ~~sig~~ port to multiple
inputs.

The delay is always **3.14ms**, regardless of the sampling rate the synthesizer
is running at.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for FbWr {
    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        _outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::inp;

        let inp = inp::FbWr::inp(inputs);

        for frame in 0..ctx.nframes() {
            self.fb_wr.write(inp.read(frame));
        }

        ctx_vals[0].set(inp.read(ctx.nframes() - 1));
    }
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FbRd {
    fb_rd: Box<SharedFeedbackReader>,
}

impl FbRd {
    pub fn new(nid: &NodeId, node_global: &NodeGlobalRef) -> Self {
        let fb_rd = if let Ok(mut node_global) = node_global.lock() {
            node_global.get_feedback_reader(nid.instance() as usize)
        } else {
            // If we can't get the lock, other issues are active and I would
            // rather not crash, so I just make a dummy feedback buffer:
            let sfb = SharedFeedback::new(44100.0);
            Box::new(SharedFeedbackReader::new(&sfb))
        };
        Self { fb_rd }
    }
    pub const vol: &'static str = "Volume of the input.\n\
         Use this to adjust the feedback amount.";
    pub const sig: &'static str = "Feedback signal output.";

    pub const DESC: &'static str = "Feedback Delay Reader\n\n\
HexoSynth does not allow direct feedback cycles in it's graph.\n\
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.\n\
This node allows you to tap into the corresponding `FbWr` signal delay \
for feedback. The delay is **3.14ms**.";
    pub const HELP: &'static str = r#"Feedback Delay Reader

HexoSynth does not allow direct feedback cycles in it's graph.
To make feedback possible anyways the `FbWr` and `FbRd` nodes are provided.
This node allows you to tap into the corresponding `FbWr` signal delay for
feedback.

The instance id of the node defines which `FbWr` and `FbRd` are connected.
That means `FbRd 0` is connected to the corresponding `FbWr 0`. You can use
the signal multiple times by connecting the `FbRd 0` ~~sig~~ port to multiple
inputs.

The delay is always **3.14ms**, regardless of the sampling rate the synthesizer
is running at.

The ~~vol~~ parameter is a convenience parameter to allow to control the
volume of the feedback.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for FbRd {
    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}

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

        let vol = inp::FbRd::vol(inputs);
        let sig = out::FbRd::sig(outputs);

        let mut last_val = 0.0;
        for frame in 0..ctx.nframes() {
            last_val = self.fb_rd.read();
            last_val *= denorm::FbRd::vol(vol, frame);
            sig.write(frame, last_val);
        }

        ctx_vals[0].set(last_val);
    }
}
