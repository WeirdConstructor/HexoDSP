// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    at, denorm, inp, out_idx, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{HxMidiEvent, MidiEventPointer, NodeAudioContext, NodeExecContext};

#[macro_export]
macro_rules! fa_midicc_cc {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        write!($formatter, "{}", $v.round() as usize)
    }};
}

/// The (stereo) output port of the plugin
#[derive(Debug, Clone)]
pub struct MidiCC {
    cur_cc1: f32,
    cur_cc2: f32,
    cur_cc3: f32,
}

impl MidiCC {
    pub fn new(_nid: &NodeId) -> Self {
        Self { cur_cc1: 0.0, cur_cc2: 0.0, cur_cc3: 0.0 }
    }

    pub const chan: &'static str = "MidiCC chan\nMIDI Channel 0 to 15\n";
    pub const slew: &'static str = "MidiCC slew\nSlew limiter for the 3 CCs\n- 'MIDI' gate same as MIDI input\n- 'Trigger' output only triggers on 'gate' output\n- 'Gate Len' output gate with the length of the 'gatel' parameter\n";
    pub const cc1: &'static str = "MidiCC cc1\nMIDI selected CC";
    pub const cc2: &'static str = "MidiCC cc1\nMIDI selected CC";
    pub const cc3: &'static str = "MidiCC cc1\nMIDI selected CC";

    pub const sig1: &'static str = "MidiCC sig1\nCC output channel 1\nRange: (0..1)";
    pub const sig2: &'static str = "MidiCC sig1\nCC output channel 1\nRange: (0..1)";
    pub const sig3: &'static str = "MidiCC sig1\nCC output channel 1\nRange: (0..1)";

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

impl DspNode for MidiCC {
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
        let chan = at::MidiCC::chan(atoms);
        let cc1 = at::MidiCC::cc1(atoms);
        let cc2 = at::MidiCC::cc2(atoms);
        let cc3 = at::MidiCC::cc3(atoms);
        let sig2_i = out_idx::MidiCC::sig2();
        let (sig1, r) = outputs.split_at_mut(sig2_i);
        let (sig2, sig3) = r.split_at_mut(1);
        let sig1 = &mut sig1[0];
        let sig2 = &mut sig2[0];
        let sig3 = &mut sig3[0];

        let midicc_channel = (chan.i() as usize % 16) as u8;
        let midicc_cc1 = (chan.i() as usize % 128) as u8;
        let midicc_cc2 = (chan.i() as usize % 128) as u8;
        let midicc_cc3 = (chan.i() as usize % 128) as u8;

        let mut ptr = MidiEventPointer::new(&ectx.midi_ccs[..]);

        let mut change = false;

        for frame in 0..ctx.nframes() {
            while let Some(ev) = ptr.next_at(frame) {
                match ev {
                    HxMidiEvent::CC { channel, cc, value } => {
                        if channel != midicc_channel {
                            continue;
                        }

                        if cc == midicc_cc1 {
                            self.cur_cc1 = value;
                            change = true;
                        } else if cc == midicc_cc2 {
                            self.cur_cc2 = value;
                            change = true;
                        } else if cc == midicc_cc3 {
                            self.cur_cc3 = value;
                            change = true;
                        }
                    }
                    _ => (),
                }
            }

            sig1.write(frame, self.cur_cc1);
            sig2.write(frame, self.cur_cc2);
            sig3.write(frame, self.cur_cc3);
        }

        ctx_vals[0].set(if change { 1.0 } else { 0.0 });
    }
}
