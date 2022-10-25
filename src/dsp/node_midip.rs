// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{
    at, denorm, inp, out_idx, DspNode, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom,
};
use crate::nodes::{HxMidiEvent, MidiEventPointer, NodeAudioContext, NodeExecContext};
use synfx_dsp::{GateSignal, TrigSignal};

#[macro_export]
macro_rules! fa_midip_chan {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        write!($formatter, "{}", $v.round() as usize)
    }};
}

#[macro_export]
macro_rules! fa_midip_gmode {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "MIDI",
            1 => "Trigger",
            2 => "Gate Len",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

/// The (stereo) output port of the plugin
#[derive(Debug, Clone)]
pub struct MidiP {
    next_gate: i8,
    cur_note: u8,
    cur_gate: u8,
    cur_vel: f32,
    trig_sig: TrigSignal,
    gate_sig: GateSignal,
}

impl MidiP {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            next_gate: 0,
            cur_note: 0,
            cur_gate: 0,
            cur_vel: 0.0,
            trig_sig: TrigSignal::new(),
            gate_sig: GateSignal::new(),
        }
    }

    pub const chan: &'static str = "MIDI Channel 0 to 15\n";
    pub const gmode: &'static str = "MIDI gate mode.\n- **MIDI** gate same as MIDI input\n- **Trigger** output only triggers on ~~gate~~ output\n- **Gate Len** output gate with the length of the ~~glen~~ parameter\n";
    pub const glen: &'static str = "MIDI gate length\n\
        If ~~gmode~~ is set to **Gate Len** this controls and overrides the gate length on a MIDI \
        note event. **Trigger** will just send a short trigger when a note event is received. \
        **MIDI** means the gate reflects the note on/off duration.";
    pub const det: &'static str = "Detune input pitch a bit";
    pub const freq: &'static str = "MIDI note frequency, detuned by ~~det~~.";
    pub const gate: &'static str = "MIDI note gate";
    pub const vel: &'static str = "MIDI note velocity";

    pub const DESC: &'static str = "MIDI Pitch/Note Input\n\n\
        This node is an input of MIDI note events into the DSP graph. \
        You get 3 outputs: frequency of the note, gate signal for the length of the note and the velocity.";
    pub const HELP: &'static str = r#"MIDI Pitch/Note Input

This node is an input of MIDI note events into the DSP graph.
You get 3 outputs: frequency of the note, gate signal for the length of
the note and the velocity.

You can modify the gate length using the ~~gmode~~ and ~~glen~~ settings.
Setting ~~gmode~~ to **Trigger** allows you to get only a short trigger
signal, which might be helpful in some situations.
The **Gate Len** setting allows you to overwrite the gate length with a
custom and fixed gate length. However, if new note is played on this
MIDI channel, the gate will restart after a very short pause.
"#;

    fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for MidiP {
    fn set_sample_rate(&mut self, srate: f32) {
        self.trig_sig.set_sample_rate(srate);
        self.gate_sig.set_sample_rate(srate);
    }
    fn reset(&mut self) {
        self.trig_sig.reset();
        self.gate_sig.reset();
    }

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        let det = inp::MidiP::det(inputs);
        let glen = inp::MidiP::glen(inputs);
        let chan = at::MidiP::chan(atoms);
        let gmode = at::MidiP::gmode(atoms);
        let out_i = out_idx::MidiP::gate();
        let (freq, r) = outputs.split_at_mut(out_i);
        let (gate, vel) = r.split_at_mut(1);
        let freq = &mut freq[0];
        let gate = &mut gate[0];
        let vel = &mut vel[0];

        let midip_channel = (chan.i() as usize % 16) as u8;

        let mut ptr = MidiEventPointer::new(&ectx.midi_notes[..]);

        let gmode = gmode.i();

        for frame in 0..ctx.nframes() {
            let gate_len = denorm::MidiP::glen(glen, frame);

            if self.next_gate > 0 {
                self.cur_gate = 1;
            } else if self.next_gate < 0 {
                self.cur_gate = 0;
            }
            self.next_gate = 0;

            while let Some(ev) = ptr.next_at(frame) {
                match ev {
                    HxMidiEvent::NoteOn { channel, note, vel } => {
                        if channel != midip_channel {
                            continue;
                        }

                        if self.cur_gate > 0 {
                            self.next_gate = 1;
                            self.cur_gate = 0;
                        } else {
                            self.cur_gate = 1;
                        }
                        self.trig_sig.trigger();
                        self.gate_sig.trigger();
                        self.cur_note = note;
                        self.cur_vel = vel;
                    }
                    HxMidiEvent::NoteOff { channel, note } => {
                        if channel != midip_channel {
                            continue;
                        }

                        if self.cur_note == note {
                            self.next_gate = -1;
                        }
                    }
                    _ => (),
                }
            }

            match gmode {
                1 => {
                    gate.write(frame, self.trig_sig.next());
                }
                2 => {
                    if self.next_gate > 0 {
                        gate.write(frame, 0.0);
                        self.cur_gate = 0;
                    } else {
                        let gsig = self.gate_sig.next(gate_len);
                        self.cur_gate = gsig.ceil() as u8;
                        gate.write(frame, gsig);
                    }
                }
                _ => {
                    gate.write(frame, if self.cur_gate > 0 { 1.0 } else { 0.0 });
                }
            }

            let note = (self.cur_note as f32 - 69.0) / 120.0;
            let note = note + det.read(frame);
            //d// println!("FRAME: {} => gate={}, freq={}, next_gate={}", frame, self.cur_gate, note, self.next_gate);
            freq.write(frame, note);
            vel.write(frame, self.cur_vel as f32);
        }

        let last_val = gate.read(ctx.nframes() - 1);
        ctx_vals[0].set(last_val);
    }
}
