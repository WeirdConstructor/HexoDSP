// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

//use super::helpers::{sqrt4_to_pow4, TrigSignal, Trigger};
use crate::nodes::SCOPE_SAMPLES;
use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::UnsyncFloatBuf;

/// A simple signal scope
#[derive(Debug, Clone)]
pub struct Scope {
    buf: [UnsyncFloatBuf; 3],
    idx: usize,
}

impl Scope {
    pub fn new(_nid: &NodeId) -> Self {
        let buf = [
            UnsyncFloatBuf::new_with_len(1),
            UnsyncFloatBuf::new_with_len(1),
            UnsyncFloatBuf::new_with_len(1),
        ];
        Self { buf, idx: 0 }
    }
    pub const in1: &'static str = "Scope in1\nSignal input 1.\nRange: (-1..1)\n";
    pub const in2: &'static str = "Scope in2\nSignal input 2.\nRange: (-1..1)\n";
    pub const in3: &'static str = "Scope in3\nSignal input 3.\nRange: (-1..1)\n";
    pub const DESC: &'static str = r#"Signal Oscilloscope Probe

This is a signal oscilloscope probe node, you can capture up to 3 signals.
"#;
    pub const HELP: &'static str = r#"Scope - Signal Oscilloscope Probe

You can have up to 8 different scopes in your patch. That means you can in theory
record up to 24 signals. The received signal will be forwarded to the GUI and
you can inspect the waveform there.
"#;

    pub fn set_scope_buffers(&mut self, buf: [UnsyncFloatBuf; 3]) {
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

        let in1 = inp::Scope::in1(inputs);
        let in2 = inp::Scope::in2(inputs);
        let in3 = inp::Scope::in3(inputs);
        let inputs = [in1, in2, in3];

        for frame in 0..ctx.nframes() {
            for (i, input) in inputs.iter().enumerate() {
                let in_val = input.read(frame);
                self.buf[i].write(self.idx, in_val);
            }

            self.idx = (self.idx + 1) % SCOPE_SAMPLES;
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(
            (in1.read(last_frame) + in2.read(last_frame) + in3.read(last_frame)).clamp(-1.0, 1.0),
        );
    }
}
