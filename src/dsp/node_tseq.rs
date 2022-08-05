// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use synfx_dsp::{Trigger, TriggerPhaseClock};
use crate::dsp::tracker::TrackerBackend;
use crate::dsp::{DspNode, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};

use crate::dsp::MAX_BLOCK_SIZE;

#[macro_export]
macro_rules! fa_tseq_cmode {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "RowT",
            1 => "PatT",
            2 => "Phase",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

#[derive(Debug)]
pub struct TSeqTime {
    clock: TriggerPhaseClock,
    trigger: Trigger,
}

/// A tracker based sequencer
#[derive(Debug)]
pub struct TSeq {
    backend: Option<Box<TrackerBackend>>,
    srate: f64,
    time: Box<TSeqTime>,
}

impl Clone for TSeq {
    fn clone(&self) -> Self {
        Self::new(&NodeId::Nop)
    }
}

impl TSeq {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            backend: None,
            srate: 48000.0,
            time: Box::new(TSeqTime { clock: TriggerPhaseClock::new(), trigger: Trigger::new() }),
        }
    }

    pub fn set_backend(&mut self, backend: TrackerBackend) {
        self.backend = Some(Box::new(backend));
    }

    pub const clock: &'static str = "TSeq clock\nClock input\nRange: (0..1)\n";
    pub const trig: &'static str =
        "TSeq trig\nSynchronization trigger which restarts the sequence.\nRange: (-1..1)\n";
    pub const cmode: &'static str = "TSeq cmode\n'clock' input signal mode:\n\
             - RowT: Trigger = advance row\n\
             - PatT: Trigger = pattern rate\n\
             - Phase: Phase to pattern index\n\
         \n";
    pub const trk1: &'static str = "TSeq trk1\nTrack 1 signal output\nRange: (-1..1)\n";
    pub const trk2: &'static str = "TSeq trk2\nTrack 2 signal output\nRange: (-1..1)\n";
    pub const trk3: &'static str = "TSeq trk3\nTrack 3 signal output\nRange: (-1..1)\n";
    pub const trk4: &'static str = "TSeq trk4\nTrack 4 signal output\nRange: (-1..1)\n";
    pub const trk5: &'static str = "TSeq trk5\nTrack 5 signal output\nRange: (-1..1)\n";
    pub const trk6: &'static str = "TSeq trk6\nTrack 6 signal output\nRange: (-1..1)\n";

    pub const gat1: &'static str = "TSeq gat1\nTrack 1 gate output\nRange: (-1..1)\n";
    pub const gat2: &'static str = "TSeq gat2\nTrack 2 gate output\nRange: (-1..1)\n";
    pub const gat3: &'static str = "TSeq gat3\nTrack 3 gate output\nRange: (-1..1)\n";
    pub const gat4: &'static str = "TSeq gat4\nTrack 4 gate output\nRange: (-1..1)\n";
    pub const gat5: &'static str = "TSeq gat5\nTrack 5 gate output\nRange: (-1..1)\n";
    pub const gat6: &'static str = "TSeq gat6\nTrack 6 gate output\nRange: (-1..1)\n";

    pub const DESC: &'static str = "Tracker Sequencer\n\n\
        This node implements a sequencer that can be programmed \
        using the tracker interface in HexoSynth on the right.\n\
        It provides 6 control signals and 6 gate outputs.";
    pub const HELP: &'static str = r#"Tracker (based) Sequencer

This sequencer gets it's speed from the clock source. The 'clock'
signal can be interpreted in different modes. But if you want to
run multiple sequencers in parallel, you want to synchronize them.
For this you can use the 'trig' input, it resets the played row to
the beginning of the sequence every time a trigger is received.

Alternatively you can run the sequencer clock using the phase mode.
With that the phase (0..1) signal on the 'clock' input determines the
exact play head position in the pattern. With this you just need to
synchronize the phase generators for different sequencers.

For an idea how to chain multiple tracker sequencers, see the next page.

This tracker provides 6 columns that each can have one of the following
types:

- Note column: for specifying pitches.
- Step column: for specifying non interpolated control signals.
- Value column: for specifying linearly interpolated control signals.
- Gate column: for specifying gates, with probability and ratcheting.

Step, value and gate cells can be set to 4096 (0xFFF) different values
or contain nothing at all. For step and value columns these values
are mapped to the 0.0-1.0 control signal range, with 0xFFF being 1.0
and 0x000 being 0.0.

Value examples:     1.0   0.9  0.75   0.5  0.25   0.1
                  0xFFF 0xE70 0xC00 0x800 0x400 0x19A
Gate examples:
    Probability  Ratcheting   Gate Length        full on gate: 0xFFF
      6%  0x000  16   0x000   1/16  0x000      2 short pulses: 0xFE0
     18%  0x200  14   0x020   3/16  0x002      4 short pulses: 0xFC0
     25%  0x300  13   0x030   4/16  0x003        2 50% pulses: 0xFE7
     50%  0x700   9   0x070   8/16  0x007        half on gate: 0xFF7
     62%  0x900   7   0x090  10/16  0x009         short pulse: 0xFF0
     75%  0xC00   4   0x0C0  12/16  0x00C    rare short pulse: 0xEF0
     87%  0xE00   2   0x0E0  15/16  0x00E   50/50 short pulse: 0x7F0
    100%  0xF00   1   0x0F0  16/16  0x00F   50/50 full gate:   0x7FF

On the next page you can read about the gate cells and the gate outputs.
---page---
Gate Input and Output

The gate cells are differently coded:

- 0x00F: The least significant nibble controls the gate length.
         With 0x00F being the full row, and 0x000 being 1/16th of a row.
- 0x0F0: The second nibble controls ratcheting, with 0x0F0 being one
         gate per row, and 0x000 being 16 gates per row.
         Length of these gates is controlled by the last significant nibble.
- 0xF00: The most significant nibble controls probability of the
         whole gate cell. With 0xF00 meaning the gate will always be
         triggered, and 0x000 means that the gate is only triggered with
         6% probability. 50% is 0x700.

The behaviour of the 6 gate outputs of TSeq depend on the corresponding
column type:

- Step gat1-gat6:  Like note columns, this will output a 1.0 for the whole
                   row if a step value is set. With two step values directly
                   following each other no 0.0 will be emitted in between
                   the rows. This means if you want to drive an envelope
                   with release phase with this signal, you need to make
                   space for the release phase.
- Note gat1-gat6:  Behaves just like step columns.
- Gate gat1-gat6:  Behaves just like step columns.
- Value gat1-gat6: Outputs a 1.0 value for the duration of the last row.
                   You can use this to trigger other things once the
                   sequence has been played.

Tip:
    If you want to use the end of a tracker sequence as trigger for
    something else, eg. switching to a different 'tseq' and restart
    it using it's 'trig' input, you will need to use the gate output
    of a value column and invert it.
"#;
}

impl DspNode for TSeq {
    fn outputs() -> usize {
        1
    }

    fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate as f64;
    }

    fn reset(&mut self) {
        self.backend = None;
        self.time.clock.reset();
        self.time.trigger.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self,
        ctx: &mut T,
        _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{at, denorm, inp, out};
        let clock = inp::TSeq::clock(inputs);
        let trig = inp::TSeq::trig(inputs);
        let cmode = at::TSeq::cmode(atoms);

        let backend = if let Some(backend) = &mut self.backend {
            backend
        } else {
            return;
        };

        backend.check_updates();

        let mut phase_out: [f32; MAX_BLOCK_SIZE] = [0.0; MAX_BLOCK_SIZE];

        let cmode = cmode.i();
        let plen = backend.pattern_len().max(1) as f64;

        let time = &mut self.time;

        for frame in 0..ctx.nframes() {
            if time.trigger.check_trigger(denorm::TSeq::trig(trig, frame)) {
                time.clock.sync();
                println!("CLOCK SYNC");
            }

            let phase = match cmode {
                0 => time.clock.next_phase(plen, clock.read(frame)) / plen,
                1 => time.clock.next_phase(1.0, clock.read(frame)),
                2 | _ => (clock.read(frame).abs() as f64).fract(),
            };

            phase_out[frame] = phase as f32;
        }

        let mut col_out: [f32; MAX_BLOCK_SIZE] = [0.0; MAX_BLOCK_SIZE];
        let mut col_out_gate: [f32; MAX_BLOCK_SIZE] = [0.0; MAX_BLOCK_SIZE];
        let col_out_slice = &mut col_out[0..ctx.nframes()];
        let col_out_gate_slice = &mut col_out_gate[0..ctx.nframes()];
        let phase_out_slice = &phase_out[0..ctx.nframes()];

        let out_t1 = out::TSeq::trk1(outputs);
        backend.get_col_at_phase(0, phase_out_slice, col_out_slice, col_out_gate_slice);
        out_t1.write_from(col_out_slice);

        let out_g1 = out::TSeq::gat1(outputs);
        out_g1.write_from(col_out_gate_slice);

        ctx_vals[0].set(col_out_slice[col_out_slice.len() - 1]);

        let out_t2 = out::TSeq::trk2(outputs);
        backend.get_col_at_phase(1, phase_out_slice, col_out_slice, col_out_gate_slice);
        out_t2.write_from(col_out_slice);

        let out_g2 = out::TSeq::gat2(outputs);
        out_g2.write_from(col_out_gate_slice);

        let out_t3 = out::TSeq::trk3(outputs);
        backend.get_col_at_phase(2, phase_out_slice, col_out_slice, col_out_gate_slice);
        out_t3.write_from(col_out_slice);

        let out_g3 = out::TSeq::gat3(outputs);
        out_g3.write_from(col_out_gate_slice);

        let out_t4 = out::TSeq::trk4(outputs);
        backend.get_col_at_phase(3, phase_out_slice, col_out_slice, col_out_gate_slice);
        out_t4.write_from(col_out_slice);

        let out_g4 = out::TSeq::gat4(outputs);
        out_g4.write_from(col_out_gate_slice);

        let out_t5 = out::TSeq::trk5(outputs);
        backend.get_col_at_phase(4, phase_out_slice, col_out_slice, col_out_gate_slice);
        out_t5.write_from(col_out_slice);

        let out_g5 = out::TSeq::gat5(outputs);
        out_g5.write_from(col_out_gate_slice);

        let out_t6 = out::TSeq::trk6(outputs);
        backend.get_col_at_phase(5, phase_out_slice, col_out_slice, col_out_gate_slice);
        out_t6.write_from(col_out_slice);

        let out_g6 = out::TSeq::gat6(outputs);
        out_g6.write_from(col_out_gate_slice);

        ctx_vals[1].set(phase_out_slice[phase_out_slice.len() - 1]);
    }
}
