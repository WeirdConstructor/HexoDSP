// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn trig_env(matrix: &mut Matrix, node_exec: &mut NodeExecutor) {
    let ad_1    = NodeId::Ad(0);
    pset_n(matrix, ad_1, "trig", 1.0);
    run_for_ms(node_exec, 7.0); // Wait for attack start.
    pset_n(matrix, ad_1, "trig", 0.0);
}

fn setup_pverb(matrix: &mut Matrix) {
    let sin_1   = NodeId::Sin(0);
    let ad_1    = NodeId::Ad(0);
    let pverb_1 = NodeId::PVerb(0);
    let out_1   = NodeId::Out(0);
    matrix.place(0, 0,
        Cell::empty(sin_1)
        .input(None, None, None)
        .out(None, None, sin_1.out("sig")));
    matrix.place(0, 1,
        Cell::empty(ad_1)
        .input(ad_1.inp("inp"), None, None)
        .out(ad_1.out("sig"), None, None));
    matrix.place(1, 0,
        Cell::empty(pverb_1)
        .input(None, None, pverb_1.inp("in_l"))
        .out(None, None, pverb_1.out("sig_l")));
    matrix.place(1, 1,
        Cell::empty(out_1)
        .input(out_1.inp("ch1"), None, None)
        .out(None, None, None));
    pset_n(matrix, ad_1, "ashp", 0.870);
    pset_n(matrix, ad_1, "dshp", 0.870);
    pset_d(matrix, ad_1, "atk", 6.0);
    pset_d(matrix, ad_1, "dcy", 100.0);
    pset_n(matrix, pverb_1, "mix", 1.000);
    // quiet down input by 50%, since in_l is doubled in volume effectively:
    pset_mod(matrix, pverb_1, "in_l", 0.5);
}

#[test]
fn check_node_pverb_dcy_1() {
    init_test!(matrix, node_exec, 3);

    let pverb_1 = NodeId::PVerb(0);

    setup_pverb(matrix);
    matrix.sync().unwrap();

//    pset_n(&mut matrix, pverb_1, "dcy", 0.675);
//    pset_n(&mut matrix, pverb_1, "dif", 1.000);
//    pset_n(&mut matrix, pverb_1, "ihpf", -1.543);
//    pset_n(&mut matrix, pverb_1, "ilpf", 0.565);
//    pset_n(&mut matrix, pverb_1, "in_l", 0.000);
//    pset_n(&mut matrix, pverb_1, "in_r", 0.000);
//    pset_n(&mut matrix, pverb_1, "mdepth", 0.200);
//    pset_n(&mut matrix, pverb_1, "mshp", 0.500);
//    pset_n(&mut matrix, pverb_1, "mspeed", 0.075);
//    pset_n(&mut matrix, pverb_1, "predly", 0.000);
//    pset_n(&mut matrix, pverb_1, "rhpf", -1.543);
//    pset_n(&mut matrix, pverb_1, "rlpf", 0.565);
//    pset_n(&mut matrix, pverb_1, "size", 0.330);

    // Dry mix:
    pset_n_wait(matrix, node_exec, pverb_1, "mix", 0.000);
    trig_env(matrix, node_exec);

    let spec = run_fft_spectrum_each_47ms(node_exec, 5, 4);
    //d// for s in &spec { println!("{:?}", s); }

    // We see the sine decaying with the AD envelope:
    assert_eq!(spec[0], vec![(388, 42), (431, 120), (474, 82), (517, 6)]);
    assert_eq!(spec[1], vec![(388, 32), (431, 92), (474, 63), (517, 5)]);
    assert_eq!(spec[2], vec![(345, 5), (388, 12), (431, 16),  (474, 14), (517, 8)]);
    assert_eq!(spec[3], vec![]);

    // Wet mix & clear out the reset in the tank:
    pset_n(matrix, pverb_1, "mix", 0.000);
    pset_n(matrix, pverb_1, "dcy", 0.000);
    run_for_ms(node_exec, 100.0); // Wait flor clearance

    // Check that there is nothing playing:
    let spec = run_fft_spectrum_each_47ms(node_exec, 20, 1);
    assert_eq!(spec[0], vec![]);

    // Wet mix and decay:
    pset_n(matrix, pverb_1, "dcy", 0.2);
    pset_n_wait(matrix, node_exec, pverb_1, "mix", 1.0);
    trig_env(matrix, node_exec);

    let spec = run_fft_spectrum_each_47ms(node_exec, 5, 20);
    //d// for (i, s) in spec.iter().enumerate() { println!("{:2} {:?}", i, s); }

    //  0 [(388, 19), (431, 74), (474, 61), (517, 10)]
    //  1 [(388, 64), (431, 150), (474, 92), (517, 11)]
    //  2 [(345, 5), (388, 72), (431, 205), (474, 138), (517, 11), (560, 5)]
    //  3 [(388, 82), (431, 220), (474, 146), (517, 8)]
    //  4 [(345, 6), (388, 54), (431, 161), (474, 109), (517, 9)]
    //  5 [(388, 9), (431, 37), (474, 26)]
    //  6 [(388, 12), (431, 11), (474, 6)]
    //  7 [(388, 10), (431, 23), (474, 11)]
    //  8 [(388, 14), (431, 37), (474, 26)]
    //  9 [(388, 18), (431, 50), (474, 37), (517, 5)]
    // 10 [(388, 10), (431, 18), (474, 7)]
    // 11 [(388, 15), (431, 34), (474, 27), (517, 7)]
    // 12 [(431, 15), (474, 16)]
    // 13 [(388, 9), (431, 29), (474, 24)]
    // 14 [(388, 18), (431, 47), (474, 30)]
    // 15 [(388, 7), (431, 18), (474, 12)]
    // 16 [(431, 6)]
    // 17 [(388, 7), (431, 19), (474, 12)]
    // 18 [(388, 7), (431, 24), (474, 17)]
    // 19 []

    // Now we see a very much longer tail:
    assert_eq!(spec[0], vec![(388, 19), (431, 74), (474, 61), (517, 10)]);
    assert_eq!(spec[5], vec![(388, 9), (431, 37), (474, 26)]);
    assert_eq!(spec[9], vec![(388, 18), (431, 50), (474, 37), (517, 5)]);
    assert_eq!(spec[19], vec![(388, 7), (431, 15), (474, 8)]);
}

#[test]
fn check_node_pverb_dcy_2() {
    init_test!(matrix, node_exec, 3);
    let pverb_1 = NodeId::PVerb(0);

    setup_pverb(matrix);
    matrix.sync().unwrap();

    // Small room, short decay:
    pset_n_wait(matrix, node_exec, pverb_1, "dcy", 0.2);
    pset_n_wait(matrix, node_exec, pverb_1, "size", 0.1);
    trig_env(matrix, node_exec);

    let rms_spec = run_and_get_rms_mimax(node_exec, 500.0, 100.0);
    //d// dump_table!(rms_spec);
    assert_vec_feq!(rms_spec.iter().map(|rms| rms.0).collect::<Vec<f32>>(),
    // Decay over 500 ms:
    vec![
        0.2108,
        0.5744,
        0.0881,
        0.0021,
        0.0006
    ]);
}

#[test]
fn check_node_pverb_dcy_3() {
    init_test!(matrix, node_exec, 3);
    let pverb_1 = NodeId::PVerb(0);

    setup_pverb(matrix);
    matrix.sync().unwrap();

    // Small room, long decay:
    pset_n_wait(matrix, node_exec, pverb_1, "dcy", 0.8);
    pset_n_wait(matrix, node_exec, pverb_1, "size", 0.1);
    trig_env(matrix, node_exec);

    // Run and get RMS for 5 seconds, averaging RMS over 1 seconds:
    let rms_spec = run_and_get_rms_mimax(node_exec, 5000.0, 1000.0);
    //d// dump_table!(rms_spec);
    assert_vec_feq!(
        rms_spec.iter().map(|rms| rms.0).collect::<Vec<f32>>(),
    // Decay over 5000 ms:
    vec![
        0.6254,
        0.2868,
        0.0633,
        0.0385,
        0.0186,
    ]);
}

#[test]
fn check_node_pverb_dcy_4() {
    init_test!(matrix, node_exec, 3);
    let pverb_1 = NodeId::PVerb(0);

    setup_pverb(matrix);
    matrix.sync().unwrap();

    // Big room, long decay:
    pset_n_wait(matrix, node_exec, pverb_1, "dcy", 0.8);
    pset_n_wait(matrix, node_exec, pverb_1, "size", 0.5);
    trig_env(matrix, node_exec);

    let rms_spec = run_and_get_rms_mimax(node_exec, 5000.0, 1000.0);
    //d// dump_table!(rms_spec);
    assert_vec_feq!(rms_spec.iter().map(|rms| rms.0).collect::<Vec<f32>>(),
    // Decay over 10000 ms:
    vec![
        0.1313,
        0.0995,
        0.0932,
        0.0507,
        0.0456,
    ]);
}


#[test]
fn check_node_pverb_dif_on() {
    init_test!(matrix, node_exec, 3);
    let ad_1    = NodeId::Ad(0);
    let pverb_1 = NodeId::PVerb(0);

    setup_pverb(matrix);
    matrix.sync().unwrap();

    // More plucky:
    pset_d(matrix, ad_1, "atk", 4.0);
    pset_d(matrix, ad_1, "dcy", 40.0);

    // Small room, long decay:
    pset_n(matrix, pverb_1, "dif", 1.0);
    pset_n(matrix, pverb_1, "dmix", 1.0);
    pset_n_wait(matrix, node_exec, pverb_1, "dcy", 0.1);
    pset_n_wait(matrix, node_exec, pverb_1, "size", 0.5);
    trig_env(matrix, node_exec);

    let spec = run_fft_spectrum_each_47ms(node_exec, 4, 20);
    //d// dump_table!(spec);

    //  0 [(388, 8), (431, 35), (474, 35), (517, 7), (560, 5)]
    //  1 [(345, 5), (388, 43), (431, 91), (474, 54), (517, 11), (560, 6), (603, 4)]
    //  2 [(388, 37), (431, 130), (474, 100), (517, 7), (560, 6)]
    //  3 [(345, 5), (388, 36), (431, 80), (474, 53), (517, 8), (560, 4)]
    //  4 [(388, 39), (431, 95), (474, 63), (517, 6)]
    //  5 [(388, 17), (431, 44), (474, 28)]
    //  6 [(388, 8), (431, 5), (474, 5)]
    //  7 [(431, 18), (474, 21), (517, 6)]
    //  8 [(388, 5), (431, 22), (474, 17)]
    //  9 [(388, 6), (431, 14), (474, 9)]
    // 10 [(388, 7), (431, 13), (474, 9)]
    // 11 [(388, 4), (431, 16), (474, 14)]
    // 12 [(431, 6), (474, 6)]
    // 13 [(388, 6), (431, 6)]
    // 14 [(388, 5), (431, 6), (474, 4)]
    // 15 [(388, 8), (431, 13), (474, 6)]
    // 16 [(431, 8), (474, 4)]
    // 17 []

    // We expect a diffuse but defined response:
    assert_eq!(spec[0], vec![(388, 8), (431, 35), (474, 35), (517, 7), (560, 5)]);
    assert_eq!(spec[7], vec![(431, 18), (474, 21), (517, 6)]);
    assert_eq!(spec[13], vec![(388, 6), (431, 6)]);
    assert_eq!(spec[17], vec![]);
}

#[test]
fn check_node_pverb_dif_off() {
    init_test!(matrix, node_exec, 3);
    let ad_1    = NodeId::Ad(0);
    let pverb_1 = NodeId::PVerb(0);

    setup_pverb(matrix);
    matrix.sync().unwrap();

    // More plucky:
    pset_d(matrix, ad_1, "atk", 4.0);
    pset_d(matrix, ad_1, "dcy", 40.0);

    // Small room, long decay:
    pset_n(matrix, pverb_1, "dif", 0.0);
    pset_n(matrix, pverb_1, "dmix", 0.0);
    pset_n_wait(matrix, node_exec, pverb_1, "dcy", 0.1);
    pset_n_wait(matrix, node_exec, pverb_1, "size", 0.5);
    trig_env(matrix, node_exec);

    let spec = run_fft_spectrum_each_47ms(node_exec, 4, 20);
    //d// dump_table!(spec);

    //  0 []
    //  1 [(301, 4), (345, 6), (388, 84), (431, 206), (474, 152), (517, 23), (560, 7)]
    //  2 []
    //  3 [(345, 7), (388, 79), (431, 198), (474, 134), (517, 15), (560, 4)]
    //  4 []
    //  5 []
    //  6 []
    //  7 []
    //  8 [(388, 6), (431, 17), (474, 11)]
    //  9 [(388, 7), (431, 20), (474, 13)]
    // 10 []
    // 11 []
    // 12 [(388, 5), (431, 8), (474, 14), (517, 6)]
    // 13 []
    // 14 []
    // 15 []
    // 16 []
    // 17 []
    // 18 []
    // 19 []

    // We expect a diffuse but defined response:
    assert_eq!(spec[0], vec![]);
    assert_eq!(spec[1], vec![(301, 4), (345, 6), (388, 84), (431, 206), (474, 152), (517, 23), (560, 7)]);
    assert_eq!(spec[2], vec![]);
    assert_eq!(spec[3], vec![(345, 7), (388, 79), (431, 198), (474, 134), (517, 15), (560, 4)]);
    assert_eq!(spec[7], vec![]);
    assert_eq!(spec[8], vec![(388, 6), (431, 17), (474, 11)]);
    assert_eq!(spec[9], vec![(388, 7), (431, 20), (474, 13)]);
    assert_eq!(spec[10], vec![]);
    assert_eq!(spec[17], vec![]);
    assert_eq!(spec[19], vec![]);
}


#[test]
fn check_node_pverb_dif_off_predly() {
    init_test!(matrix, node_exec, 3);
    let ad_1    = NodeId::Ad(0);
    let pverb_1 = NodeId::PVerb(0);

    setup_pverb(matrix);
    matrix.sync().unwrap();

    // More plucky:
    pset_d(matrix, ad_1, "atk", 4.0);
    pset_d(matrix, ad_1, "dcy", 40.0);

    // Enable pre-delay of 150ms:
    pset_d(matrix, pverb_1, "predly", 150.0);

    // Small room, long decay:
    pset_n(matrix, pverb_1, "dif", 0.0);
    pset_n(matrix, pverb_1, "dmix", 0.0);
    pset_n_wait(matrix, node_exec, pverb_1, "dcy", 0.1);
    pset_n_wait(matrix, node_exec, pverb_1, "size", 0.5);
    trig_env(matrix, node_exec);

    let spec = run_fft_spectrum_each_47ms(node_exec, 4, 20);
    dump_table!(spec);


    // 0 []
    // 1 []
    // 2 []
    // 3 []
    // 4 [(215, 5), (301, 11), (345, 15), (388, 46), (431, 105), (474, 86), (517, 18), (560, 14), (603, 5)]
    // 5 [(345, 6), (388, 21), (431, 35), (474, 30), (517, 11), (560, 4)]
    // 6 [(258, 5), (301, 7), (345, 18), (388, 52), (431, 140), (474, 76), (517, 13), (560, 10), (603, 5)]
    // 7 [(345, 7), (388, 17), (431, 24), (474, 21), (517, 12), (560, 4)]
    // 8 []
    // 9 []

    // We expect a diffuse but defined response:
    assert_eq!(spec[0], vec![]); // ~50ms
    assert_eq!(spec[1], vec![]); // ~50ms
    assert_eq!(spec[2], vec![]); // ~50ms
    assert_eq!(spec[3], vec![]); // ~50ms
    assert_eq!(spec[4], vec![(215, 5), (301, 11), (345, 15), (388, 46), (431, 105), (474, 86), (517, 18), (560, 14), (603, 5)]);
}
