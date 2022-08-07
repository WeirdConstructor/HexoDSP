// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

use hexodsp::nodes::SCOPE_SAMPLES;

fn read_scope_buf(matrix: &Matrix, sig_idx: usize) -> (Vec<f32>, Vec<f32>, f32, f32) {
    let handle = matrix.get_scope_handle(0).unwrap();

    let mut min = vec![];
    let mut max = vec![];
    let mut total_min: f32 = 99999.9;
    let mut total_max: f32 = -99999.9;

    for i in 0..SCOPE_SAMPLES {
        let (ma, mi) = handle.read(sig_idx, i);
        min.push(mi);
        max.push(ma);
        total_min = total_min.min(mi);
        total_max = total_max.max(ma);
    }

    (max, min, total_max, total_min)
}

#[test]
fn check_node_scope_inputs() {
    for (sig_idx, inp_name) in ["in1", "in2", "in3"].iter().enumerate() {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        let mut chain = MatrixCellChain::new(CellDir::B);
        chain
            .node_out("amp", "sig")
            .node_inp("scope", inp_name)
            .set_denorm("time", (1000.0 / 44100.0) * (SCOPE_SAMPLES as f32))
            .place(&mut matrix, 0, 0)
            .unwrap();
        matrix.sync().unwrap();

        node_pset_d(&mut matrix, "amp", 0, "inp", 1.0);
        let _res = run_for_ms(&mut node_exec, 11.0);

        let (minv, maxv, max, min) = read_scope_buf(&matrix, sig_idx);
        // This tests the smoothing ramp that is applied to setting the "inp" of the Amp(0) node:
        assert_decimated_feq!(minv, 80, vec![0.0022, 0.1836, 0.3650, 0.5464, 0.7278, 0.9093, 1.0]);
        assert_decimated_feq!(maxv, 80, vec![0.0022, 0.1836, 0.3650, 0.5464, 0.7278, 0.9093, 1.0]);
        assert_float_eq!(min, 0.0);
        assert_float_eq!(max, 1.0);
    }
}

#[test]
fn check_node_scope_offs_gain_thrsh() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("amp", "sig").node_inp("scope", "in1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    node_pset_d(&mut matrix, "scope", 0, "off1", 0.1);
    node_pset_d(&mut matrix, "scope", 0, "off2", 0.2);
    node_pset_d(&mut matrix, "scope", 0, "off3", 0.3);
    node_pset_d(&mut matrix, "scope", 0, "gain1", 2.0);
    node_pset_d(&mut matrix, "scope", 0, "gain2", 3.0);
    node_pset_d(&mut matrix, "scope", 0, "gain3", 4.0);
    node_pset_d(&mut matrix, "scope", 0, "thrsh", 0.95);
    node_pset_s(&mut matrix, "scope", 0, "tsrc", 1);
    wait_params_smooth(&mut node_exec);

    let handle = matrix.get_scope_handle(0).unwrap();
    let _res = run_for_ms(&mut node_exec, 11.0);

    let thres = handle.get_threshold().unwrap();
    assert_float_eq!(thres, 0.95);

    let (off, gain) = handle.get_offs_gain(0);
    assert_float_eq!(off, 0.1);
    assert_float_eq!(gain, 2.0);

    let (off, gain) = handle.get_offs_gain(1);
    assert_float_eq!(off, 0.2);
    assert_float_eq!(gain, 3.0);

    let (off, gain) = handle.get_offs_gain(2);
    assert_float_eq!(off, 0.3);
    assert_float_eq!(gain, 4.0);
}

#[test]
fn check_node_scope_sine_2hz() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("sin", "sig")
        .set_denorm("freq", 2.0)
        .node_io("amp", "inp", "sig")
        .node_inp("scope", "in1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    wait_params_smooth(&mut node_exec);

    let _res = run_for_ms(&mut node_exec, 1000.0);
    let (maxv, minv, max, min) = read_scope_buf(&matrix, 0);
    // 2 Hz is exactly 2 sine peaks in 1000ms. 1000ms is the default time of the Scope.
    assert_decimated_feq!(
        maxv,
        64,
        vec![0.0264, 1.0, -0.0004, -0.99968, 0.02546, 1.0, -0.0011, -0.9996]
    );
    assert_decimated_feq!(
        minv,
        64,
        vec![0.0016, 0.9996, -0.0249, -1.0, 0.0009, 0.9996, -0.0256, -0.9999]
    );
    assert_float_eq!(max, 1.0);
    assert_float_eq!(min, -1.0);

    // Now change timing to 500ms, so we expect one peak:
    node_pset_d(&mut matrix, "scope", 0, "time", 500.0);
    let _res = run_for_ms(&mut node_exec, 1000.0);

    let (maxv, minv, max, min) = read_scope_buf(&matrix, 0);
    // 2 Hz is exactly 1 sine peaks in 500ms.
    assert_decimated_feq!(maxv, 128, vec![0.1494, 0.9905, -0.1371, -0.9887]);
    assert_decimated_feq!(minv, 128, vec![0.1376, 0.9888, -0.1489, -0.9904]);
    assert_float_eq!(max, 1.0);
    assert_float_eq!(min, -1.0);
}

#[test]
fn check_node_scope_sine_oversampled() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("sin", "sig")
        .set_denorm("freq", 440.0)
        .node_io("amp", "inp", "sig")
        .node_inp("scope", "in1")
        .set_denorm("time", 1.0)
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    wait_params_smooth(&mut node_exec);

    let _res = run_for_ms(&mut node_exec, 1000.0);
    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    assert_decimated_feq!(
        maxv[0..25],
        5,
        // We expect multiple copies of the same sample at the
        // time resolution of 1 millisecond.
        vec![
            0.4506, 0.4506, 0.4506, 0.3938, 0.3938, 0.3354, 0.3354, 0.2150, 0.2150, 0.1534, 0.1534,
            0.1534,
        ]
    );
    // Full wave does not fit into the buffer at 1ms for 512 samples
    assert_float_eq!(max, 0.9996);
    assert_float_eq!(min, -0.5103);
}

#[test]
fn check_node_scope_sine_threshold() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("sin", "sig")
        .set_denorm("freq", 10.0)
        .node_io("amp", "inp", "sig")
        .set_denorm("att", 0.9)
        .node_inp("scope", "in1")
        .set_denorm("time", 100.0)
        .set_atom("tsrc", SAtom::setting(1))
        .set_denorm("thrsh", 1.0)
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    wait_params_smooth(&mut node_exec);

    // Expect a sine that starts at the beginning, because the
    // at the beginning of the Scope state it is basically "triggered"
    // by default. That means it will record one full buffer at startup:
    let _res = run_for_ms(&mut node_exec, 1000.0);

    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    assert_decimated_feq!(maxv[0..35], 5, vec![0.0115, 0.06666, 0.1214, 0.1758]);
    assert_float_eq!(max, 0.8999);
    assert_float_eq!(min, -0.8999);

    // Expect getting a waveform that starts at the top:
    node_pset_d(&mut matrix, "scope", 0, "thrsh", 0.9 - 0.0002);
    wait_params_smooth(&mut node_exec);
    let _res = run_for_ms(&mut node_exec, 1000.0);
    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    // Confirm we are starting at the threshold top:
    assert_decimated_feq!(maxv[0..35], 5, vec![0.8999, 0.8988, 0.8942, 0.8864]);
    assert_float_eq!(max, 0.8999);
    assert_float_eq!(min, -0.8999);

    // Expect frozen waveform:
    node_pset_d(&mut matrix, "scope", 0, "thrsh", 1.0);
    wait_params_smooth(&mut node_exec);
    let _res = run_for_ms(&mut node_exec, 1000.0);

    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    assert_decimated_feq!(maxv[0..35], 5, vec![0.8999, 0.8988, 0.8942, 0.8864]);
    assert_float_eq!(max, 0.8999);
    assert_float_eq!(min, -0.8999);
}

#[test]
fn check_node_scope_sine_ext_trig() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("sin", "sig")
        .set_denorm("freq", 10.0)
        .node_io("amp", "inp", "sig")
        .set_denorm("att", 0.9)
        .node_inp("scope", "in1")
        .set_denorm("time", 100.0)
        .set_atom("tsrc", SAtom::setting(2))
        .set_denorm("thrsh", 0.0)
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    wait_params_smooth(&mut node_exec);

    // Expect a sine that starts at the beginning, because the
    // at the beginning of the Scope state it is basically "triggered"
    // by default. That means it will record one full buffer at startup:
    let _res = run_for_ms(&mut node_exec, 1000.0);
    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    assert_decimated_feq!(maxv[0..35], 5, vec![0.0115, 0.06666, 0.1214, 0.1758]);
    assert_float_eq!(max, 0.8999);
    assert_float_eq!(min, -0.8999);

    // Expect the buffer to not change:
    let _res = run_for_ms(&mut node_exec, 1000.0);
    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    assert_decimated_feq!(maxv[0..35], 5, vec![0.0115, 0.06666, 0.1214, 0.1758]);
    assert_float_eq!(max, 0.8999);
    assert_float_eq!(min, -0.8999);

    // Apply external trigger and expect the buffer to change:
    node_pset_d(&mut matrix, "scope", 0, "trig", 1.0);
    wait_params_smooth(&mut node_exec);
    let _res = run_for_ms(&mut node_exec, 1000.0);
    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    assert_decimated_feq!(maxv[0..35], 5, vec![0.7325241, 0.7631615, 0.7909356, 0.8157421]);
    assert_float_eq!(max, 0.8999);
    assert_float_eq!(min, -0.8999);

    // Expect the buffer to not change, because the trigger has not reset/changed:
    let _res = run_for_ms(&mut node_exec, 1000.0);
    let (maxv, _minv, max, min) = read_scope_buf(&matrix, 0);
    assert_decimated_feq!(maxv[0..35], 5, vec![0.7325241, 0.7631615, 0.7909356, 0.8157421]);
    assert_float_eq!(max, 0.8999);
    assert_float_eq!(min, -0.8999);
}
