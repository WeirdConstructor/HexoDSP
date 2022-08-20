// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    denorm, inp, out_idx, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{HxMidiEvent, MidiEventPointer, NodeAudioContext, NodeExecContext};
use synfx_dsp::SlewValue;

/// The (stereo) output port of the plugin
#[derive(Debug, Clone)]
pub struct ExtA {
    slew1: SlewValue<f32>,
    slew2: SlewValue<f32>,
    slew3: SlewValue<f32>,
}

impl ExtA {
    pub fn new(_nid: &NodeId) -> Self {
        Self { slew1: SlewValue::new(), slew2: SlewValue::new(), slew3: SlewValue::new() }
    }

    pub const slew: &'static str = "ExtA slew\nSlew limiter for the 3 parameters\nRange: (0..1)";
    pub const atv1: &'static str = "ExtA atv1\nAttenuverter for the A1 parameter\nRange: (-1..1)";
    pub const atv2: &'static str = "ExtA atv2\nAttenuverter for the A2 parameter\nRange: (-1..1)";
    pub const atv3: &'static str = "ExtA atv3\nAttenuverter for the A3 parameter\nRange: (-1..1)";

    pub const sig1: &'static str = "ExtA sig1\nA1 output channel\nRange: (-1..1)";
    pub const sig2: &'static str = "ExtA sig2\nA2 output channel\nRange: (-1..1)";
    pub const sig3: &'static str = "ExtA sig3\nA3 output channel\nRange: (-1..1)";

    pub const DESC: &'static str = "External Parameter Set A Input\n\n\
        \
        \
        \
        ";
    pub const HELP: &'static str = r#"External Parameter Set A Input
"#;
}

impl DspNode for ExtA {
    fn outputs() -> usize {
        0
    }

    fn set_sample_rate(&mut self, _srate: f32) {}
    fn reset(&mut self) {}

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        let slew = inp::ExtA::slew(inputs);
        let atv1 = inp::ExtA::atv1(inputs);
        let atv2 = inp::ExtA::atv2(inputs);
        let atv3 = inp::ExtA::atv3(inputs);
        let sig2_i = out_idx::ExtA::sig2();
        let (sig1, r) = outputs.split_at_mut(sig2_i);
        let (sig2, sig3) = r.split_at_mut(1);
        let sig1 = &mut sig1[0];
        let sig2 = &mut sig2[0];
        let sig3 = &mut sig3[0];

        if let Some(params) = &ectx.ext_param {
            for frame in 0..ctx.nframes() {
                let slew_ms = denorm::ExtA::slew(slew, frame);
                sig1.write(
                    frame,
                    denorm::ExtA::atv1(atv1, frame) * self.slew1.next(params.a1(), slew_ms),
                );
                sig2.write(
                    frame,
                    denorm::ExtA::atv2(atv2, frame) * self.slew2.next(params.a2(), slew_ms),
                );
                sig3.write(
                    frame,
                    denorm::ExtA::atv3(atv3, frame) * self.slew3.next(params.a3(), slew_ms),
                );
            }
        }

        //        ctx_vals[0].set(if change { 1.0 } else { 0.0 });
    }
}
