// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup_fvafilt_matrix() -> (Matrix, NodeExecutor) {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("noise", "sig")
        .node_io("fvafilt", "inp", "sig")
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    (matrix, node_exec)
}

fn fft_with_freq_res_type(
    matrix: &mut Matrix,
    node_exec: &mut NodeExecutor,
    ftype: i64,
    freq: f32,
    res: f32,
) -> Vec<(u16, u32)> {
    let va = NodeId::FVaFilt(0);
    pset_d(matrix, va, "freq", freq);
    pset_d_wait(matrix, node_exec, va, "res", res);
    pset_s(matrix, va, "ftype", ftype);
    run_and_get_fft4096(node_exec, 0, 1000.0)
}

#[test]
fn check_node_fvafilt_ladder() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);
    println!("START");

    pset_d(&mut matrix, va, "freq", 250.00);
    pset_d(&mut matrix, va, "res", 0.5);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    let out = run_and_get_fft4096(&mut node_exec, 2, 500.0);

    println!("{:#?}", out);
    //    assert!(false);
}

#[test]
fn check_overdriven_dc_svf_bug() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("bosc", "sig")
        .set_atom("wtype", SAtom::setting(3))
        .set_denorm("freq", 440.0)
        .node_io("fvafilt", "inp", "sig")
        .set_norm("drive", 1.0)
        .set_denorm("freq", 14000.0)
        .set_atom("ftype", SAtom::setting(1))
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 2000.0);
    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 100.0);
    println!("{:#?}", rmsmima);
    assert_rmsmima!(rmsmima, (1.0, -1.0, -1.0));
}

#[test]
fn check_overdriven_dc_sallen_key_ok() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("bosc", "sig")
        .set_atom("wtype", SAtom::setting(3))
        .set_denorm("freq", 440.0)
        .node_io("fvafilt", "inp", "sig")
        .set_norm("drive", 1.0)
        .set_denorm("freq", 14000.0)
        .set_atom("ftype", SAtom::setting(2))
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 2000.0);
    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 100.0);
    println!("{:#?}", rmsmima);
    assert_rmsmima!(rmsmima, (0.96078, -1.1445, 1.1434));
}

#[test]
fn check_overdriven_dc_ladder_ok() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("bosc", "sig")
        .set_atom("wtype", SAtom::setting(3))
        .set_denorm("freq", 440.0)
        .node_io("fvafilt", "inp", "sig")
        .set_norm("drive", 1.0)
        .set_denorm("freq", 14000.0)
        .set_atom("ftype", SAtom::setting(0))
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 2000.0);
    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 100.0);
    println!("{:#?}", rmsmima);
    assert_rmsmima!(rmsmima, (0.4004, -0.7787, 0.6732));
}
