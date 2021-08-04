// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_bosc_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let bosc  = NodeId::BOsc(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(bosc)
                       .out(None, None, bosc.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));
    pset_s(&mut matrix, bosc, "wtype", 0); // Sine
    pset_d(&mut matrix, bosc, "freq", 220.0);
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 10.0);
    let fft = run_and_get_fft4096_now(&mut node_exec, 20);
    assert_eq!(fft[0].0, 194);
    assert_eq!(fft[1].0, 205);
    assert_eq!(fft[2].0, 215);
    assert_eq!(fft[3].0, 226);
    assert_eq!(fft[4].0, 237);
    assert_eq!(fft[5].0, 248);

    pset_s(&mut matrix, bosc, "wtype", 1); // Triangle
    let fft = run_and_get_fft4096_now(&mut node_exec, 20);
    assert_eq!(fft[0].0, 194);
    assert_eq!(fft[1].0, 205);
    assert_eq!(fft[2].0, 215);
    assert_eq!(fft[3].0, 226);
    assert_eq!(fft[4].0, 237);
    assert_eq!(fft[5].0, 646);
    assert_eq!(fft[6].0, 657);
    assert_eq!(fft[7].0, 668);
    assert_eq!(fft[8].0, 1098);
    assert_eq!(fft[9].0, 1109);

    pset_s(&mut matrix, bosc, "wtype", 2); // Saw
    let fft = run_and_get_fft4096_now(&mut node_exec, 120);
    assert_eq!(fft[0].0, 205);
    assert_eq!(fft[1].0, 215);
    assert_eq!(fft[2].0, 226);
    assert_eq!(fft[3].0, 431);
    assert_eq!(fft[4].0, 441);
    assert_eq!(fft[5].0, 452);
    assert_eq!(fft[6].0, 657);
    assert_eq!(fft[7].0, 668);
    assert_eq!(fft[8].0, 883);
    assert_eq!(fft[9].0, 1098);

    pset_s(&mut matrix, bosc, "wtype", 3); // Pulse
    pset_n(&mut matrix, bosc, "pw", 0.0);  // Pulse width no mod
    run_for_ms(&mut node_exec, 10.0);
    let fft = run_and_get_fft4096_now(&mut node_exec, 120);
    assert_eq!(fft[0].0, 205);
    assert_eq!(fft[1].0, 215);
    assert_eq!(fft[2].0, 226);
    assert_eq!(fft[3].0, 237);
    assert_eq!(fft[4].0, 646);
    assert_eq!(fft[5].0, 657);
    assert_eq!(fft[6].0, 668);
    assert_eq!(fft[7].0, 1098);
    assert_eq!(fft[8].0, 1109);
    assert_eq!(fft[9].0, 1540);
    assert_eq!(fft[10].0, 1981);

    pset_n(&mut matrix, bosc, "pw", 1.0);  // Pulse width no mod
    run_for_ms(&mut node_exec, 10.0);
    let fft = run_and_get_fft4096_now(&mut node_exec, 120);
    assert_eq!(fft[0].0, 215);
    assert_eq!(fft[1].0, 226);
    assert_eq!(fft[2].0, 431);
    assert_eq!(fft[3].0, 441);
    assert_eq!(fft[4].0, 452);
    assert_eq!(fft[5].0, 657);
    assert_eq!(fft[6].0, 668);
    assert_eq!(fft[7].0, 872);
    assert_eq!(fft[8].0, 883);
    assert_eq!(fft[9].0, 1098);
    assert_eq!(fft[10].0, 1109);
    assert_eq!(fft[11].0, 1314);
    assert_eq!(fft[12].0, 1324);
    assert_eq!(fft[13].0, 1540);
}

#[test]
fn check_node_bosc_det() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let bosc  = NodeId::BOsc(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(bosc)
                       .out(None, None, bosc.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));
    pset_s(&mut matrix, bosc, "wtype", 3); // Pulse
    pset_n(&mut matrix, bosc, "pw", 0.0);  // Pulse width no mod
    pset_d(&mut matrix, bosc, "freq", 220.0);
    matrix.sync().unwrap();

    let fft = run_and_get_fft4096_now(&mut node_exec, 120);
//    println!("TO {:?}", fft);
    assert_eq!(fft[0].0, 205);
    assert_eq!(fft[1].0, 215);
    assert_eq!(fft[2].0, 226);
    assert_eq!(fft[3].0, 237);
    assert_eq!(fft[4].0, 646);
    assert_eq!(fft[5].0, 657);
    assert_eq!(fft[6].0, 668);
    assert_eq!(fft[7].0, 1098);
    assert_eq!(fft[8].0, 1109);
    assert_eq!(fft[9].0, 1540);
    assert_eq!(fft[10].0, 1981);

    pset_d_wait(&mut matrix, &mut node_exec, bosc, "freq", 1000.0);
    let fft = run_and_get_fft4096_now(&mut node_exec, 200);
//    println!("TO {:?}", fft);
    assert_eq!(fft[0].0, 991);
    assert_eq!(fft[1].0, 1001);
    assert_eq!(fft[2].0, 1012);
    assert_eq!(fft[3].0, 2993);
    assert_eq!(fft[4].0, 3004);
    assert_eq!(fft[5].0, 4996);

    pset_n_wait(&mut matrix, &mut node_exec, bosc, "det", 0.1);
    let fft = run_and_get_fft4096_now(&mut node_exec, 200);
//    println!("TO {:?}", fft);
    assert_eq!(fft[0].0, 1992);
    assert_eq!(fft[1].0, 2003);
    assert_eq!(fft[2].0, 2013);
    assert_eq!(fft[3].0, 5997);
    assert_eq!(fft[4].0, 6008);
    assert_eq!(fft[5].0, 10002);
}
