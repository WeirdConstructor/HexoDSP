// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup() -> (Matrix, NodeExecutor) {
    let (node_conf, node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("code", "sig1")
        .set_denorm("in1", 0.5)
        .set_denorm("in2", -0.6)
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    (matrix, node_exec)
}

#[test]
fn check_node_code_1() {
    let (mut matrix, mut node_exec) = setup();

    let block_fun = matrix.get_block_function(0).expect("block fun exists");
    {
        let mut block_fun = block_fun.lock().expect("matrix lock");
        block_fun.instanciate_at(0, 0, 1, "value", Some("0.3".to_string()));
        block_fun.instanciate_at(0, 1, 1, "set", Some("&sig1".to_string()));
    }

    matrix.check_block_function(0).expect("no compile error");

    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_feq!(res.0, 50, vec![0.3; 10]);
}

