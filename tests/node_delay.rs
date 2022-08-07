// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_delay_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let ad = NodeId::Ad(0);
    let sin = NodeId::Sin(0);
    let dly = NodeId::Delay(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(ad).input(ad.inp("inp"), None, None).out(None, None, ad.out("sig")),
    );
    matrix.place(
        0,
        2,
        Cell::empty(dly).input(dly.inp("inp"), None, None).out(None, None, dly.out("sig")),
    );
    matrix.place(0, 3, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));
    matrix.sync().unwrap();

    pset_d(&mut matrix, ad, "atk", 50.0);
    pset_d(&mut matrix, ad, "dcy", 50.0);
    pset_n(&mut matrix, ad, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 500.0);
    // 441 decimation => 10ms resolution
    assert_decimated_feq!(
        res.0,
        441,
        vec![
            // smoothing time:
            0.0,
            // burst of sine for 100ms:
            0.04741215,
            -0.17181772,
            0.2669262,
            -0.22376089,
            0.000030220208,
            0.24654882,
            -0.30384964,
            0.20876096,
            -0.070250794,
            0.0000024548233,
            // 150ms silence:
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // delayed burst of sine for 100ms:
            0.05125899,
            -0.17475566,
            0.2607654,
            -0.20392825,
            -0.03003881,
            0.26745066,
            -0.30965388,
            0.20431,
            -0.064184606,
            -0.0012322,
            // silence afterwards:
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );
}

#[test]
fn check_node_delay_2() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let dly = NodeId::Delay(0);
    let out = NodeId::Out(0);
    matrix.place(0, 2, Cell::empty(dly).out(None, None, dly.out("sig")));
    matrix.place(0, 3, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));
    matrix.sync().unwrap();

    pset_d(&mut matrix, dly, "time", 31.0);
    pset_d(&mut matrix, dly, "inp", 1.0);

    let res = run_for_ms(&mut node_exec, 150.0);
    // 441 decimation => 10ms resolution
    assert_decimated_feq!(
        res.0,
        441,
        vec![
            // 10ms smoothing time for "inp"
            0.001133, // 30ms delaytime just mixing the 0.5:
            0.5,
            0.5,
            0.5,         // the delayed smoothing ramp (10ms):
            0.950001113, // the delay + input signal:
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0
        ]
    );
}

#[test]
fn check_node_delay_time_mod() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let sin = NodeId::Sin(1);
    let dly = NodeId::Delay(0);
    let out = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        1,
        2,
        Cell::empty(dly).input(dly.inp("inp"), None, dly.inp("time")).out(
            None,
            None,
            dly.out("sig"),
        ),
    );
    matrix.place(1, 3, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, dly, "mix", 1.0);
    pset_d(&mut matrix, dly, "time", 100.0);

    // skip delay time:
    run_for_ms(&mut node_exec, 100.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 600);
    assert_eq!(fft[0], (431, 614));
    assert_eq!(fft[1], (441, 1012));

    let sin2 = NodeId::Sin(0);
    matrix.place(0, 3, Cell::empty(sin2).out(sin2.out("sig"), None, None));

    matrix.sync().unwrap();
    pset_d(&mut matrix, sin2, "freq", 0.5);

    // let everything settle down and the delay buffer fill with stuff:
    run_for_ms(&mut node_exec, 5000.0);

    // skip some time to let everything settle:
    run_for_ms(&mut node_exec, 670.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 110);
    // Expect a sine sweep over a
    // range of low frequencies:
    assert_eq!(fft[0], (97, 113));
    assert_eq!(fft[5], (312, 114));
    assert_eq!(fft[10], (635, 110));

    // Sweep upwards:
    run_for_ms(&mut node_exec, 300.0);
    let fft = run_and_get_fft4096_now(&mut node_exec, 122);
    assert_eq!(fft[0], (2498, 122));
    assert_eq!(fft[7], (2681, 122));

    // Sweep at mostly highest point:
    run_for_ms(&mut node_exec, 700.0);
    let fft = run_and_get_fft4096_now(&mut node_exec, 300);
    assert_eq!(fft[0], (6417, 309));
    assert_eq!(fft[4], (6471, 407));
}

#[test]
fn check_node_delay_trig() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let dly = NodeId::Delay(0);
    let out = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test).out(None, None, test.out("tsig")));
    matrix.place(0, 3, Cell::empty(test).out(test.out("sig"), None, None));
    matrix.place(
        1,
        2,
        Cell::empty(dly).input(dly.inp("inp"), None, dly.inp("trig")).out(
            None,
            None,
            dly.out("sig"),
        ),
    );
    matrix.place(1, 3, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, dly, "mix", 1.0);
    pset_n(&mut matrix, dly, "mode", 1.0);
    pset_d(&mut matrix, dly, "time", 5.0);

    // Trigger the delay 2 times, with an interval of 20ms:
    pset_n(&mut matrix, test, "p", 1.0);
    run_for_ms(&mut node_exec, 10.0);
    pset_n(&mut matrix, test, "p", 0.0);
    run_for_ms(&mut node_exec, 10.0);
    pset_n(&mut matrix, test, "p", 1.0);
    run_for_ms(&mut node_exec, 10.0);
    pset_n(&mut matrix, test, "p", 0.0);
    run_for_ms(&mut node_exec, 10.0);

    // Now the delay should have a 20ms delay time.

    // Emit the trigger signal:
    pset_n(&mut matrix, test, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 30.0);

    let mut idx_first_non_zero = 99999;
    for i in 0..res.0.len() {
        if res.0[i] > 0.0 {
            idx_first_non_zero = i;
            break;
        }
    }

    // We expect the signal to be delayed by 20ms:
    assert_eq!(idx_first_non_zero, (44100 * 20) / 1000 + 1);
}

#[test]
fn check_node_delay_fb() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let dly = NodeId::Delay(0);
    let out = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test).out(None, None, test.out("tsig")));
    matrix.place(
        1,
        2,
        Cell::empty(dly).input(dly.inp("inp"), None, None).out(None, None, dly.out("sig")),
    );
    matrix.place(1, 3, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));

    pset_n(&mut matrix, dly, "mix", 1.0);
    pset_d(&mut matrix, dly, "time", 5.0);
    pset_n(&mut matrix, dly, "fb", 0.5);

    matrix.sync().unwrap();

    // Emit the trigger signal:
    pset_n(&mut matrix, test, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 100.0);

    let idxs_big = collect_signal_changes(&res.0[..], 50);

    // We expect the signal to be delayed by 20ms:
    assert_eq!(idxs_big, vec![(222, 106), (444, 53)]);
}

#[test]
fn check_node_delay_fb_neg() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let dly = NodeId::Delay(0);
    let out = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test).out(None, None, test.out("tsig")));
    matrix.place(
        1,
        2,
        Cell::empty(dly).input(dly.inp("inp"), None, None).out(None, None, dly.out("sig")),
    );
    matrix.place(1, 3, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));

    pset_n(&mut matrix, dly, "mix", 1.0);
    pset_d(&mut matrix, dly, "time", 10.0);
    pset_n(&mut matrix, dly, "fb", -1.0);

    matrix.sync().unwrap();

    // Emit the trigger signal:
    pset_n(&mut matrix, test, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 40.0);

    let idxs_big = collect_signal_changes(&res.0[..], 70);

    assert_eq!(idxs_big, vec![(442, 100), (884, -100), (1326, 100)]);
}

#[test]
fn check_node_delay_fb_pos() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let dly = NodeId::Delay(0);
    let out = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test).out(None, None, test.out("tsig")));
    matrix.place(
        1,
        2,
        Cell::empty(dly).input(dly.inp("inp"), None, None).out(None, None, dly.out("sig")),
    );
    matrix.place(1, 3, Cell::empty(out).input(out.inp("ch1"), None, None).out(None, None, None));

    pset_n(&mut matrix, dly, "mix", 1.0);
    pset_d(&mut matrix, dly, "time", 10.0);
    pset_n(&mut matrix, dly, "fb", 1.0);

    matrix.sync().unwrap();

    // Emit the trigger signal:
    pset_n(&mut matrix, test, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 100.0);

    let idxs_big = collect_signal_changes(&res.0[..], 70);

    assert_eq!(
        idxs_big,
        vec![
            (442, 100),
            (442 + 1 * 442, 100),
            (442 + 2 * 442, 100),
            (442 + 3 * 442, 100),
            (442 + 4 * 442, 100),
            (442 + 5 * 442, 100),
            (442 + 6 * 442, 100),
            (442 + 7 * 442, 100),
            (442 + 8 * 442, 100),
        ]
    );
}
