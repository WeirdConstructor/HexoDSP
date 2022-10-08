// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup_fvafilt_matrix() -> (Matrix, NodeExecutor) {
    let (node_conf, node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let va = NodeId::FVaFilt(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise).out(None, None, noise.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(va).input(va.inp("inp"), None, None).out(None, None, va.out("sig")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.place(1, 1, Cell::empty(noise).out(None, None, noise.out("sig")));
    matrix.place(1, 2, Cell::empty(out).input(out.inp("ch2"), None, None));
    pset_n(&mut matrix, noise, "atv", 1.0);
//    pset_d(&mut matrix, va, "freq", 17219.88);
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
//    pset_d(&mut matrix, va, "freq", 19000.0);
//    pset_d(&mut matrix, va, "res", 0.0);
//    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 2.0);
    println!("START");

    let out = run_and_get_fft4096(&mut node_exec, 10, 500.0);

    pset_d(&mut matrix, va, "freq", 250.00);
    pset_d(&mut matrix, va, "res", 0.5);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 2.0);

    println!("{:#?}", out);
//    assert!(false);
}
