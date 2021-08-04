// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_delay_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let ad   = NodeId::Ad(0);
    let sin  = NodeId::Sin(0);
    let dly  = NodeId::Delay(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(ad)
                       .input(ad.inp("inp"), None, None)
                       .out(None, None, ad.out("sig")));
    matrix.place(0, 2, Cell::empty(dly)
                       .input(dly.inp("inp"), None, None)
                       .out(None, None, dly.out("sig")));
    matrix.place(0, 3, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));
    matrix.sync().unwrap();

    pset_d(&mut matrix, ad, "atk", 50.0);
    pset_d(&mut matrix, ad, "dcy", 50.0);
    pset_n(&mut matrix, ad, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 500.0);
    // 441 decimation => 10ms resolution
    assert_decimated_feq!(res.0, 441, vec![
        // 10ms smoothing time
        0.0,
        // burst of sine for 100ms:
        0.018363932, -0.124816686, 0.21992423, -0.19471036, 0.00002711302,
        0.27546832, -0.35064548, 0.25555965, -0.0991776, 0.000008648983,
        // 150ms silence:
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0,
        // delayed burst of sine for 100ms:
        0.015279313, -0.119179465, 0.22757527, -0.22698581, 0.05398392,
        0.22569486, -0.3332433, 0.26348564, -0.11514694, 0.008539479,
        // silence afterwards:
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
    ]);
}

#[test]
fn check_node_delay_2() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let dly  = NodeId::Delay(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 2, Cell::empty(dly)
                       .out(None, None, dly.out("sig")));
    matrix.place(0, 3, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));
    matrix.sync().unwrap();

    pset_d(&mut matrix, dly, "time", 31.0);
    pset_d(&mut matrix, dly, "inp",  1.0);

    let res = run_for_ms(&mut node_exec, 150.0);
    // 441 decimation => 10ms resolution
    assert_decimated_feq!(res.0, 441, vec![
        // 10ms smoothing time for "inp"
        0.001133,
        // 30ms delaytime just mixing the 0.5:
        0.5, 0.5, 0.5,
        // the delayed smoothing ramp (10ms):
        0.9513,
        // the delay + input signal:
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0
    ]);
}

#[test]
fn check_node_delay_time_mod() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let sin  = NodeId::Sin(1);
    let dly  = NodeId::Delay(0);
    let out  = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(1, 2, Cell::empty(dly)
                       .input(dly.inp("inp"), None, dly.inp("time"))
                       .out(None, None, dly.out("sig")));
    matrix.place(1, 3, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, dly, "mix",  1.0);
    pset_d(&mut matrix, dly, "time", 100.0);

    // skip delay time:
    run_for_ms(&mut node_exec, 100.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 600);
    assert_eq!(fft[0], (431,  614));
    assert_eq!(fft[1], (441, 1012));

    let sin2 = NodeId::Sin(0);
    matrix.place(0, 3, Cell::empty(sin2)
                       .out(sin2.out("sig"), None, None));

    matrix.sync().unwrap();
    pset_d(&mut matrix, sin2, "freq", 0.5);

    // let everything settle down and the delay buffer fill with stuff:
    run_for_ms(&mut node_exec, 5000.0);

    // skip some time to let everything settle:
    run_for_ms(&mut node_exec, 670.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 110);
    // Expect a sine sweep over a
    // range of low frequencies:
    assert_eq!(fft[0],  (86,  112));
    assert_eq!(fft[5],  (237, 112));
    assert_eq!(fft[10], (517, 111));

    // Sweep upwards:
    run_for_ms(&mut node_exec, 300.0);
    let fft = run_and_get_fft4096_now(&mut node_exec, 122);
    assert_eq!(fft[0], (2509, 123));
    assert_eq!(fft[7], (2821, 123));

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
    let dly  = NodeId::Delay(0);
    let out  = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test)
                       .out(None, None, test.out("tsig")));
    matrix.place(0, 3, Cell::empty(test)
                       .out(test.out("sig"), None, None));
    matrix.place(1, 2, Cell::empty(dly)
                       .input(dly.inp("inp"), None, dly.inp("trig"))
                       .out(None, None, dly.out("sig")));
    matrix.place(1, 3, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, dly, "mix",  1.0);
    pset_n(&mut matrix, dly, "mode", 1.0);
    pset_d(&mut matrix, dly, "time", 5.0);

    // Trigger the delay 2 times, with an interval of 20ms:
    pset_n(&mut matrix, test, "p",  1.0);
    run_for_ms(&mut node_exec, 10.0);
    pset_n(&mut matrix, test, "p",  0.0);
    run_for_ms(&mut node_exec, 10.0);
    pset_n(&mut matrix, test, "p",  1.0);
    run_for_ms(&mut node_exec, 10.0);
    pset_n(&mut matrix, test, "p",  0.0);
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
    assert_eq!(idx_first_non_zero, (44100 * 20) / 1000);
}


#[test]
fn check_node_delay_fb() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let dly  = NodeId::Delay(0);
    let out  = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test)
                       .out(None, None, test.out("tsig")));
    matrix.place(1, 2, Cell::empty(dly)
                       .input(dly.inp("inp"), None, None)
                       .out(None, None, dly.out("sig")));
    matrix.place(1, 3, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));

    pset_n(&mut matrix, dly, "mix",  1.0);
    pset_d(&mut matrix, dly, "time", 5.0);
    pset_n(&mut matrix, dly, "fb",   0.5);

    matrix.sync().unwrap();

    // Emit the trigger signal:
    pset_n(&mut matrix, test, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 100.0);

    let idxs_big = collect_signal_changes(&res.0[..], 50);

    // We expect the signal to be delayed by 20ms:
    assert_eq!(idxs_big, vec![(220, 106), (440, 53)]);
}

#[test]
fn check_node_delay_fb_neg() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let dly  = NodeId::Delay(0);
    let out  = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test)
                       .out(None, None, test.out("tsig")));
    matrix.place(1, 2, Cell::empty(dly)
                       .input(dly.inp("inp"), None, None)
                       .out(None, None, dly.out("sig")));
    matrix.place(1, 3, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));

    pset_n(&mut matrix, dly, "mix",  1.0);
    pset_d(&mut matrix, dly, "time", 10.0);
    pset_n(&mut matrix, dly, "fb",   -1.0);

    matrix.sync().unwrap();

    // Emit the trigger signal:
    pset_n(&mut matrix, test, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 40.0);

    let idxs_big = collect_signal_changes(&res.0[..], 70);

    assert_eq!(idxs_big, vec![(441, 100), (882, -100), (1323, 100)]);
}

#[test]
fn check_node_delay_fb_pos() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 4, 4);

    let test = NodeId::Test(0);
    let dly  = NodeId::Delay(0);
    let out  = NodeId::Out(0);
    matrix.place(1, 1, Cell::empty(test)
                       .out(None, None, test.out("tsig")));
    matrix.place(1, 2, Cell::empty(dly)
                       .input(dly.inp("inp"), None, None)
                       .out(None, None, dly.out("sig")));
    matrix.place(1, 3, Cell::empty(out)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, None));

    pset_n(&mut matrix, dly, "mix",  1.0);
    pset_d(&mut matrix, dly, "time", 10.0);
    pset_n(&mut matrix, dly, "fb",   1.0);

    matrix.sync().unwrap();

    // Emit the trigger signal:
    pset_n(&mut matrix, test, "trig", 1.0);

    let res = run_for_ms(&mut node_exec, 100.0);

    let idxs_big = collect_signal_changes(&res.0[..], 70);

    assert_eq!(idxs_big, vec![
        (441,           100),
        (441 + 1 * 441, 100),
        (441 + 2 * 441, 100),
        (441 + 3 * 441, 100),
        (441 + 4 * 441, 100),
        (441 + 5 * 441, 100),
        (441 + 6 * 441, 100),
        (441 + 7 * 441, 100),
        (441 + 8 * 441, 100),
    ]);
}
