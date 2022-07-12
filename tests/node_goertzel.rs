// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

//WRITEME: setup sin into gzfilt, assert that gzfilt at matched freq has const output, and at unmatched has no output
fn setup_gnode_matrix() -> (Matrix, NodeExecutor) {
    let (node_conf, node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let goertzel = NodeId::GzFilt(0);
    let sin    = NodeId::Sin(0);
    let out   = NodeId::Out(0);
    //FIXME: link properly sin880 -> goertzel tuned to 880 -> out
    matrix.place(0, 0, Cell::empty(sin)
                       .input(None, None, None)
                       .out(None, None, sf.out("sig")));
    matrix.place(1, 1, Cell::empty(goertzel)
                       .out(goertzel.out("sig"), None, None);
    matrix.place(1, 2, Cell::empty(out)
                       .input(out.inp("sig"), None, None));
    matrix.sync().unwrap();

    (matrix, node_exec)
}

//WRITEME: expect signal to be > 0.2 for 880, change goertzel param to 600 target -> expect it to be < 0.2
#[test]
fn check_node_goertzel() {
    let (matrix, node_exec) = setup_gnode_matrix();

    let (out_l, _) = run_for_ms(&mut node_exec, 300.0);
    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 25.0);

}

