// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers::CtrlPitchQuantizer;

#[macro_export]
macro_rules! fa_cqnt { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    write!($formatter, "?")
} } }

#[macro_export]
macro_rules! fa_cqnt_omin { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "-0",
            1  => "-1",
            2  => "-2",
            3  => "-3",
            4  => "-4",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

#[macro_export]
macro_rules! fa_cqnt_omax { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "+0",
            1  => "+1",
            2  => "+2",
            3  => "+3",
            4  => "+4",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

/// A control signal to pitch quantizer/converter
#[derive(Debug, Clone)]
pub struct CQnt {
    quant: Box<CtrlPitchQuantizer>,
}

impl CQnt {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            quant: Box::new(CtrlPitchQuantizer::new()),
        }
    }
    pub const inp : &'static str =
        "CQnt inp\n\nRange: (0..1)";
    pub const oct : &'static str =
        "CQnt oct\n\nRange: (-1..1)";
    pub const omin : &'static str =
        "CQnt omin\n\nRange: (-1..1)";
    pub const omax : &'static str =
        "CQnt omax\n\nRange: (-1..1)";
    pub const sig : &'static str =
        "CQnt sig\n\nRange: (-1..1)";
    pub const keys : &'static str =
        "CQnt keys\n";
    pub const DESC : &'static str =
r#"Control Pitch Quantizer

This special quantizer maps the 0..1 input range on 'inp' evenly to the selected keys and octaves.
"#;
    pub const HELP : &'static str =
r#"CQnt - A control signal to pitch quantizer

This is a specialized quantizer to generate a pitch/frequency from a signal
within the 0..1 range. It does not quantize a typical -1..1 frequency signal
like the 'Quant' node.
"#;
}

impl DspNode for CQnt {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, _srate: f32) { }
    fn reset(&mut self) { }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{at, out, inp, denorm};

        let inp = inp::CQnt::inp(inputs);
        let oct = inp::CQnt::oct(inputs);
        let out = out::CQnt::sig(outputs);
        let keys = at::CQnt::keys(atoms);
        let omin = at::CQnt::omin(atoms);
        let omax = at::CQnt::omax(atoms);

        self.quant.update_keys(keys.i(), omin.i(), omax.i());

        if self.quant.has_no_keys() {
            for frame in 0..ctx.nframes() {
                out.write(
                    frame,
                    denorm::CQnt::inp(inp, frame)
                    + denorm::CQnt::oct(oct, frame));
            }

            ctx_vals[1].set(100.0); // some unreachable value for Keys widget
            ctx_vals[0].set(out.read(ctx.nframes() - 1));

        } else {
            let mut last_key = 0;

            for frame in 0..ctx.nframes() {
                let pitch =
                    self.quant.signal_to_pitch(
                        denorm::CQnt::inp(inp, frame));
                out.write(frame, pitch + denorm::CQnt::oct(oct, frame));
            }

            let last_pitch = self.quant.last_key_pitch();
            ctx_vals[1].set(last_pitch * 10.0 + 0.0001);
            ctx_vals[0].set((last_pitch * 10.0 - 0.5) * 2.0);
        }
    }
}
