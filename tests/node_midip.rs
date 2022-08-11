// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_midip_gate_inserts() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("midip", "gate").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    let (ch1, _) = node_exec.test_run(0.005, false, vec![
        HxTimedEvent::note_on(5, 0, 69, 1.0),
        HxTimedEvent::note_on(10, 0, 68, 1.0),
        HxTimedEvent::note_on(130, 0, 57, 1.0),
    ]);

    let changes = collect_signal_changes(&ch1[..], 0);

    assert_eq!(changes, vec![
        (5, 100),
        (11, 100),
        (131, 100),
    ]);
}
