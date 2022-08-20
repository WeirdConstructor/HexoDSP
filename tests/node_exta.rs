// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

struct MyParams {}

impl hexodsp::nodes::ExternalParams for MyParams {
    fn a1(&self) -> f32 {
        0.23
    }
    fn a2(&self) -> f32 {
        0.44
    }
    fn a3(&self) -> f32 {
        -0.33
    }
}

#[test]
fn check_node_exta() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let myparams = std::sync::Arc::new(MyParams {});

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("exta", "sig1").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("exta", "sig3").node_inp("out", "ch2").place(&mut matrix, 1, 0).unwrap();
    matrix.sync().unwrap();

    node_exec.set_external_params(myparams);

    let (ch1, ch2) = node_exec.test_run(0.1, false, &[]);
    assert_decimated_feq!(ch1, 10, vec![0.23; 100]);
    assert_decimated_feq!(ch2, 10, vec![-0.33; 100]);

    node_pset_n(&mut matrix, "exta", 0, "atv1", -1.0);
    node_pset_n(&mut matrix, "exta", 0, "atv3", 0.5);

    let (ch1, ch2) = node_exec.test_run(0.1, false, &[]);
    assert_decimated_feq!(
        ch1,
        80,
        vec![
            0.22895692,
            0.14551038,
            0.062063817,
            -0.021382917,
            -0.10482957,
            -0.18827613,
            -0.23,
            -0.23,
            -0.23,
            -0.23,
            -0.23
        ]
    );
    assert_decimated_feq!(
        ch2,
        80,
        vec![
            -0.32962584,
            -0.29969355,
            -0.26976123,
            -0.23982893,
            -0.20989662,
            -0.17996432,
            -0.165,
            -0.165,
            -0.165,
            -0.165,
            -0.165,
            -0.165,
            -0.165
        ]
    );
}
