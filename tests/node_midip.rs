// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_midip_gate_inserts() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    // Create a DSP matrix with a "MidiP" node and an Out node:
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("midip", "gate").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    // Test run for 5ms with 3 Note On events at sample positions
    // 5, 10 and 130 in this block of samples:
    let (ch1, _) = node_exec.test_run(
        0.005,
        false,
        vec![
            HxTimedEvent::note_on(5, 0, 69, 1.0),
            HxTimedEvent::note_on(10, 0, 68, 1.0),
            HxTimedEvent::note_on(130, 0, 57, 1.0),
        ],
    );

    // Collect the signal changes (raising edges):
    let changes = collect_signal_changes(&ch1[..], 0);

    assert_eq!(
        changes,
        vec![
            (5, 100),   // First note triggers right
            (11, 100),  // Second note needs to shortly pause the gate, which has 1 sample delay
            (131, 100), // Third note also shortly pauses one sample later.
        ]
    );
}

#[test]
fn check_node_midip_pitch_track() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    // Create a DSP matrix with a "MidiP" node and an Out node:
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("midip", "freq").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    // Test run for 5ms with 3 Note On events at sample positions
    // 5, 10 and 130 in this block of samples:
    let (ch1, _) = node_exec.test_run(
        0.005,
        false,
        vec![
            HxTimedEvent::note_on(5, 0, 69, 1.0),
            HxTimedEvent::note_on(10, 0, 68, 1.0),
            HxTimedEvent::note_on(130, 0, 57, 1.0),
        ],
    );

    let changes = collect_signal_changes_flt(&ch1[..], 0.01);
    assert_eq!(changes, vec![(0, -0.575), (5, 0.0), (130, -0.1)]);
}

#[test]
fn check_node_midip_pitch_det() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    // Create a DSP matrix with a "MidiP" node and an Out node:
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("midip", "freq")
        .set_denorm("det", 0.1)
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    // Test run for 5ms with 3 Note On events at sample positions
    // 5, 10 and 130 in this block of samples:
    let (ch1, _) = node_exec.test_run(
        0.005,
        false,
        vec![
            HxTimedEvent::note_on(5, 0, 69, 1.0),
            HxTimedEvent::note_on(10, 0, 68, 1.0),
            HxTimedEvent::note_on(130, 0, 57, 1.0),
        ],
    );

    let changes = collect_signal_changes_flt(&ch1[..], 0.01);
    assert_eq!(changes, vec![(0, -0.475), (5, 0.1), (130, 0.0)]);
}

#[test]
fn check_node_midip_vel_track() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    // Create a DSP matrix with a "MidiP" node and an Out node:
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("midip", "vel").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    // Test run for 5ms with 3 Note On events at sample positions
    // 5, 10 and 130 in this block of samples:
    let (ch1, _) = node_exec.test_run(
        0.005,
        false,
        vec![
            HxTimedEvent::note_on(5, 0, 69, 0.4),
            HxTimedEvent::note_on(10, 0, 68, 1.0),
            HxTimedEvent::note_on(130, 0, 57, 0.6),
        ],
    );

    let changes = collect_signal_changes_flt(&ch1[..], 0.01);
    assert_eq!(changes, vec![(5, 0.4), (10, 1.0), (130, 0.6)]);
}
