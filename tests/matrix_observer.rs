// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

use std::sync::{Arc, Mutex};

struct MatrixTestRecorder {
    records: Mutex<Vec<String>>,
}

impl MatrixObserver for MatrixTestRecorder {
    fn update_prop(&self, _key: &str) {}
    fn update_monitor(&self, _cell: &Cell) {}
    fn update_param(&self, _param_id: &ParamId) {}
    fn update_matrix(&self) {}
    fn update_all(&self) {}
    fn midi_event(&self, midi_ev: HxMidiEvent) {
        self.records.lock().expect("recorder lock ok").push(format!("{:?}", midi_ev));
    }
}

#[test]
fn check_matrix_observer() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);
    let recorder = Arc::new(MatrixTestRecorder { records: Mutex::new(vec![]) });
    matrix.set_observer(recorder.clone());

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("test", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    matrix.inject_midi_event(HxMidiEvent::NoteOn { channel: 1, note: 57, vel: 0.751 });

    //    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 10.0);
    matrix.handle_graph_events();

    let rec =
        recorder.records.lock().expect("lock recorder for pop").pop().expect("A record present");

    assert_eq!(rec, "NoteOn { channel: 1, note: 57, vel: 0.751 }");
}
