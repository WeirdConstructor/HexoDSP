// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup_quant(matrix: &mut Matrix, trig_out: bool) {
    let qnt = NodeId::Quant(0);
    let out = NodeId::Out(0);

    if trig_out {
        matrix.place(0, 0, Cell::empty(qnt).out(None, None, qnt.out("t")));
    } else {
        matrix.place(0, 0, Cell::empty(qnt).out(None, None, qnt.out("sig")));
    }
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));

    pset_s(matrix, qnt, "keys", 0);
    matrix.sync().unwrap();
}

#[test]
fn check_node_quant_1() {
    init_test!(matrix, node_exec, 3);
    setup_quant(matrix, false);

    let qnt = NodeId::Quant(0);

    let mut v = vec![];
    for i in 0..20 {
        let x = i as f32 / 200.0;

        pset_n_wait(matrix, node_exec, qnt, "freq", x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![
            0.0, 0.0083, 0.0083, 0.0166, 0.0250, 0.0250, 0.0333, 0.0333, 0.0416, 0.0500, 0.0500,
            0.0583, 0.0583, 0.0666, 0.075, 0.075, 0.0833, 0.0833, 0.0916, 0.1
        ]
    );
}

#[test]
fn check_node_quant_2() {
    init_test!(matrix, node_exec, 3);
    setup_quant(matrix, false);

    let qnt = NodeId::Quant(0);

    let mut v = vec![];
    for i in 0..20 {
        let x = i as f32 / 200.0;

        pset_n_wait(matrix, node_exec, qnt, "freq", -1.0 * x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![
            0.0,
            0.0,
            -1.0 * 0.0083,
            -1.0 * 0.0166,
            -1.0 * 0.0166,
            -1.0 * 0.0250,
            -1.0 * 0.0250,
            -1.0 * 0.0333,
            -1.0 * 0.0416,
            -1.0 * 0.0416,
            -1.0 * 0.0500,
            -1.0 * 0.0500,
            -1.0 * 0.0583,
            -1.0 * 0.0666,
            -1.0 * 0.0666,
            -1.0 * 0.075,
            -1.0 * 0.075,
            -1.0 * 0.0833,
            -1.0 * 0.0916,
            -1.0 * 0.0916,
            -1.0 * 0.1
        ]
    );
}

#[test]
fn check_node_quant_oct() {
    init_test!(matrix, node_exec, 3);
    setup_quant(matrix, false);

    let qnt = NodeId::Quant(0);

    pset_n(matrix, qnt, "oct", 0.1);

    let mut v = vec![];
    for i in 0..20 {
        let x = i as f32 / 200.0;

        pset_n_wait(matrix, node_exec, qnt, "freq", x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![
            0.1 + 0.0,
            0.1 + 0.0083,
            0.1 + 0.0083,
            0.1 + 0.0166,
            0.1 + 0.0250,
            0.1 + 0.0250,
            0.1 + 0.0333,
            0.1 + 0.0333,
            0.1 + 0.0416,
            0.1 + 0.0500,
            0.1 + 0.0500,
            0.1 + 0.0583,
            0.1 + 0.0583,
            0.1 + 0.0666,
            0.1 + 0.075,
            0.1 + 0.075,
            0.1 + 0.0833,
            0.1 + 0.0833,
            0.1 + 0.0916,
            0.1 + 0.1
        ]
    );
}

#[test]
fn check_node_quant_keys() {
    init_test!(matrix, node_exec, 3);
    setup_quant(matrix, false);

    let quant = NodeId::Quant(0);

    pset_s(matrix, quant, "keys", 0b11100);

    let mut v = vec![];
    for i in 0..20 {
        let x = i as f32 / 200.0;

        pset_n_wait(matrix, node_exec, quant, "freq", x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![
            -0.0416, 0.0416, 0.0416, 0.0416, 0.0416, 0.0416, 0.0416, 0.0416, 0.0416, 0.0500,
            0.0500, 0.0583, 0.0583, 0.0583, 0.0583, 0.0583, 0.0583, 0.0583, 0.0583, 0.0583
        ]
    );
}

#[test]
fn check_node_quant_trig_out() {
    init_test!(matrix, node_exec, 3);
    setup_quant(matrix, true);

    let quant = NodeId::Quant(0);

    pset_s(matrix, quant, "keys", 0b11100);

    let mut v = vec![];
    for i in 0..100 {
        let x = i as f32 / 1000.0;

        pset_n(matrix, quant, "freq", x);
        let res = run_for_ms(node_exec, 10.0);
        v.extend_from_slice(&res.0[..]);
    }

    let idxs_big = collect_signal_changes(&v[..], 50);

    assert_eq!(idxs_big, vec![(0, 100), (1359, 100), (19734, 100), (23409, 100)]);
}
