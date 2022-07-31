// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_blocklang_1() {
    let (node_conf, mut node_exec) = new_node_engine();
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

    let code = NodeId::Code(0);

    let block_fun = matrix.get_block_function(0).expect("block fun exists");
    {
        let mut block_fun = block_fun.lock().expect("matrix lock");

        block_fun.instanciate_at(0, 0, 0, "get", Some("in1".to_string()));
        block_fun.instanciate_at(0, 0, 1, "value", Some("0.3".to_string()));
        block_fun.instanciate_at(0, 1, 0, "+", None);
        block_fun.instanciate_at(0, 2, 0, "set", Some("&sig1".to_string()));

        block_fun.instanciate_at(0, 3, 0, "get", Some("in1".to_string()));
        block_fun.instanciate_at(0, 3, 1, "get", Some("in2".to_string()));
        block_fun.instanciate_at(0, 4, 0, "-", None);
        block_fun.instanciate_at(0, 5, 0, "->3", None);
        block_fun.instanciate_at(0, 6, 1, "set", Some("*a".to_string()));
        block_fun.instanciate_at(0, 6, 2, "set", Some("x".to_string()));
        block_fun.instanciate_at(0, 6, 0, "->", None);
        block_fun.instanciate_at(0, 7, 0, "->2", None);

        block_fun.instanciate_at(0, 0, 3, "get", Some("in1".to_string()));
        block_fun.instanciate_at(0, 0, 4, "get", Some("in2".to_string()));
        block_fun.instanciate_at(0, 1, 3, "/%", None);
        block_fun.instanciate_at(0, 2, 3, "->", None);
        block_fun.instanciate_at(0, 3, 3, "/%", None);
        block_fun.instanciate_at(0, 4, 3, "set", Some("&sig2".to_string()));
        block_fun.instanciate_at(0, 4, 4, "set", Some("*ap".to_string()));
    }

    matrix.check_block_function(0);

    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_feq!(
        res.0,
        50,
        vec![
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
        ]
    );
}
