// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_ad_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = hexodsp::chain_builder::MatrixCellChain::new(CellDir::B);
    chain.node_out("ad", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();
    matrix.sync().unwrap();

    let ad = NodeId::Ad(0);
    let trig_p = ad.inp_param("trig").unwrap();

    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.007558584,
            0.007558584,
            0.007558584,
            // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.0022675693,
            -0.0022675693,
            -0.0022675842,
            -0.0022675693,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );

    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 10.0);
    matrix.set_param(trig_p, SAtom::param(1.0));

    let res = run_for_ms(&mut node_exec, 25.0);
    let c = collect_non_zero(&res.0[..]);
    // start index at 220, length of the env: 573
    assert_eq!(c, vec![(220, 573)]);

    let peak = res.0[220 + ((44.1_f64 * 3.0).floor() as usize)];
    assert_float_eq!(peak, 1.0);
}

#[test]
fn check_node_ad_retrig() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let test = NodeId::Test(0);
    let ad = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(test).out(None, None, test.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(ad).input(ad.inp("trig"), None, None).out(None, None, ad.out("sig")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let trig_p = test.inp_param("p").unwrap();

    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            // XXX: Direct trigger!
            // Due to Test node outputting an unsmoothed value!

            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.007558584,
            0.007558584,
            0.007558584,
            // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.0022675693,
            -0.0022675693,
            -0.0022675842,
            -0.0022675693,
            -0.0022675726,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );

    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 0.1);
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 1.5);
    assert_decimated_feq!(
        res.0,
        2,
        vec![
            0.0075585,
            0.022675736,
            0.03779289,
            0.05291005,
            0.068027206,
            0.08314436,
            0.09826152,
            0.113378674,
            0.12849583,
            0.143613,
            0.15873015,
            0.1738473,
            0.18896446,
            0.20408161,
            0.21919878,
            0.23431593,
            0.24943309,
            0.26455024,
            0.2796674,
            0.29478455,
            0.3099017,
            0.32501888,
            0.34013602,
            0.3552532,
            0.37037033,
            0.3854875,
            0.40060467,
            0.4157218,
            0.43083897,
            0.4459561,
            0.46107328,
            0.47619045,
            0.4913076
        ]
    );

    // Reset trigger
    matrix.set_param(trig_p, SAtom::param(0.0));
    let res = run_for_ms(&mut node_exec, 0.1);
    assert_slope_feq!(res.0, vec![0.00755; 3]);

    // Retrigger attack (should do nothing)
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 0.1);
    assert_slope_feq!(res.0, vec![0.00755; 7]);

    // Wait into decay phase
    matrix.set_param(trig_p, SAtom::param(0.0));
    let res = run_for_ms(&mut node_exec, 1.4);
    let mut v = vec![0.00755; 57];
    v.append(&mut vec![0.002267, -0.002267, -0.002267]);
    assert_slope_feq!(res.0, v);

    // Decay some more
    let res = run_for_ms(&mut node_exec, 0.8);
    assert_slope_feq!(res.0, vec![-0.002267; 100]);

    // Retrigger right in the decay phase
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 1.0);
    assert_slope_feq!(
        res.0,
        vec![
            // Re-attack until we are at 1.0 again
            0.007558584,
            0.007558584,
            0.007558584,
            0.0075585246,
            0.007558584,
            0.007558584,
            0.007558584,
            0.007558584,
            0.007558584,
            0.007558584,
            0.0007558465,
            // Restart decay after 1.0 was reached:
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395
        ]
    );
}

#[test]
fn check_node_ad_inp_sin() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let ad = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(ad).input(ad.inp("inp"), None, None).out(None, None, ad.out("sig")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let trig_p = ad.inp_param("trig").unwrap();
    let atk_p = ad.inp_param("atk").unwrap();
    let dcy_p = ad.inp_param("dcy").unwrap();

    // check if we have any frequencies resembling 440Hz
    matrix.set_param(trig_p, SAtom::param(1.0));
    run_for_ms(&mut node_exec, 4.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 6);
    assert_eq!(fft, vec![(409, 6), (420, 7), (431, 7), (441, 7), (452, 7), (463, 7), (474, 6)]);

    // Next we test if lengthening the attack has
    // effect on the captured frequencies.
    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);

    matrix.set_param(atk_p, SAtom::param(atk_p.norm(40.0)));
    matrix.set_param(trig_p, SAtom::param(1.0));
    let fft = run_and_get_fft4096_now(&mut node_exec, 300);
    assert_eq!(fft, vec![(431, 318), (441, 354)]);

    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);

    // Next we test if lengthening the decay too has
    // effect on the captured frequencies.
    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);

    matrix.set_param(dcy_p, SAtom::param(dcy_p.norm(40.0)));
    matrix.set_param(trig_p, SAtom::param(1.0));
    run_for_ms(&mut node_exec, 7.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 300);
    assert_eq!(fft[0], (431, 477));
    assert_eq!(fft[1], (441, 628));
    assert_eq!(fft[2], (452, 389));

    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);
}

#[test]
fn check_node_ad_shp_log() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let ad = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(ad).out(None, None, ad.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, ad, "trig", 1.0);
    pset_n(&mut matrix, ad, "ashp", 1.0);
    pset_n(&mut matrix, ad, "dshp", 1.0);

    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.008391023,
            0.0045030117,
            0.0026732683,
            // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
            -0.0005967021,
            -0.000685215,
            -0.0007713437,
            -0.0008877516,
            -0.0010555983,
            -0.0013227463,
            -0.0018290281,
            -0.0032775402,
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
fn check_node_ad_shp_exp() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let ad = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(ad).out(None, None, ad.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, ad, "trig", 1.0);
    pset_n(&mut matrix, ad, "ashp", 0.0);
    pset_n(&mut matrix, ad, "dshp", 0.0);

    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.0029080845,
            0.007420212,
            0.023684025,
            // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
            -0.006719053,
            -0.004248917,
            -0.0026466101,
            -0.0015081167,
            -0.00075439364,
            -0.00030602422,
            -0.00008370355,
            -0.000008119583,
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
fn check_node_ad_eoet() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let ad = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(ad).out(None, None, ad.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.place(1, 0, Cell::empty(ad).out(None, None, ad.out("eoet")));
    matrix.place(1, 1, Cell::empty(out).input(out.inp("ch2"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, ad, "trig", 1.0);
    let res = run_for_ms(&mut node_exec, 25.0);
    // just make sure we are running an env:
    assert_decimated_slope_feq!(
        res.0,
        50,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.007558584,
            0.007558584,
            0.007558584,
            // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
            -0.002267599,
            -0.0022675395,
            -0.002267599,
            -0.0022675395,
            -0.0022675693,
            -0.0022675693,
            -0.0022675842,
            -0.0022675693,
            0.0, // <- EOET expected here
            0.0,
            0.0,
            0.0
        ]
    );

    // check if trigger appears:
    assert_decimated_feq!(
        res.1,
        50,
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
            0.0, 0.0, 0.0,
            // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, // <- End of envelope!
            0.0, 0.0, 0.0
        ]
    );
}

#[test]
fn check_node_ad_atk_dcy() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let test = NodeId::Test(0);
    let ad = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(test).out(None, None, test.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(ad).input(ad.inp("trig"), None, None).out(None, None, ad.out("sig")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_d(&mut matrix, ad, "atk", 20.0);
    pset_n(&mut matrix, test, "p", 0.0);
    run_for_ms(&mut node_exec, 10.0);

    pset_n(&mut matrix, test, "p", 1.0);
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_slope_feq!(res.0, 10, vec![0.001133787; 50]);

    pset_d(&mut matrix, ad, "atk", 50.0);
    let res = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_slope_feq!(
        res.0,
        40,
        vec![
            // Slope is getting less and less steep, as expected:
            0.0011277795,
            0.0010179877,
            0.00092345476,
            0.00084143877,
            0.0007699132,
            0.0007072091,
            0.0006517768,
            0.00060266256,
            0.00055885315,
            0.0005196929,
            0.00048446655,
            0.00045353174,
            // Slope does not change after the "atk" change has been smoothed
            0.00045353174,
            0.00045353174,
            0.00045353174,
            0.00045347214,
            0.00045353174,
            0.00045353174,
            0.00045353174,
            0.00045353174,
            0.00045347214,
            0.00045353174,
            // attack phase ended, and now we decay:
            -0.002267599
        ]
    );

    // check if decay stays stable:
    let res = run_for_ms(&mut node_exec, 2.0);
    assert_decimated_slope_feq!(res.0, 40, vec![-0.002267599; 3]);

    pset_d(&mut matrix, ad, "dcy", 200.0);
    let res = run_for_ms(&mut node_exec, 20.0);
    assert_decimated_slope_feq!(
        res.0,
        40,
        vec![
            // Slope is getting less and less steep, as expected:
            -0.002197802,
            -0.0012806058,
            -0.00083732605,
            -0.0005899668,
            -0.00043797493,
            -0.00033789873,
            -0.00026863813,
            -0.00021868944,
            -0.00018143654,
            -0.00015294552,
            -0.00013071299,
            // Slope does not change after the "dcy" change has been smoothed
            -0.000113368034,
            -0.000113368034,
            -0.00011339784,
            -0.00011339784,
            -0.000113368034,
            -0.000113368034,
            -0.00011339784,
            -0.000113368034,
            -0.000113368034,
            -0.00011339784,
            -0.000113368034,
            -0.000113368034
        ]
    );
}

#[test]
fn check_node_ad_mult() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let ad = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(ad).out(None, None, ad.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, ad, "trig", 1.0);
    pset_n(&mut matrix, ad, "mult", 2.0);
    let res = run_for_ms(&mut node_exec, 2000.0);
    assert_decimated_slope_feq_fine!(
        res.0,
        1800,
        vec![
            0.0,
            // looong attack:
            0.00007558,
            0.00007558,
            0.00007558,
            0.00007558,
            0.00007558,
            0.00007558,
            0.00007558,
            // looong decay:
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
            -0.000022709,
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
        ]
    );
}
