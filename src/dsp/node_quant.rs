// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals, NodeContext};
use crate::dsp::helpers::{Trigger};

#[macro_export]
macro_rules! fa_quant { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    write!($formatter, "?")
} } }

/// A 9 channel signal multiplexer
#[derive(Debug, Clone)]
pub struct Quant {
}

impl Quant {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
        }
    }
    pub const inp : &'static str =
        "Quant inp\n\nRange: (0..1)";
    pub const oct : &'static str =
        "Quant oct\n\nRange: (-1..1)";
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

        let mut key_count = 0;
        let mut used_keys = [0.0; 12];

        let mask = keys.i();
        let tune_to_a4 = (9.0 / 12.0) * 0.1;
        for i in 0..9 {
            if mask & (0x1 << i) > 0 {
                used_keys[key_count] = ((i as f32 / 12.0) * 0.1) - tune_to_a4;
                key_count += 1;
            }
        }
        for i in 9..12 {
            let key_pitch_idx = (i + 9 + 12) % 12;
            if mask & (0x1 << i) > 0 {
                used_keys[key_count] = (i as f32 / 12.0) * 0.1 - tune_to_a4;
                key_count += 1;
            }
        }


        if key_count == 0 {
            for frame in 0..ctx.nframes() {
                out.write(
                    frame,
                    denorm::Quant::inp(inp, frame)
                    + denorm::Quant::oct(oct, frame));
            }

            ctx_vals[1].set(100.0); // some unreachable value for Keys widget
            ctx_vals[0].set(out.read(ctx.nframes() - 1));
        } else {
            let mut last_pitch = 0.0;

            for frame in 0..ctx.nframes() {
                let key =
                    (denorm::Quant::inp(inp, frame) * (key_count as f32))
                    .floor();
                let key = key as usize % key_count;
                let pitch = used_keys[key];
                last_pitch = pitch;
                out.write(frame, pitch + denorm::Quant::oct(oct, frame));
            }

            ctx_vals[1].set((last_pitch + tune_to_a4) * 10.0 + 0.0001);
            ctx_vals[0].set(((last_pitch + tune_to_a4) * 10.0 - 0.5) * 2.0);
        }
    }
}
