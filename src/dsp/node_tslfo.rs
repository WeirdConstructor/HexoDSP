// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    DspNode, GraphAtomData, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{TriSawLFO, Trigger};

#[derive(Debug, Clone)]
pub struct TsLFO {
    lfo: Box<TriSawLFO<f64>>,
    trig: Trigger,
}

impl TsLFO {
    pub fn new(_nid: &NodeId) -> Self {
        Self { lfo: Box::new(TriSawLFO::new()), trig: Trigger::new() }
    }

    pub const time: &'static str =
        "The frequency or period time of the LFO, goes all the \
        way from 0.1ms up to 30s. Please note, that the text entry is always \
        in milliseconds.";
    pub const trig: &'static str =
        "Triggers a phase reset of the LFO.";
    pub const rev: &'static str =
        "The reverse point of the LFO waveform. At 0.5 the LFO \
        will follow a triangle waveform. At 0.0 or 1.0 the LFO waveform will \
        be (almost) a (reversed) saw tooth. Node: A perfect sawtooth can not be \
        achieved with this oscillator, as there will always be a minimal \
        rise/fall time.";
    pub const sig: &'static str = "The LFO output.";
    pub const DESC: &'static str = r#"TriSaw LFO

This simple LFO has a configurable waveform. You can blend between triangular to sawtooth waveforms using the 'rev' parameter.
"#;
    pub const HELP: &'static str = r#"TriSaw LFO

This simple LFO has a configurable waveform. You can blend between
triangular to sawtooth waveforms using the 'rev' parameter.

Using the 'trig' input you can reset the LFO phase, which allows to use it
kind of like an envelope.
"#;
}

impl DspNode for TsLFO {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.lfo.set_sample_rate(srate as f64);
    }

    fn reset(&mut self) {
        self.lfo.reset();
        self.trig.reset();
    }

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
        use crate::dsp::{denorm, inp, out};

        let time = inp::TsLFO::time(inputs);
        let trig = inp::TsLFO::trig(inputs);
        let rev = inp::TsLFO::rev(inputs);
        let out = out::TsLFO::sig(outputs);

        let lfo = &mut *self.lfo;

        for frame in 0..ctx.nframes() {
            if self.trig.check_trigger(denorm::TsLFO::trig(trig, frame)) {
                lfo.reset();
            }

            let time_ms = denorm::TsLFO::time(time, frame).clamp(0.1, 300000.0);

            lfo.set((1000.0 / time_ms) as f64, denorm::TsLFO::rev(rev, frame) as f64);

            out.write(frame, lfo.next_unipolar() as f32);
        }

        ctx_vals[0].set(out.read(ctx.nframes() - 1));
    }

    fn graph_fun() -> Option<GraphFun> {
        let mut lfo = TriSawLFO::new();
        lfo.set_sample_rate(160.0);

        Some(Box::new(move |gd: &dyn GraphAtomData, init: bool, _x: f32, _xn: f32| -> f32 {
            if init {
                lfo.reset();
                let time_idx = NodeId::TsLFO(0).inp_param("time").unwrap().inp();
                let rev_idx = NodeId::TsLFO(0).inp_param("rev").unwrap().inp();

                let time = gd.get_norm(time_idx as u32).sqrt();
                let rev = gd.get_norm(rev_idx as u32);
                lfo.set(5.0 * (1.0 - time) + time * 1.0, rev);
            }

            lfo.next_unipolar() as f32
        }))
    }
}
