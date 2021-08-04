// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_noise_bipolar() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_n(&mut matrix, noise, "atv", 1.0);
    matrix.sync().unwrap();

    let (out_l, _) = run_for_ms(&mut node_exec, 25.0);

    assert_float_eq!(out_l[0],   0.1545);
    assert_float_eq!(out_l[10],  0.5924);
    assert_float_eq!(out_l[11], -0.3643);

    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 10.0);
    assert_rmsmima!(rms_mimax[0], (0.3374, -0.9958, 0.9971));
    assert_rmsmima!(rms_mimax[1], (0.3384, -0.9997, 0.9993));
}

#[test]
fn check_node_noise_seed() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let nois2 = NodeId::Noise(1);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.place(1, 0, Cell::empty(nois2)
                       .out(None, None, nois2.out("sig")));
    matrix.place(1, 1, Cell::empty(out)
                       .input(out.inp("ch2"), None, None));
    pset_n(&mut matrix, noise, "atv", 1.0);
    pset_n(&mut matrix, nois2, "atv", 1.0);
    matrix.sync().unwrap();

    let (out_l, out_r) = run_for_ms(&mut node_exec, 25.0);

    assert_float_eq!(out_l[0],   0.1545);
    assert_float_eq!(out_l[10],  0.5924);

    assert_float_eq!(out_r[0],  -0.2156);
    assert_float_eq!(out_r[10],  0.9441);
}

#[test]
fn check_node_noise_unipolar() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_s(&mut matrix, noise, "mode", 1);
    pset_n(&mut matrix, noise, "atv", 1.0);
    matrix.sync().unwrap();

    let (out_l, _) = run_for_ms(&mut node_exec, 25.0);

    assert_float_eq!(out_l[0],   0.5772);
    assert_float_eq!(out_l[10],  0.7962);
    assert_float_eq!(out_l[11],  0.3178);

    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 10.0);
    assert_rmsmima!(rms_mimax[0], (0.3214, 0.002, 0.9985));
    assert_rmsmima!(rms_mimax[1], (0.3373, 0.0001, 0.9996));
}


#[test]
fn check_node_noise_atv_offs() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_s(&mut matrix, noise, "mode", 1);
    pset_n(&mut matrix, noise, "atv", 0.5);
    pset_n(&mut matrix, noise, "offs", 0.3);
    matrix.sync().unwrap();

    let (out_l, _) = run_for_ms(&mut node_exec, 100.0);
    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 100.0);
    println!("mima {:?}", rms_mimax);
    assert_rmsmima!(rms_mimax[0], (0.3223, 0.3, 0.7998));
}

#[test]
fn check_node_noise_atv_offs_2() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_s(&mut matrix, noise, "mode", 1);
    pset_n(&mut matrix, noise, "atv", -0.5);
    pset_n(&mut matrix, noise, "offs", -0.4);
    matrix.sync().unwrap();

    let (out_l, _) = run_for_ms(&mut node_exec, 100.0);
    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 100.0);
    println!("mima {:?}", rms_mimax);
    assert_rmsmima!(rms_mimax[0], (0.4422, -0.8998, -0.4));
}

#[test]
fn check_node_noise_atv_offs_bipolar() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_s(&mut matrix, noise, "mode", 0);
    pset_n(&mut matrix, noise, "atv", 0.5);
    pset_n(&mut matrix, noise, "offs", 0.4);
    matrix.sync().unwrap();

    let (out_l, _) = run_for_ms(&mut node_exec, 100.0);
    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 100.0);
    println!("mima {:?}", rms_mimax);
    assert_rmsmima!(rms_mimax[0], (0.2407, -0.0998, 0.8996));
}

#[test]
fn check_node_noise_fft() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_s(&mut matrix, noise, "mode", 0);
    pset_n(&mut matrix, noise, "atv", 1.0);
    pset_n(&mut matrix, noise, "offs", 0.0);
    matrix.sync().unwrap();

    let fft = run_and_get_fft4096(&mut node_exec, 50, 1000.0);
    assert!(fft.len() > 15);
    for (_freq, lvl) in fft {
        assert_float_eq!(
            (((lvl as i64 - 57) as f32).abs() / 10.0).floor(),
            0.0);
    }
}
