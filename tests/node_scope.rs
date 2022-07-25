// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

use hexodsp::nodes::SCOPE_SAMPLES;

#[test]
fn check_node_scope_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_inp("scope", "in1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    let scope = NodeId::Scope(0);
    let in1_p = scope.inp_param("in1").unwrap();
    let in2_p = scope.inp_param("in2").unwrap();
    let in3_p = scope.inp_param("in3").unwrap();

    matrix.set_param(in1_p, SAtom::param(1.0));
    matrix.set_param(in2_p, SAtom::param(1.0));
    matrix.set_param(in3_p, SAtom::param(1.0));
    let _res = run_for_ms(&mut node_exec, 11.0);

    let scope = matrix.get_scope_buffers(0).unwrap();
    let mut v = vec![];
    for x in 0..SCOPE_SAMPLES {
        v.push(scope[0].read(x));
    }
    assert_decimated_feq!(v, 80, vec![0.0022, 0.1836, 0.3650, 0.5464, 0.7278, 0.9093, 1.0]);

    let mut v = vec![];
    for x in 0..SCOPE_SAMPLES {
        v.push(scope[1].read(x));
    }
    assert_decimated_feq!(v, 80, vec![0.0022, 0.1836, 0.3650, 0.5464, 0.7278, 0.9093, 1.0]);

    let mut v = vec![];
    for x in 0..SCOPE_SAMPLES {
        v.push(scope[2].read(x));
    }
    assert_decimated_feq!(v, 80, vec![0.0022, 0.1836, 0.3650, 0.5464, 0.7278, 0.9093, 1.0]);
}
