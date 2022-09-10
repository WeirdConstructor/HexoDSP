// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_inp_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("inp", "sig1").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("inp", "sig2").node_inp("out", "ch2").place(&mut matrix, 1, 0).unwrap();
    matrix.sync().unwrap();

    let (out_l, out_r) = node_exec.test_run_input(&[0.44; 512], false, &[]);
    assert!(out_l.len() == 512);
    assert_decimated_feq!(out_l, 1, vec![0.44; 512]);

    assert!(out_r.len() == 512);
    assert_decimated_feq!(out_r, 1, vec![0.44; 512]);
}

#[test]
fn check_node_inp_vol() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("inp", "sig1")
        .set_denorm("vol", 0.5)
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("inp", "sig2")
        .set_denorm("vol", 0.5)
        .node_inp("out", "ch2")
        .place(&mut matrix, 1, 0)
        .unwrap();
    matrix.sync().unwrap();

    let (out_l, out_r) = node_exec.test_run_input(&[0.44; 512], false, &[]);
    assert!(out_l.len() == 512);
    assert_decimated_feq!(out_l, 1, vec![0.22; 512]);

    assert!(out_r.len() == 512);
    assert_decimated_feq!(out_r, 1, vec![0.22; 512]);
}
