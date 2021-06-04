// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FbWr {
    fb_index:   u8,
}

impl FbWr {
    pub fn new(nid: &NodeId) -> Self {
        Self {
            fb_index: nid.instance() as u8,
        }
    }
    pub const inp : &'static str =
        "FbWr inp\nSignal input\nRange: (-1..1)\n";
}

impl DspNode for FbWr {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{inp};

        let inp  = inp::FbWr::inp(inputs);

        for frame in 0..ctx.nframes() {
            ectx.feedback_delay_buffers[self.fb_index as usize]
                .write(inp.read(frame));
        }

        ctx_vals[0].set(inp.read(ctx.nframes() - 1));
    }
}


/// A simple amplifier
#[derive(Debug, Clone)]
pub struct FbRd {
    fb_index:   u8,
}

impl FbRd {
    pub fn new(nid: &NodeId) -> Self {
        Self {
            fb_index: nid.instance() as u8,
        }
    }
    pub const atv : &'static str =
        "FbRd atv\nAttenuate or invert input.\n\
         Use this to adjust the feedback amount.\nRange: (0..1)\n";
    pub const sig : &'static str =
        "FbRd sig\nFeedback signal output.\nRange: (-1..1)\n";
}

impl DspNode for FbRd {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, denorm};

        let atv  = inp::FbRd::atv(inputs);
        let sig  = out::FbRd::sig(outputs);

        let mut last_val = 0.0;
        for frame in 0..ctx.nframes() {
            last_val =
                ectx.feedback_delay_buffers[self.fb_index as usize]
                    .read();
            last_val *= denorm::FbRd::atv(atv, frame);
            sig.write(frame, last_val);
        }

        ctx_vals[0].set(last_val);
    }
}
