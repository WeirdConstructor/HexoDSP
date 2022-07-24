// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_allp() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let ap = NodeId::AllP(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(test).out(None, None, test.out("tsig")));
    matrix.place(
        0,
        1,
        Cell::empty(ap).input(ap.inp("inp"), None, None).out(None, None, ap.out("sig")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));
    matrix.place(1, 0, Cell::empty(test).out(None, None, test.out("tsig")));
    matrix.place(1, 1, Cell::empty(out).input(out.inp("ch2"), None, None).out(None, None, None));
    pset_d(&mut matrix, ap, "time", 3.0);
    matrix.sync().unwrap();

    pset_s(&mut matrix, test, "trig", 1);

    let res = run_for_ms(&mut node_exec, 20.0);

    // the original signal on ch2: 2ms trigger up:
    let mut v = vec![1.0; (2.0 * 44.1_f32).ceil() as usize];
    v.append(&mut vec![0.0; (18.0 * 44.1_f32).ceil() as usize]);
    assert_vec_feq!(res.1, v);

    // now signal on ch1 from the allpass:
    // starts with original signal * -0.7
    let mut v = vec![0.7; (2.0 * 44.1_f32).ceil() as usize];
    // silence for 1ms, which is the internal delay of the allpass
    v.append(&mut vec![0.0; (1.0 * 44.1_f32).floor() as usize - 1]);

    // allpass feedback of the original signal for 2ms:
    // XXX: the smearing before and after the allpass is due to the
    // cubic interpolation!
    v.append(&mut vec![-0.03748519, 0.37841395, 0.5260659]);
    v.append(&mut vec![0.51; (2.0 * 44.1_f32).ceil() as usize - 3]);
    // 1ms allpass silence like before:
    v.append(&mut vec![0.54748523, 0.13158606, -0.016065884]);
    v.append(&mut vec![0.0; (1.0 * 44.1_f32).floor() as usize - 4]);

    // 2ms the previous 1.0 * 0.7 fed back into the filter,
    // including even more smearing due to cubic interpolation:
    v.append(&mut vec![
        -0.0019286226,
        0.04086761,
        -0.1813516,
        -0.35157663,
        -0.36315754,
        -0.35664573,
    ]);
    v.append(&mut vec![-0.357; (2.0 * 44.1_f32).floor() as usize - 5]);
    v.append(&mut vec![
        -0.3550714,
        -0.39786762,
        -0.1756484,
        -0.005423375,
        0.006157537,
        -0.00035427228,
    ]);
    v.append(&mut vec![0.0; 10]);

    //d// println!("res={:?}", res.1);
    assert_vec_feq!(res.0, v);
}
