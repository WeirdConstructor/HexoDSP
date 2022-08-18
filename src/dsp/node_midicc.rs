// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    at, denorm, inp, out_idx, DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{HxMidiEvent, MidiEventPointer, NodeAudioContext, NodeExecContext};
use synfx_dsp::SlewValue;

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
    slew_cc1: SlewValue<f32>,
    slew_cc2: SlewValue<f32>,
    slew_cc3: SlewValue<f32>,
}

impl MidiCC {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            cur_cc1: 0.0,
            cur_cc2: 0.0,
            cur_cc3: 0.0,
            slew_cc1: SlewValue::new(),
            slew_cc2: SlewValue::new(),
            slew_cc3: SlewValue::new(),
        }
    }

    pub const chan: &'static str = "MidiCC chan\nMIDI Channel 0 to 15\n";
    pub const slew: &'static str = "MidiCC slew\nSlew limiter for the 3 CCs\nRange: (0..1)";
    pub const cc1: &'static str = "MidiCC cc1\nMIDI selected CC 1";
    pub const cc2: &'static str = "MidiCC cc2\nMIDI selected CC 2";
    pub const cc3: &'static str = "MidiCC cc3\nMIDI selected CC 3";

    pub const sig1: &'static str = "MidiCC sig1\nCC output channel 1\nRange: (0..1)";
    pub const sig2: &'static str = "MidiCC sig2\nCC output channel 2\nRange: (0..1)";
    pub const sig3: &'static str = "MidiCC sig3\nCC output channel 3\nRange: (0..1)";

    pub const DESC: &'static str = "MIDI CC Input\n\n\
        This node is an input of MIDI CC events/values into the DSP graph. \
        You get 3 CC value outputs: 'sig1', 'sig2' and 'sig3'. To set which CC \
        gets which output you have to set the corresponding 'cc1', 'cc2' and \
        'cc3' parameters.";
    pub const HELP: &'static str = r#"MIDI CC Input

This node is an input of MIDI CC events/values into the DSP graph.
You get 3 CC value outputs: 'sig1', 'sig2' and 'sig3'. To set which CC
gets which output you have to set the corresponding 'cc1', 'cc2' and
'cc3' parameters.";

If the CC values change to rapidly or you hear audible artifacts, you can
try to limit the speed of change with the 'slew' limiter.

If you need different 'slew' values for the CCs, I recommend creating other
MidiCC instances with different 'slew' settings.
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
        let slew = inp::MidiCC::slew(inputs);
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
        let midicc_cc1 = (cc1.i() as usize % 128) as u8;
        let midicc_cc2 = (cc2.i() as usize % 128) as u8;
        let midicc_cc3 = (cc3.i() as usize % 128) as u8;

        let mut ptr = MidiEventPointer::new(&ectx.midi_ccs[..]);

        let mut change = false;

        for frame in 0..ctx.nframes() {
            let slew_ms = denorm::MidiCC::slew(slew, frame);

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

            sig1.write(frame, self.slew_cc1.next(self.cur_cc1, slew_ms));
            sig2.write(frame, self.slew_cc2.next(self.cur_cc2, slew_ms));
            sig3.write(frame, self.slew_cc3.next(self.cur_cc3, slew_ms));
        }

        ctx_vals[0].set(if change { 1.0 } else { 0.0 });
    }
}
