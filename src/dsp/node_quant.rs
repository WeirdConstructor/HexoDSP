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

/// A pitch quantizer
#[derive(Debug, Clone)]
pub struct Quant {
    quant: Box<Quantizer>,
}

impl Quant {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            quant: Box::new(Quantizer::new()),
        }
    }
    pub const freq : &'static str =
        "Quant freq\n\nRange: (0..1)";
    pub const oct : &'static str =
        "Quant oct\n\nRange: (-1..1)";
    pub const sig : &'static str =
        "Quant sig\n\nRange: (-1..1)";
    pub const keys : &'static str =
        "Quant keys\n";
    pub const DESC : &'static str =
r#"Pitch Quantizer

This is a simple quantizer, that snaps a pitch signal on 'freq' to the closest selected notes within their octave.
"#;
    pub const HELP : &'static str =
r#"Quant - A pitch quantizer

This is a simple quantizer, that snaps a pitch signal on 'freq' to the
closest selected notes within their octave.

If you sweep along pitches you will notice that notes that are closer together
are travelled across faster. That means the notes are not evenly distributed
across the pitch input. If you want a more evenly distributed pitch selection
please see also the 'CQnt' node.
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

        let freq = inp::Quant::freq(inputs);
        let oct  = inp::Quant::oct(inputs);
        let keys = at::Quant::keys(atoms);
        let out  = out::Quant::sig(outputs);

        self.quant.set_keys(keys.i());

//        let mut last_key = 0;

        for frame in 0..ctx.nframes() {
            let pitch = self.quant.process(freq.read(frame));
            out.write(frame, pitch + denorm::Quant::oct(oct, frame));
        }

//        let last_pitch = self.quant.last_key_pitch();
//        ctx_vals[1].set(last_pitch * 10.0 + 0.0001);
//        ctx_vals[0].set((last_pitch * 10.0 - 0.5) * 2.0);
    }
}
