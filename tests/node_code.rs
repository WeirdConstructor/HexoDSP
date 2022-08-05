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

#[test]
fn check_node_code_state() {
    let (mut matrix, mut node_exec) = setup();

    let block_fun = matrix.get_block_function(0).expect("block fun exists");
    {
        let mut block_fun = block_fun.lock().expect("matrix lock");
        block_fun.instanciate_at(0, 0, 2, "value", Some("220.0".to_string()));
        block_fun.instanciate_at(0, 1, 2, "phase", None);
        block_fun.instanciate_at(0, 1, 3, "value", Some("2.0".to_string()));
        block_fun.instanciate_at(0, 2, 2, "*", None);
        block_fun.instanciate_at(0, 3, 1, "-", None);
        block_fun.instanciate_at(0, 2, 1, "value", Some("1.0".to_string()));
        block_fun.instanciate_at(0, 4, 1, "set", Some("&sig1".to_string()));
    }

    matrix.check_block_function(0).expect("no compile error");

    let fft = run_and_get_fft4096_now(&mut node_exec, 50);
    // Aliasing sawtooth I expect:
    assert_eq!(
        fft,
        vec![
            (205, 133),
            (215, 576),
            (226, 527),
            (237, 90),
            (431, 195),
            (441, 322),
            (452, 131),
            (646, 61),
            (657, 204),
            (668, 157),
            (872, 113),
            (883, 155),
            (894, 51),
            (1098, 127),
            (1109, 82),
            (1314, 85),
            (1324, 98),
            (1540, 93),
            (1755, 70),
            (1766, 67),
            (1981, 72),
            (2196, 60),
            (2422, 57),
            (2638, 52)
        ]
    );
}
