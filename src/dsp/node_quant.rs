// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphFun, LedPhaseVals, NodeContext, NodeGlobalRef, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{ChangeTrig, Quantizer};

#[macro_export]
macro_rules! fa_quant {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        write!($formatter, "?")
    }};
}

/// A pitch quantizer
#[derive(Debug, Clone)]
pub struct Quant {
    quant: Box<Quantizer>,
    change_trig: ChangeTrig,
}

impl Quant {
    pub fn new(_nid: &NodeId, _node_global: &NodeGlobalRef) -> Self {
        Self { quant: Box::new(Quantizer::new()), change_trig: ChangeTrig::new() }
    }
    pub const freq: &'static str = "Any signal that is to be pitch quantized.";
    pub const oct: &'static str = "Pitch offset, the knob is snapping to octave offsets. \
        Feed signal values snapped to **0.1** multiples for exact octave offsets.\
        ";
    pub const sig: &'static str = "The quantized output signal that is rounded to \
        the next selected note pitch within the octave of the \
        original input to ~~freq~~.";
    pub const keys: &'static str = "Select the notes you want to snap to here. \
        If no notes are selected, the quantizer will snap the \
        incoming signal to any closest note.";
    pub const t: &'static str = "Everytime the quantizer snaps to a new pitch, it will \
        emit a short trigger on this signal output. This is useful \
        to trigger for example an envelope.";
    pub const DESC: &'static str = r#"Pitch Quantizer

This is a simple quantizer, that snaps a pitch signal on ~~freq~~ to the closest selected notes within their octave.
"#;
    pub const HELP: &'static str = r#"A pitch quantizer

This is a simple quantizer, that snaps a pitch signal on ~~freq~~ to the
closest selected notes within their octave.

If you sweep along pitches you will notice that notes that are closer together
are travelled across faster. That means the notes are not evenly distributed
across the pitch input. If you want a more evenly distributed pitch selection
please see also the `CQnt` node.
"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for Quant {
    fn set_sample_rate(&mut self, srate: f32) {
        self.change_trig.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.change_trig.reset();
    }

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp, out_buf};

        let freq = inp::Quant::freq(inputs);
        let oct = inp::Quant::oct(inputs);
        let keys = at::Quant::keys(atoms);
        let mut out = out_buf::CQnt::sig(outputs);
        let mut t = out_buf::CQnt::t(outputs);

        self.quant.set_keys(keys.i());

        for frame in 0..ctx.nframes() {
            let pitch = self.quant.process(freq.read(frame));

            t.write(frame, self.change_trig.next(pitch));
            out.write(frame, pitch + denorm::Quant::oct(oct, frame));
        }

        let last_pitch = self.quant.last_key_pitch();
        ctx_vals[1].set(last_pitch * 10.0 + 0.0001);
        ctx_vals[0].set((last_pitch * 10.0 - 0.5) * 2.0);
    }
}
