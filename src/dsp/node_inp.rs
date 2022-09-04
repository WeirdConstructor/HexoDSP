// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    denorm, inp, out_idx, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};

/// The (stereo) input port of the plugin
#[derive(Debug, Clone)]
pub struct Inp {}

impl Inp {
    pub fn new(_nid: &NodeId) -> Self {
        Self {}
    }

    pub const gain: &'static str =
        "The gain of the two plugin input ports, applied to all channels. \
        Please note that this is a linear control, to prevent inaccuracies for **1.0**. \
        ";
    pub const sig1: &'static str = "Audio input channel 1 (left)";
    pub const sig2: &'static str = "Audio input channel 2 (right)";

    pub const DESC: &'static str = "Audio Input Port\n\n\
        This node gives you access to the two input ports of the HexoSynth plugin. \
        Build effects or what ever you can imagine with this!
        ";
    pub const HELP: &'static str = r#"Audio Input Port

This node gives you access to the two input ports of the HexoSynth plugin.
You can build an effects plugin with this node and the `Out` node.
Or a synthesizer that reacts to audio rate control signals on these two
input ports.
"#;
}

impl DspNode for Inp {
    fn outputs() -> usize {
        0
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
        let gain = inp::Inp::gain(inputs);

        let sig_i = out_idx::Inp::sig2();
        let (sig1, sig2) = outputs.split_at_mut(sig_i);
        let sig1 = &mut sig1[0];
        let sig2 = &mut sig2[0];

        for frame in 0..ctx.nframes() {
            let gain = denorm::Inp::gain(gain, frame);
            sig1.write(frame, gain * ctx.input(0, frame));
            sig2.write(frame, gain * ctx.input(1, frame));
        }

        let last_val = sig1.read(ctx.nframes() - 1);
        ctx_vals[0].set(last_val);
    }
}
