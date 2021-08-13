// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_rndwk_def_trig() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk)
                       .out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 7.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(out_l, 40, vec![
        0.0, // start value
        // 10ms ramp:
        0.0049022376, 0.015222744, 0.025543215, 0.035863716, 0.04618426,
        0.056504805, 0.066825345, 0.07714589, 0.08746643, 0.09778698,
        0.10810752,
        // end value:
        0.11378352, 0.11378352, 0.11378352, 0.11378352, 0.11378352,
    ]);

    pset_n(&mut matrix, rwk, "trig", 0.0);
    pset_d_wait(&mut matrix, &mut node_exec, rwk, "slewt", 1.0);
    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 7.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(out_l, 15, vec![
        0.11378352, 0.11378352, // last value
        0.1436584, 0.19344981, 0.24324122, // 1ms ramp 15 * 3 => ~44.1 samples
        0.26017055, 0.26017055, // end value
    ]);
}

#[test]
fn check_node_rndwk_step() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk)
                       .out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "step", 1.0);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 7.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(out_l, 60, vec![
        0.0, // start value
        // 10ms ramp:
        0.050312463, 0.12771615, 0.20512024, 0.28252393, 0.35992712,
        0.4373303, 0.51473385,
        // end value
        // which is 5.0 * 0.11378352
        // (the first random sample, see previous test)
        0.56891763, 0.56891763,
    ]);
}

#[test]
fn check_node_rndwk_offs() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk)
                       .out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "offs", 0.3);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 7.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(out_l, 60, vec![
        0.0, // start value
        // 10ms ramp:
        0.03659311, 0.0928901, 0.14918698, 0.20548387,
        0.26178095, 0.31807873, 0.3743765,
        // end value
        // which is 0.11378352 + 0.3
        // (the first random sample, see previous test)
        0.41378355,
        0.41378355,
        0.41378355,
    ]);
}

#[test]
fn check_node_rndwk_offs_neg() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk)
                       .out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "offs", -0.2);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 7.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(out_l, 60, vec![
        0.0, // start value
        // 10ms ramp:
        0.007624589, 0.019354708, 0.03108479, 0.042814985, 0.05454518,
        0.06627537, 0.07800557,
        // end value
        // which is (0.11378352 - 0.2).abs()
        0.08621648, 0.08621648,
    ]);
}

#[test]
fn check_node_rndwk_max() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk)
                       .out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "step", 1.0); // => first sample is 0.56891763
    pset_d(&mut matrix, rwk, "max", 0.5);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 7.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(out_l, 60, vec![
        0.0, // start value
        // 10ms ramp:
        0.006094757, 0.015471312, 0.024847867, 0.03422442, 0.043600976,
        0.052977532, 0.062354088,
        // end value
        // which is 0.5 - 0.56891763
        0.06891763, 0.06891763,
    ]);
}

#[test]
fn check_node_rndwk_min() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk)
                       .out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "step", 1.0); // => first sample is 0.56891763
    pset_d(&mut matrix, rwk, "max", 1.0);
    pset_d(&mut matrix, rwk, "min", 0.75); // wraps first sample to 0.93108237
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 7.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(out_l, 60, vec![
        0.0, // start value
        // 10ms ramp:
        0.08234063, 0.20901868, 0.33569613, 0.4623733, 0.5890517,
        0.71573067, 0.8424096,
        // end value
        0.93108237, 0.93108237, 0.93108237, 0.93108237, 0.93108237, 0.93108237,
    ]);
}
