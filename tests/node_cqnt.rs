// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup_cqnt(matrix: &mut Matrix, trig_out: bool) {
    let cqnt = NodeId::CQnt(0);
    let out = NodeId::Out(0);

    if trig_out {
        matrix.place(0, 0, Cell::empty(cqnt).out(None, None, cqnt.out("t")));
    } else {
        matrix.place(0, 0, Cell::empty(cqnt).out(None, None, cqnt.out("sig")));
    }
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));

    pset_s(matrix, cqnt, "keys", 0);
    matrix.sync().unwrap();
}

#[test]
fn check_node_cqnt_1() {
    init_test!(matrix, node_exec, 3);
    setup_cqnt(matrix, false);

    let cqnt = NodeId::CQnt(0);

    let mut v = vec![];
    for i in 0..20 {
        let x = i as f32 / 20.0;

        pset_n_wait(matrix, node_exec, cqnt, "inp", x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![
            -0.075, -0.075, -0.0666, -0.0666, -0.0583, -0.05, -0.05, -0.04166, -0.04166, -0.0333,
            -0.0250, -0.0250, -0.0166, -0.0166, -0.0083, 0.0, 0.0, 0.0083, 0.0083, 0.0166
        ]
    );
}

#[test]
fn check_node_cqnt_one_key() {
    init_test!(matrix, node_exec, 3);
    setup_cqnt(matrix, false);

    let cqnt = NodeId::CQnt(0);

    pset_s(matrix, cqnt, "keys", 0b100);

    let mut v = vec![];
    for i in 0..10 {
        let x = i as f32 / 10.0;

        pset_n_wait(matrix, node_exec, cqnt, "inp", x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![-0.05833, -0.05833, -0.05833, -0.05833, -0.05833, -0.05833, -0.05833, -0.05833,]
    );
}

#[test]
fn check_node_cqnt_one_key_oct() {
    init_test!(matrix, node_exec, 3);
    setup_cqnt(matrix, false);

    let cqnt = NodeId::CQnt(0);

    pset_s(matrix, cqnt, "keys", 0b100);
    pset_s(matrix, cqnt, "omin", 1);
    pset_s(matrix, cqnt, "omax", 1);

    let mut v = vec![];
    for i in 0..10 {
        let x = i as f32 / 10.0;

        pset_n_wait(matrix, node_exec, cqnt, "inp", x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![-0.1583, -0.1583, -0.1583, -0.1583, -0.0583, -0.0583, -0.0583, 0.0416, 0.0416, 0.0416]
    );
}

#[test]
fn check_node_cqnt_min_max_oct() {
    init_test!(matrix, node_exec, 3);
    setup_cqnt(matrix, false);

    let cqnt = NodeId::CQnt(0);

    pset_s(matrix, cqnt, "keys", 0b11100);
    pset_s(matrix, cqnt, "omin", 1);
    pset_s(matrix, cqnt, "omax", 1);

    let mut v = vec![];
    for i in 0..20 {
        let x = i as f32 / 20.0;

        pset_n_wait(matrix, node_exec, cqnt, "inp", x);
        let res = run_for_ms(node_exec, 1.0);
        v.push(res.0[0]);
    }

    assert_vec_feq!(
        v,
        vec![
            -0.1583, -0.1583, -0.1583, -0.15, -0.15, -0.1416, -0.1416, -0.0583, -0.0583, -0.05,
            -0.05, -0.05, -0.0416, -0.0416, 0.0416, 0.0416, 0.0499, 0.0499, 0.0583, 0.0583
        ]
    );
}

#[test]
fn check_node_cqnt_min_max_oct_trig_out() {
    init_test!(matrix, node_exec, 3);
    setup_cqnt(matrix, true);

    let cqnt = NodeId::CQnt(0);

    pset_s(matrix, cqnt, "keys", 0b11100);
    pset_s(matrix, cqnt, "omin", 1);
    pset_s(matrix, cqnt, "omax", 1);

    let mut v = vec![];
    for i in 0..100 {
        let x = i as f32 / 100.0;

        pset_n(matrix, cqnt, "inp", x);
        let res = run_for_ms(node_exec, 10.0);
        v.extend_from_slice(&res.0[..]);
    }

    let idxs_big = collect_signal_changes(&v[..], 50);

    assert_eq!(
        idxs_big,
        vec![
            (0, 100),
            (5341, 100),
            (10240, 100),
            (15140, 100),
            (20041, 100),
            (24941, 100),
            (29841, 100),
            (34740, 100),
            (39640, 100)
        ]
    );
}
