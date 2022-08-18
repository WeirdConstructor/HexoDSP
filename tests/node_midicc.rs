// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_midicc_test_receive() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("midicc", "sig1")
        .set_atom("cc1", SAtom::setting(10))
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("midicc", "sig2")
        .set_atom("cc2", SAtom::setting(12))
        .node_inp("out", "ch2")
        .place(&mut matrix, 1, 0)
        .unwrap();
    matrix.sync().unwrap();

    // Test run for 5ms with 3 Note On events at sample positions
    // 5, 10 and 130 in this block of samples:
    let (ch1, ch2) = node_exec.test_run(
        0.005,
        false,
        &[
            HxTimedEvent::cc(3, 0, 12, 1.55),
            HxTimedEvent::cc(5, 0, 10, 0.55),
            HxTimedEvent::cc(100, 0, 10, 0.35),
            HxTimedEvent::cc(120, 0, 10, 0.15),
            HxTimedEvent::cc(190, 0, 10, 0.05),
            HxTimedEvent::cc(200, 0, 12, 1.15),
        ],
    );

    let changes = collect_signal_changes(&ch1[..], 0);
    assert_eq!(changes, &[(5, 55), (100, 35), (120, 15), (190, 5)]);

    let changes = collect_signal_changes(&ch2[..], 0);
    assert_eq!(changes, &[(3, 155), (200, 115)]);
}

#[test]
fn check_node_midicc_test_slew() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("midicc", "sig1")
        .set_atom("cc1", SAtom::setting(10))
        .set_denorm("slew", 4.0)
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    // Test run for 5ms with 3 Note On events at sample positions
    // 5, 10 and 130 in this block of samples:
    let (ch1, ch2) = node_exec.test_run(
        0.005,
        false,
        &[
            HxTimedEvent::cc(3, 0, 12, 1.55),
            HxTimedEvent::cc(5, 0, 10, 0.55),
            HxTimedEvent::cc(100, 0, 10, 0.35),
            HxTimedEvent::cc(120, 0, 10, 0.15),
            HxTimedEvent::cc(190, 0, 10, 0.05),
            HxTimedEvent::cc(200, 0, 12, 1.15),
        ],
    );

    let changes: Vec<(usize, i32)> = collect_signal_changes_flt(&ch1[..], 0.0)
        .iter()
        .step_by(20)
        .map(|(a, b)| (*a, (b * 1000.0).round() as i32))
        .collect();
    assert_eq!(
        changes,
        &[
            (5, 6),
            (25, 119),
            (45, 232),
            (65, 346),
            (85, 459),
            (105, 505),
            (125, 391),
            (145, 278),
            (165, 164),
            (206, 54)
        ]
    );
}

#[test]
fn check_node_midicc_test_slew2() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("midicc", "sig1")
        .set_atom("cc1", SAtom::setting(10))
        .set_denorm("slew", 2.0)
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    // Test run for 5ms with 3 Note On events at sample positions
    // 5, 10 and 130 in this block of samples:
    let (ch1, ch2) = node_exec.test_run(
        0.005,
        false,
        &[
            HxTimedEvent::cc(3, 0, 12, 1.55),
            HxTimedEvent::cc(5, 0, 10, 0.55),
            HxTimedEvent::cc(100, 0, 10, 0.35),
            HxTimedEvent::cc(120, 0, 10, 0.15),
            HxTimedEvent::cc(190, 0, 10, 0.05),
            HxTimedEvent::cc(200, 0, 12, 1.15),
        ],
    );

    let changes: Vec<(usize, i32)> = collect_signal_changes_flt(&ch1[..], 0.0)
        .iter()
        .step_by(20)
        .map(|(a, b)| (*a, (b * 1000.0).round() as i32))
        .collect();
    assert_eq!(changes, &[(5, 11), (25, 238), (45, 465), (111, 414), (133, 191),]);
}
