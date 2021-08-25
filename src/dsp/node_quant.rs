// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers::Quantizer;

#[macro_export]
macro_rules! fa_quant { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    write!($formatter, "?")
} } }

#[macro_export]
macro_rules! fa_quant_omin { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
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
macro_rules! fa_quant_omax { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
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

/// A 9 channel signal multiplexer
#[derive(Debug, Clone)]
pub struct Quant {
    quant: Quantizer,
}

impl Quant {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            quant: Quantizer::new(),
        }
    }
    pub const inp : &'static str =
        "Quant inp\n\nRange: (0..1)";
    pub const oct : &'static str =
        "Quant oct\n\nRange: (-1..1)";
    pub const omin : &'static str =
        "Quant omin\n\nRange: (-1..1)";
    pub const omax : &'static str =
        "Quant omax\n\nRange: (-1..1)";
    pub const sig : &'static str =
        "Quant sig\n\nRange: (-1..1)";
    pub const keys : &'static str =
        "Quant keys\n";
    pub const DESC : &'static str =
r#"Pitch/Note Quantizer

"#;
    pub const HELP : &'static str =
r#"Quant - A pitch quantizer

"#;
}

impl DspNode for Quant {
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

        let inp = inp::Quant::inp(inputs);
        let oct = inp::Quant::oct(inputs);
        let out = out::Quant::sig(outputs);
        let keys = at::Quant::keys(atoms);
        let omin = at::Quant::omin(atoms);
        let omax = at::Quant::omax(atoms);

        self.quant.update_keys(keys.i(), omin.i(), omax.i());

        if self.quant.has_no_keys() {
            for frame in 0..ctx.nframes() {
                out.write(
                    frame,
                    denorm::Quant::inp(inp, frame)
                    + denorm::Quant::oct(oct, frame));
            }

            ctx_vals[1].set(100.0); // some unreachable value for Keys widget
            ctx_vals[0].set(out.read(ctx.nframes() - 1));

        } else {
            let mut last_key = 0;

            for frame in 0..ctx.nframes() {
                let pitch =
                    self.quant.signal_to_pitch(
                        denorm::Quant::inp(inp, frame));
                out.write(frame, pitch + denorm::Quant::oct(oct, frame));
            }

            let last_pitch = self.quant.last_key_pitch();
            ctx_vals[1].set(last_pitch * 10.0 + 0.0001);
            ctx_vals[0].set((last_pitch * 10.0 - 0.5) * 2.0);
        }
    }
}
