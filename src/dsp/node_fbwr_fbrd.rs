// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FbWr {
    fb_index: u8,
}

impl FbWr {
    pub fn new(nid: &NodeId) -> Self {
        Self { fb_index: nid.instance() as u8 }
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
}

impl DspNode for FbWr {
    fn outputs() -> usize {
        0
    }

    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        _outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::inp;

        let inp = inp::FbWr::inp(inputs);

        for frame in 0..ctx.nframes() {
            ectx.feedback_delay_buffers[self.fb_index as usize].write(inp.read(frame));
        }

        ctx_vals[0].set(inp.read(ctx.nframes() - 1));
    }
}

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FbRd {
    fb_index: u8,
}

impl FbRd {
    pub fn new(nid: &NodeId) -> Self {
        Self { fb_index: nid.instance() as u8 }
    }
    pub const atv: &'static str = "Attenuate or invert input.\n\
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

The ~~atv~~ parameter is a convenience parameter to allow attenuating or
even inverting the signal.
"#;
}

impl DspNode for FbRd {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{denorm, inp, out};

        let atv = inp::FbRd::atv(inputs);
        let sig = out::FbRd::sig(outputs);

        let mut last_val = 0.0;
        for frame in 0..ctx.nframes() {
            last_val = ectx.feedback_delay_buffers[self.fb_index as usize].read();
            last_val *= denorm::FbRd::atv(atv, frame);
            sig.write(frame, last_val);
        }

        ctx_vals[0].set(last_val);
    }
}
