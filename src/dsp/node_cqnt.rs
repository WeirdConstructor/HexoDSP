// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{ChangeTrig, CtrlPitchQuantizer};

#[macro_export]
macro_rules! fa_cqnt {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        write!($formatter, "?")
    }};
}

#[macro_export]
macro_rules! fa_cqnt_omin {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "-0",
            1 => "-1",
            2 => "-2",
            3 => "-3",
            4 => "-4",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

#[macro_export]
macro_rules! fa_cqnt_omax {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "+0",
            1 => "+1",
            2 => "+2",
            3 => "+3",
            4 => "+4",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// A control signal to pitch quantizer/converter
#[derive(Debug, Clone)]
pub struct CQnt {
    quant: Box<CtrlPitchQuantizer>,
    change_trig: ChangeTrig,
}

impl CQnt {
    pub fn new(_nid: &NodeId) -> Self {
        Self { quant: Box::new(CtrlPitchQuantizer::new()), change_trig: ChangeTrig::new() }
    }
    pub const inp: &'static str =
        "The unipolar input signal that is to be mapped to the \
        selected pitch range.";
    pub const oct: &'static str = "The octave offset from A4.";
    pub const omin: &'static str =
        "The minimum octave of the range. If **0** it will be ~~oct~~.";
    pub const omax: &'static str =
        "The maximum octave of the range. If **0** it will be ~~oct~~.";
    pub const sig: &'static str = "The output pitch signal.";
    pub const t: &'static str = "Everytime the quantizer snaps to a new pitch, it will \
        emit a short trigger on this signal output. This is useful \
        to trigger for example an envelope.";
    pub const keys: &'static str =
        "Here you can select the individual notes of the range. \
        If no note is selected, it's the same as if all notes were selected.";
    pub const DESC: &'static str = r#"Ctrl Pitch Quantizer

This special quantizer maps the unipolar **0**..**1** control signal
input range on ~~inp~~ evenly to the selected keys and octaves.
"#;
    pub const HELP: &'static str = r#"A control signal to pitch quantizer

This is a specialized control signal quantizer to generate a pitch/frequency
from a signal within the **0**..**1** range. It does not quantize a typical **-1**..**1**
frequency signal like the `Quant` node.

In contrast to `Quant`, this quantizer maps the incoming signal evenly
to the available note range. It will result in more evenly played notes
if you sweep across the input signal range.
"#;
}

impl DspNode for CQnt {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.change_trig.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.change_trig.reset();
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
        use crate::dsp::{at, denorm, inp, out_buf};

        let inp = inp::CQnt::inp(inputs);
        let oct = inp::CQnt::oct(inputs);
        let mut out = out_buf::CQnt::sig(outputs);
        let mut t = out_buf::CQnt::t(outputs);
        let keys = at::CQnt::keys(atoms);
        let omin = at::CQnt::omin(atoms);
        let omax = at::CQnt::omax(atoms);

        self.quant.update_keys(keys.i(), omin.i(), omax.i());

        for frame in 0..ctx.nframes() {
            let pitch = self.quant.signal_to_pitch(denorm::CQnt::inp(inp, frame));

            t.write(frame, self.change_trig.next(pitch));
            out.write(frame, pitch + denorm::CQnt::oct(oct, frame));
        }

        let last_pitch = self.quant.last_key_pitch();
        ctx_vals[1].set(last_pitch * 10.0 + 0.0001);
        ctx_vals[0].set((last_pitch * 10.0 - 0.5) * 2.0);
    }
}
