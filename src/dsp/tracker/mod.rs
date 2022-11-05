// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod pattern;
mod sequencer;

use ringbuf::{Consumer, Producer, RingBuffer};

use std::sync::{Arc, Mutex};

pub const MAX_COLS: usize = 6;
pub const MAX_PATTERN_LEN: usize = 256;
pub const MAX_RINGBUF_SIZE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternColType {
    Note,
    Step,
    Value,
    Gate,
}

pub use pattern::{PatternData, UIPatternModel};
pub use sequencer::PatternSequencer;

#[derive(Debug, Clone, Copy)]
pub enum PatternUpdateMsg {
    UpdateColumn {
        col: usize,
        col_type: PatternColType,
        pattern_len: usize,
        data: [(f32, u8); MAX_PATTERN_LEN],
    },
}

pub struct Tracker {
    data: Arc<Mutex<PatternData>>,
    data_prod: Producer<PatternUpdateMsg>,
    seq: Option<PatternSequencer>,
    seq_cons: Option<Consumer<PatternUpdateMsg>>,
}

impl Clone for Tracker {
    fn clone(&self) -> Self {
        Tracker::new()
    }
}

pub struct TrackerBackend {
    seq: PatternSequencer,
    seq_cons: Consumer<PatternUpdateMsg>,
    col_types: [PatternColType; MAX_COLS],
}

impl std::fmt::Debug for TrackerBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tracker")
            .field("col_types", &self.col_types)
            .field("seq", &"PatternSequencer")
            .field("seq_cons", &"RingbufConsumer")
            .finish()
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Tracker {
    pub fn new() -> Self {
        let rb = RingBuffer::new(MAX_RINGBUF_SIZE);
        let (prod, con) = rb.split();

        Self {
            data: Arc::new(Mutex::new(PatternData::new(MAX_PATTERN_LEN))),
            data_prod: prod,
            seq: Some(PatternSequencer::new(MAX_PATTERN_LEN)),
            seq_cons: Some(con),
        }
    }

    pub fn data(&self) -> Arc<Mutex<PatternData>> {
        self.data.clone()
    }

    pub fn send_one_update(&mut self) -> bool {
        let mut data = self.data.lock().unwrap();

        for col in 0..MAX_COLS {
            if data.col_is_modified_reset(col) {
                data.sync_out_data(col);
                let out_data = data.get_out_data();
                let msg = PatternUpdateMsg::UpdateColumn {
                    col_type: data.col_type(col),
                    pattern_len: data.rows(),
                    data: out_data[col],
                    col,
                };

                let _ = self.data_prod.push(msg);

                return true;
            }
        }

        false
    }

    pub fn get_backend(&mut self) -> TrackerBackend {
        if self.seq.is_none() {
            let rb = RingBuffer::new(MAX_RINGBUF_SIZE);
            let (prod, con) = rb.split();

            self.seq = Some(PatternSequencer::new(MAX_PATTERN_LEN));
            self.data_prod = prod;
            self.seq_cons = Some(con);
        }

        let seq = self.seq.take().unwrap();
        let seq_cons = self.seq_cons.take().unwrap();

        TrackerBackend { seq, seq_cons, col_types: [PatternColType::Value; MAX_COLS] }
    }
}

impl TrackerBackend {
    pub fn check_updates(&mut self) -> bool {
        if let Some(msg) = self.seq_cons.pop() {
            match msg {
                PatternUpdateMsg::UpdateColumn { col, col_type, pattern_len, data } => {
                    self.col_types[col] = col_type;
                    self.seq.set_rows(pattern_len);
                    self.seq.set_col(col, &data);
                }
            }

            true
        } else {
            false
        }
    }

    pub fn pattern_len(&self) -> usize {
        self.seq.rows()
    }

    pub fn get_col_at_phase(
        &mut self,
        col: usize,
        phase: &[f32],
        out: &mut [f32],
        out_gate: &mut [f32],
    ) {
        if self.seq.rows() == 0 {
            return;
        }

        match self.col_types[col] {
            PatternColType::Note | PatternColType::Step => {
                self.seq.col_get_at_phase(col, phase, out, out_gate)
            }
            PatternColType::Value => self.seq.col_interpolate_at_phase(col, phase, out, out_gate),
            PatternColType::Gate => self.seq.col_gate_at_phase(col, phase, out, out_gate),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_float_eq {
        ($a:expr, $b:expr) => {
            if ($a - $b).abs() > 0.0001 {
                panic!(
                    r#"assertion failed: `(left == right)`
      left: `{:?}`,
     right: `{:?}`"#,
                    $a, $b
                )
            }
        };
    }

    #[test]
    fn check_tracker_com_step() {
        let mut t = Tracker::new();
        let mut backend = t.get_backend();

        t.data().lock().unwrap().set_rows(16);
        t.data().lock().unwrap().set_col_step_type(0);
        t.data().lock().unwrap().set_cell_value(0, 0, 0xFFF);
        t.data().lock().unwrap().set_cell_value(7, 0, 0x777);
        t.data().lock().unwrap().set_cell_value(15, 0, 0x000);

        while t.send_one_update() {}
        while backend.check_updates() {}

        let mut out = [0.0; 16];
        let mut out_gate = [0.0; 16];

        backend.get_col_at_phase(0, &[0.2, 0.5, 0.99], &mut out[..], &mut out_gate[..]);

        assert_float_eq!(out[0], 1.0);
        assert_float_eq!(out[1], 0.46666666);
        assert_float_eq!(out[2], 0.0);
        assert_float_eq!(out_gate[0], 0.0);
        assert_float_eq!(out_gate[1], 1.0);
        assert_float_eq!(out_gate[2], 1.0);
    }

    #[test]
    fn check_tracker_com_interp() {
        let mut t = Tracker::new();
        let mut backend = t.get_backend();

        t.data().lock().unwrap().set_rows(16);
        t.data().lock().unwrap().set_col_value_type(0);
        t.data().lock().unwrap().set_cell_value(0, 0, 0xFFF);
        t.data().lock().unwrap().set_cell_value(7, 0, 0x777);
        t.data().lock().unwrap().set_cell_value(15, 0, 0x000);

        while t.send_one_update() {}
        while backend.check_updates() {}

        let mut out = [0.0; 16];
        let mut out_gate = [0.0; 16];

        backend.get_col_at_phase(0, &[0.2, 0.5, 0.999999], &mut out[..], &mut out_gate[..]);
        assert_float_eq!(out[0], 0.83238);
        assert_float_eq!(out[1], 0.46666666);
        assert_float_eq!(out[2], 0.0);
        assert_float_eq!(out_gate[0], 0.0);
        assert_float_eq!(out_gate[1], 0.0);
        assert_float_eq!(out_gate[2], 1.0);
    }

    #[test]
    fn check_tracker_com_gate() {
        let mut t = Tracker::new();
        let mut backend = t.get_backend();

        t.data().lock().unwrap().set_rows(4);
        t.data().lock().unwrap().set_col_gate_type(0);
        t.data().lock().unwrap().set_cell_value(0, 0, 0xFF7);
        t.data().lock().unwrap().clear_cell(1, 0);
        t.data().lock().unwrap().set_cell_value(2, 0, 0xFF0);
        t.data().lock().unwrap().set_cell_value(3, 0, 0xFFF);

        while t.send_one_update() {}
        while backend.check_updates() {}

        let mut out = [0.0; 64];
        let mut out_gate = [0.0; 64];

        let mut phase = [0.0; 64];
        for (i, p) in phase.iter_mut().enumerate() {
            *p = i as f32 / 63.0;
        }

        //d// println!("----");
        backend.get_col_at_phase(0, &phase[..], &mut out[..], &mut out_gate[..]);
        //d// println!("out: {:?}", &out[16..32]);

        assert_eq!(out[0..8], [1.0; 8]);
        assert_eq!(out[8..16], [0.0; 8]);
        assert_eq!(out[16..32], [0.0; 16]);
        assert_eq!(out_gate[0..8], [1.0; 8]);
        assert_eq!(out_gate[8..16], [1.0; 8]);
        assert_eq!(out_gate[16..32], [0.0; 16]);

        assert_float_eq!(out[32], 1.0);
        assert_eq!(out[33..48], [0.0; 15]);
        assert_float_eq!(out_gate[32], 1.0);
        assert_eq!(out_gate[33..48], [1.0; 15]);

        assert_eq!(out[48..64], [1.0; 16]);
        assert_eq!(out_gate[48..64], [1.0; 16]);
    }
}
