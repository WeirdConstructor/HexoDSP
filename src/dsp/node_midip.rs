// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    at, denorm, inp, out_idx, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{NodeAudioContext, NodeExecContext};

#[macro_export]
macro_rules! fa_midip_chan {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        write!($formatter, "{}", $v.round() as usize)
    }};
}

/// The (stereo) output port of the plugin
#[derive(Debug, Clone)]
pub struct MidiP {
    /// - 0: signal channel 1
    /// - 1: signal channel 2
    #[allow(dead_code)]
    input: [f32; 2],
}

impl MidiP {
    pub fn new(_nid: &NodeId) -> Self {
        Self { input: [0.0; 2] }
    }

    pub const chan: &'static str = "MidiP chan\nMIDI Channel 0 to 15\n";
    pub const det: &'static str = "MidiP det\nDetune input pitch a bit\nRange: (-1..1)";
    pub const freq: &'static str =
        "MidiP freq\nMIDI note frequency, detuned by 'det'.\nRange: (-1..1)";
    pub const gate: &'static str = "MidiP gate\nMIDI note gate\nRange: (0..1)";
    pub const vel: &'static str = "MidiP vel\nMIDI note velocity\nRange: (0..1)";

    pub const ch1: &'static str = "MidiP ch1\nAudio channel 1 (left)\nRange: (-1..1)";
    pub const ch2: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";

    pub const ch3: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch4: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch5: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch6: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch7: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch8: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch9: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch10: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch11: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch12: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch13: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch14: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch15: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch16: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";
    pub const ch17: &'static str = "MidiP ch2\nAudio channel 2 (right)\nRange: (-1..1)";

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

impl DspNode for MidiP {
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
        _inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        let out_i = out_idx::MidiP::gate();
        let (freq, r) = outputs.split_at_mut(out_i);
        let (gate, vel) = r.split_at_mut(1);
        let freq = &mut freq[0];
        let gate = &mut gate[0];
        let vel = &mut vel[0];

        for frame in 0..ctx.nframes() {
            let chan = ectx.note_buffer.get_chan_at(0, frame as u8);
            freq.write(frame, chan.note as f32 * 1.0 / 127.0);
            gate.write(frame, chan.gate as f32);
            vel.write(frame, chan.vel as f32);
        }

        let last_val = gate.read(ctx.nframes() - 1);
        ctx_vals[0].set(last_val);
    }
}
