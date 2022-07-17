// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{at, denorm, inp, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};

#[macro_export]
macro_rules! fa_out_mono {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Stereo",
            1 => "Mono",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// The (stereo) output port of the plugin
#[derive(Debug, Clone)]
pub struct Out {
    /// - 0: signal channel 1
    /// - 1: signal channel 2
    #[allow(dead_code)]
    input: [f32; 2],
}

impl Out {
    pub fn new(_nid: &NodeId) -> Self {
        Self { input: [0.0; 2] }
    }

    pub const mono: &'static str =
        "Out mono\nIf set to 'Mono', ch1 will be sent to both output channels.\n(UI only)";
    pub const gain: &'static str =
        "Out gain\nThe main gain of the synthesizer output, applied to all channels. \
        Please note that this is a linear control, to prevent inaccuracies for 1.0. \
        \nRange: (0..1)";
    pub const ch1: &'static str = "Out ch1\nAudio channel 1 (left)\nRange: (-1..1)";
    pub const ch2: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";

    pub const ch3: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch4: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch5: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch6: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch7: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch8: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch9: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch10: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch11: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch12: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch13: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch14: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch15: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch16: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch17: &'static str = "Out ch2\nAudio channel 2 (right)\nRange: (-1..1)";

    pub const DESC: &'static str = "Audio Output Port\n\n\
        This output port node allows you to send audio signals \
        to audio devices or tracks in your DAW.";
    pub const HELP: &'static str = r#"Audio Output Port

This output port node allows you to send audio signals to audio devices
or tracks in your DAW. If you need a stereo output but only have a mono
signal you can use the 'mono' setting to duplicate the signal on the 'ch1'
input to the second channel 'ch2'.
"#;
}

impl DspNode for Out {
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
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        _outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        let in1 = inp::Out::ch1(inputs);
        let gain = inp::Out::gain(inputs);

        if at::Out::mono(atoms).i() > 0 {
            for frame in 0..ctx.nframes() {
                let gain = denorm::Out::gain(gain, frame);
                ctx.output(0, frame, gain * in1.read(frame));
                ctx.output(1, frame, gain * in1.read(frame));
            }
        } else {
            let in2 = inp::Out::ch2(inputs);

            for frame in 0..ctx.nframes() {
                let gain = denorm::Out::gain(gain, frame);
                ctx.output(0, frame, gain * in1.read(frame));
                ctx.output(1, frame, gain * in2.read(frame));
            }
        }

        let last_val = in1.read(ctx.nframes() - 1);
        ctx_vals[0].set(last_val);
    }
}
