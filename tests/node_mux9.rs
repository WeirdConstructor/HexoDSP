// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup_mux9_slct(matrix: &mut Matrix) {
    let amp  = NodeId::Amp(0);
    let mux9 = NodeId::Mux9(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(amp)
                       .out(None, None, amp.out("sig")));
    matrix.place(0, 1, Cell::empty(mux9)
                       .input(mux9.inp("slct"), None, None)
                       .out(None, None, mux9.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    for i in 1..=9 {
        pset_n(matrix, mux9, &format!("in_{}", i), 0.01 * (i as f32));
    }

    matrix.sync().unwrap();
}

fn setup_mux9(matrix: &mut Matrix) {
    let mux9 = NodeId::Mux9(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(mux9)
                       .out(None, None, mux9.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    for i in 1..=9 {
        pset_n(matrix, mux9, &format!("in_{}", i), 0.01 * (i as f32));
    }

    matrix.sync().unwrap();
}

#[test]
fn check_node_mux9_9() {
    init_test!(matrix, node_exec, 3);
    setup_mux9(matrix);

    let mux9 = NodeId::Mux9(0);

    pset_s(matrix, mux9, "in_cnt", 9.into());

    let (out, _) = run_for_ms(node_exec, 1.0);
    assert_float_eq!(out[0], 0.01);

    for i in 2..=9 {
        pset_n_wait(matrix, node_exec, mux9, "t_up", 0.0);
        pset_n_wait(matrix, node_exec, mux9, "t_up", 1.0);
        let (out, _) = run_for_ms(node_exec, 1.0);
        assert_float_eq!(out[0], 0.01 * (i as f32));
    }
}

#[test]
fn check_node_mux9_limit() {
    init_test!(matrix, node_exec, 3);
    setup_mux9(matrix);

    let mux9 = NodeId::Mux9(0);

    pset_s(matrix, mux9, "in_cnt", 2.into());

    let (out, _) = run_for_ms(node_exec, 1.0);
    assert_float_eq!(out[0], 0.01);

    for i in 2..=3 {
        pset_n_wait(matrix, node_exec, mux9, "t_up", 0.0);
        pset_n_wait(matrix, node_exec, mux9, "t_up", 1.0);
        let (out, _) = run_for_ms(node_exec, 1.0);
        assert_float_eq!(out[0], 0.01 * (i as f32));
    }

    for i in 1..=3 {
        pset_n_wait(matrix, node_exec, mux9, "t_up", 0.0);
        pset_n_wait(matrix, node_exec, mux9, "t_up", 1.0);
        let (out, _) = run_for_ms(node_exec, 1.0);
        assert_float_eq!(out[0], 0.01 * (i as f32));
    }
}

#[test]
fn check_node_mux9_reset() {
    init_test!(matrix, node_exec, 3);
    setup_mux9(matrix);

    let mux9 = NodeId::Mux9(0);

    pset_s(matrix, mux9, "in_cnt", 5.into());

    let (out, _) = run_for_ms(node_exec, 1.0);
    assert_float_eq!(out[0], 0.01);

    for i in 2..=3 {
        pset_n_wait(matrix, node_exec, mux9, "t_up", 0.0);
        pset_n_wait(matrix, node_exec, mux9, "t_up", 1.0);
        let (out, _) = run_for_ms(node_exec, 1.0);
        assert_float_eq!(out[0], 0.01 * (i as f32));
    }

    pset_n_wait(matrix, node_exec, mux9, "t_rst", 1.0);
    pset_n_wait(matrix, node_exec, mux9, "t_rst", 0.0);

    for i in 1..=3 {
        let (out, _) = run_for_ms(node_exec, 1.0);
        assert_float_eq!(out[0], 0.01 * (i as f32));
        pset_n_wait(matrix, node_exec, mux9, "t_up", 0.0);
        pset_n_wait(matrix, node_exec, mux9, "t_up", 1.0);
    }
}

#[test]
fn check_node_mux9_slct() {
    init_test!(matrix, node_exec, 3);
    setup_mux9_slct(matrix);

    let mux9 = NodeId::Mux9(0);
    let amp  = NodeId::Amp(0);

    pset_s(matrix, mux9, "in_cnt", 5.into());
    pset_n_wait(matrix, node_exec, amp, "inp", 0.9);

    let (out, _) = run_for_ms(node_exec, 1.0);
    assert_float_eq!(out[0], 0.06);

    pset_n_wait(matrix, node_exec, amp, "inp", 0.0);
    let (out, _) = run_for_ms(node_exec, 1.0);
    assert_float_eq!(out[0], 0.01);

    pset_n(matrix, amp, "inp", 1.0);
    let (out, _) = run_for_ms(node_exec, 20.0);
    assert_decimated_feq!(out, 90, vec![
        0.01,
        0.02,
        0.03,
        0.04,
        0.05,
        0.06,
        0.06,
        0.06,
        0.06,
    ]);

    pset_s(matrix, mux9, "in_cnt", 8.into());
    pset_n_wait(matrix, node_exec, amp, "inp", 0.0);

    pset_n(matrix, amp, "inp", 1.0);
    let (out, _) = run_for_ms(node_exec, 20.0);
    assert_decimated_feq!(out, 50, vec![
        0.01,
        0.02,
        0.03,
        0.04,
        0.05,
        0.06,
        0.07,
        0.08,
        0.09,
        0.09,
        0.09,
        0.09,
        0.09,
    ]);
}
