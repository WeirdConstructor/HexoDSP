// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
// Tests gate input and default values for the node
fn check_node_adsr_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("adsr", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    node_pset_n(&mut matrix, "adsr", 0, "gate", 1.0);
    let res = run_for_ms(&mut node_exec, 40.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.007558584,
            0.007558584,
            0.007558584,
            // 44.1 per ms, decay is default 10.0ms (roughly 9 * 50 samples):
            -0.0011337,
            -0.0011337,
            -0.0011337,
            -0.0011337,
            -0.0011337,
            -0.0011337,
            -0.0011337,
            -0.0011337,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );

    node_pset_n(&mut matrix, "adsr", 0, "gate", 0.0);
    let res = run_for_ms(&mut node_exec, 50.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            // 44.1 per ms, decay is default 40.0ms (roughly 35 * 50 samples):
            -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834,
            -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834,
            -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834,
            -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834,
            -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834, -0.0002834,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ]
    );
}

#[test]
// Tests attack decay and release value adjustment.
fn check_node_adsr_2() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("adsr", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    node_pset_d(&mut matrix, "adsr", 0, "atk", 5.0);
    node_pset_d(&mut matrix, "adsr", 0, "dcy", 5.0);
    node_pset_d(&mut matrix, "adsr", 0, "rel", 5.0);
    node_pset_d(&mut matrix, "adsr", 0, "sus", 0.2);
    wait_params_smooth(&mut node_exec);
    node_pset_n(&mut matrix, "adsr", 0, "gate", 1.0);
    let res = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.0045351535,
            0.0045351535,
            0.0045351535,
            0.0045351535,
            // 44.1 per ms, decay is default 10.0ms (roughly 9 * 50 samples):
            -0.0036280751,
            -0.0036280751,
            -0.0036280751,
            -0.0036280751,
            -0.0036280751,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );

    node_pset_n(&mut matrix, "adsr", 0, "gate", 0.0);
    let res = run_for_ms(&mut node_exec, 50.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, decay is default 40.0ms (roughly 35 * 50 samples):
            -0.0009070337,
            -0.0009070337,
            -0.0009070337,
            -0.0009070337,
            -0.00045377004,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ]
    );
}

#[test]
// Tests attack decay and release value adjustment.
fn check_node_adsr_mult() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("adsr", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    node_pset_d(&mut matrix, "adsr", 0, "atk", 0.5);
    node_pset_d(&mut matrix, "adsr", 0, "dcy", 0.5);
    node_pset_d(&mut matrix, "adsr", 0, "rel", 0.5);
    node_pset_d(&mut matrix, "adsr", 0, "sus", 0.2);
    node_pset_s(&mut matrix, "adsr", 0, "mult", 1);
    wait_params_smooth(&mut node_exec);
    node_pset_n(&mut matrix, "adsr", 0, "gate", 1.0);
    let res = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.0045351535,
            0.0045351535,
            0.0045351535,
            0.0045351535,
            // 44.1 per ms, decay is default 10.0ms (roughly 9 * 50 samples):
            -0.0036280751,
            -0.0036280751,
            -0.0036280751,
            -0.0036280751,
            -0.0036280751,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );

    node_pset_n(&mut matrix, "adsr", 0, "gate", 0.0);
    let res = run_for_ms(&mut node_exec, 50.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, decay is default 40.0ms (roughly 35 * 50 samples):
            -0.0009070337,
            -0.0009070337,
            -0.0009070337,
            -0.0009070337,
            -0.00045377004,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ]
    );
}

#[test]
// Tests attack decay and release shape adjustment.
fn check_node_adsr_3() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("adsr", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    node_pset_d(&mut matrix, "adsr", 0, "atk", 10.0);
    node_pset_d(&mut matrix, "adsr", 0, "dcy", 10.0);
    node_pset_d(&mut matrix, "adsr", 0, "rel", 10.0);
    node_pset_n(&mut matrix, "adsr", 0, "ashp", 0.0);
    node_pset_n(&mut matrix, "adsr", 0, "dshp", 1.0);
    node_pset_n(&mut matrix, "adsr", 0, "rshp", 0.25);
    node_pset_d(&mut matrix, "adsr", 0, "sus", 0.2);
    wait_params_smooth(&mut node_exec);
    node_pset_n(&mut matrix, "adsr", 0, "gate", 1.0);
    let res = run_for_ms(&mut node_exec, 40.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.00000336,
            0.00000572,
            0.00024048518,
            0.0006323159,
            0.0013120994,
            0.0023591071,
            0.0038526654,
            0.0058722496,
            0.008496821,
            // 44.1 per ms, decay is default 10.0ms (roughly 9 * 50 samples):
            -0.00000554,
            -0.000062704,
            -0.00023466349,
            -0.0005848408,
            -0.0011769533,
            -0.0020741224,
            -0.0033401847,
            -0.005038202,
            -0.007215932,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );

    node_pset_n(&mut matrix, "adsr", 0, "gate", 0.0);
    let res = run_for_ms(&mut node_exec, 50.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, decay is default 40.0ms (roughly 35 * 50 samples):
            -0.0010517985,
            -0.0005671382,
            -0.0004337877,
            -0.00036469102,
            -0.000320673,
            -0.0002895333,
            -0.00026599318,
            -0.00024739467,
            -0.0002322197,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ]
    );
}

#[test]
// Tests attack decay and release shape adjustment.
fn check_node_adsr_eoet() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("adsr", "eoet").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    node_pset_d(&mut matrix, "adsr", 0, "atk", 10.0);
    node_pset_d(&mut matrix, "adsr", 0, "dcy", 10.0);
    node_pset_d(&mut matrix, "adsr", 0, "rel", 10.0);
    node_pset_d(&mut matrix, "adsr", 0, "sus", 0.2);
    wait_params_smooth(&mut node_exec);
    node_pset_n(&mut matrix, "adsr", 0, "gate", 1.0);
    let res = run_for_ms(&mut node_exec, 40.0);
    assert_decimated_feq!(res.0, 50, vec![0.0; 100]);

    node_pset_n(&mut matrix, "adsr", 0, "gate", 0.0);
    let res = run_for_ms(&mut node_exec, 50.0);
    assert_decimated_feq!(
        res.0,
        50,
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            1.0, // end of envelope trigger!
            1.0, // end of envelope trigger!
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ]
    );
}
