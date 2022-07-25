// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

//use super::helpers::{sqrt4_to_pow4, TrigSignal, Trigger};
use crate::dsp::{
    DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::UnsyncFloatBuf;

/// A simple signal scope
#[derive(Debug, Clone)]
pub struct Scope {
    buf: UnsyncFloatBuf,
    idx: usize,
}

impl Scope {
    pub fn new(_nid: &NodeId) -> Self {
        Self { buf: UnsyncFloatBuf::new_with_len(1), idx: 0 }
    }
    pub const inp: &'static str = "Scope inp\nSignal input.\nRange: (-1..1)\n";
    pub const sig: &'static str =
        "Scope sig\nSignal output. The exact same signal that was received on 'inp'!\n";
    pub const DESC: &'static str = r#"Signal Oscilloscope Probe

This is a signal oscilloscope probe node. You can have up to 16 of these,
which will be displayed in the GUI.
"#;
    pub const HELP: &'static str = r#"Scope - Signal Oscilloscope Probe

You can have up to 16 of these probes in your patch. The received signal will be
forwarded to the GUI and you can inspect the waveform there.
"#;

    pub fn set_scope_buffer(&mut self, buf: UnsyncFloatBuf) {
        self.buf = buf;
    }
}

impl DspNode for Scope {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, _srate: f32) {}

    fn reset(&mut self) {}

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{inp, out};

        let inp = inp::Ad::inp(inputs);
        let out = out::Scope::sig(outputs);

        for frame in 0..ctx.nframes() {
            let in_val = inp.read(frame);
            self.buf.write(self.idx, in_val);
            self.idx = (self.idx + 1) % self.buf.len();
            out.write(frame, in_val);
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(out.read(last_frame));
    }
}
