// Copyright (c) 2021-2022 Weird Constructor <weirdconstructor@gmail.com>
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
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, decay is default 40.0ms (roughly 35 * 50 samples):
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
            -0.0002834,
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
            0.0,
            0.0,
            0.0,
            0.0,
        ]
    );
}

