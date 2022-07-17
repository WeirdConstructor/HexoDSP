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
    matrix.place(0, 0, Cell::empty(rwk).out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 4.2); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(
        out_l,
        40,
        vec![
            0.0, // start value
            // slew ramp:
            0.001814059,
            0.0139077855,
            0.026001511,
            0.03809524,
            0.050188966,
            0.062282693,
            0.07437642,
            0.08647014,
            0.09856387,
            0.110657595,
            // end value:
            0.11378352,
            0.11378352,
            0.11378352,
            0.11378352,
            0.11378352,
        ]
    );

    pset_n(&mut matrix, rwk, "trig", 0.0);
    pset_d_wait(&mut matrix, &mut node_exec, rwk, "slew", 10.0);
    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 4.0); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(
        out_l,
        15,
        vec![
            0.11378352, 0.11378352, 0.11378352, // last value
            0.11831867, 0.15233228, 0.18634588, 0.22035949, 0.2543731, 0.26017055,
            0.26017055, // end value
        ]
    );
}

#[test]
fn check_node_rndwk_step() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk).out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "step", 1.0);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 4.51); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 50.0);
    assert_decimated_feq!(
        out_l,
        200,
        vec![
            0.0, // start value
            // slew ramp:
            0.054119427,
            0.11458806,
            0.1750567,
            0.23552532,
            0.29599395,
            0.3564626,
            0.4169312,
            0.47739986,
            0.5378685,
            // end value
            // which is 5.0 * 0.11378352
            // (the first random sample, see previous test)
            0.56891763,
            0.56891763,
        ]
    );
}

#[test]
fn check_node_rndwk_offs() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk).out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "offs", 0.3);
    pset_d(&mut matrix, rwk, "slew", 10.0);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 4.51); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(
        out_l,
        60,
        vec![
            0.0, // start value
            // slew ramp:
            0.088435374,
            0.2244898,
            0.3605442,
            // end value
            // which is 0.11378352 + 0.3
            // (the first random sample, see previous test)
            0.41378355,
            0.41378355,
            0.41378355,
        ]
    );
}

#[test]
fn check_node_rndwk_offs_neg() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk).out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "offs", -0.2);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 4.51); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_feq!(
        out_l,
        60,
        vec![
            0.0, // start value
            // slew ramp:
            0.011791383,
            0.029931974,
            0.04807256,
            0.06621315,
            0.084353745,
            // end value
            // which is (0.11378352 - 0.2).abs()
            0.08621648,
            0.08621648,
        ]
    );
}

#[test]
fn check_node_rndwk_max() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk).out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "step", 1.0); // => first sample is 0.56891763
    pset_d(&mut matrix, rwk, "max", 0.5);
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 4.51); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 50.0);
    assert_decimated_feq!(
        out_l,
        200,
        vec![
            0.0, // start value
            // slew ramp:
            0.054119427,
            0.11458806,
            0.1750567,
            0.23552532,
            0.29599395,
            0.3564626,
            0.4169312,
            // end value
            // which is (0.5 - 0.43108237) == 0.06891763
            0.43108237,
            0.43108237,
            0.43108237,
            0.43108237
        ]
    );
}

#[test]
fn check_node_rndwk_min() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let rwk = NodeId::RndWk(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(rwk).out(None, None, rwk.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    pset_d(&mut matrix, rwk, "step", 1.0); // => first sample is 0.56891763
    pset_d(&mut matrix, rwk, "max", 1.0);
    pset_d(&mut matrix, rwk, "min", 0.75); // wraps first sample to 0.93108237
    matrix.sync().unwrap();

    pset_n(&mut matrix, rwk, "trig", 1.0);
    run_for_ms(&mut node_exec, 4.51); // wait for trigger...

    let (out_l, _) = run_for_ms(&mut node_exec, 100.0); // 75ms slew time default
    assert_decimated_feq!(
        out_l,
        400,
        vec![
            0.0, // start value
            // slew ramp:
            0.11458806, 0.23552532, 0.3564626, 0.47739986, 0.5983371, 0.7192744, 0.84021163,
            // end value
            0.93108237, 0.93108237, 0.93108237, 0.93108237, 0.93108237, 0.93108237,
        ]
    );
}
