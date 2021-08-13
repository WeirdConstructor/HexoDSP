// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_tslfo_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let tsl = NodeId::TsLFO(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(tsl)
                       .out(None, None, tsl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, tsl, "time", 0.1);
    matrix.sync().unwrap();

    // Test shortest time 0.1ms:
    let (out_l, _) = run_for_ms(&mut node_exec, 1.0);
    assert_decimated_feq!(out_l, 1, vec![
        0.0, 0.4535, 0.9070, 0.6394, 0.1859,
        0.2675, 0.7210, 0.8253, 0.37188, 0.0816
    ]);

    // Test 1ms:
    pset_d_wait(&mut matrix, &mut node_exec, tsl, "time", 1.0);
    let (out_l, _) = run_for_ms(&mut node_exec, 5.0);
    assert_decimated_feq!(out_l, 10, vec![
        0.7103, 0.8361,
        0.3826, 0.0708, 0.5244, 0.9779, 0.5685, 0.1150,
        0.3384
    ]);

    // Test 300000.0 ms
    pset_d_wait(&mut matrix, &mut node_exec, tsl, "time", 300000.0);
    let (out_l, _) = run_for_ms(&mut node_exec, 3000.0);
    let ramp_slope = 1.0_f64 / (300.0 * 44100.0);
    let tri_slope = ramp_slope * 2.0;
    assert_float_eq!(
        (out_l[0] - out_l[10000]).abs(),
        (tri_slope * 10000.0) as f32);
    assert_float_eq!(
        (out_l[10000] - out_l[11000]).abs(),
        (tri_slope * 1000.0) as f32);
    assert_decimated_feq!(out_l, 10000, vec![
        0.7566, // => Slope is ~0.0015 per 10000 samples
        0.7551,
        0.7536,
        0.7521,
        0.7506,
        0.7490,
        0.7475,
    ]);
}

#[test]
fn check_node_tslfo_trig_slopes() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let tsl = NodeId::TsLFO(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(tsl)
                       .out(None, None, tsl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    // Test 1ms but at full ramp up and resync/trigger:
    pset_d(&mut matrix, tsl, "rev", 1.0);
    pset_d_wait(&mut matrix, &mut node_exec, tsl, "time", 10.0);
    pset_d_wait(&mut matrix, &mut node_exec, tsl, "trig", 1.0);
    let (out_l, _) = run_for_ms(&mut node_exec, 15.0);
    let ramp_slope = 1.0_f64 / ((10.0 / 1000.0) * 44100.0);
    assert_float_eq!(
        (out_l[0] - out_l[1]).abs(),
        ramp_slope as f32);
    assert_decimated_feq!(out_l, 50, vec![
        0.7513184, 0.8648106, 0.9783028, 0.09079399, 0.20428619, 0.3177784
    ]);

    pset_d_wait(&mut matrix, &mut node_exec, tsl, "trig", 0.0);

    // Test 1ms but at full ramp down and resync/trigger:
    pset_d(&mut matrix, tsl, "rev", 0.0);
    pset_d_wait(&mut matrix, &mut node_exec, tsl, "time", 10.0);
    pset_d_wait(&mut matrix, &mut node_exec, tsl, "trig", 1.0);
    let (out_l, _) = run_for_ms(&mut node_exec, 15.0);
    let ramp_slope = 1.0_f64 / ((10.0 / 1000.0) * 44100.0);
    assert_float_eq!(
        (out_l[0] - out_l[1]).abs(),
        ramp_slope as f32);
    assert_decimated_feq!(out_l, 50, vec![
        0.24945788, 0.13606784, 0.022677781, 0.90938777, 0.7959977
    ]);
}
