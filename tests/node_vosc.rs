// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup_vosc(matrix: &mut Matrix) {
    let vosc_1  = NodeId::VOsc(0);
    let amp_1   = NodeId::Amp(0);
    let out_1   = NodeId::Out(0);
    matrix.place(0, 1,
        Cell::empty(vosc_1)
        .input(vosc_1.inp("damt"), None, None)
        .out(vosc_1.out("sig"), None, None));
    matrix.place(1, 0,
        Cell::empty(amp_1)
        .input(None, None, amp_1.inp("inp"))
        .out(amp_1.out("sig"), None, None));
    matrix.place(2, 0,
        Cell::empty(out_1)
        .input(None, None, out_1.inp("ch1"))
        .out(None, None, None));

    pset_n(matrix, vosc_1, "d", 0.500);
    pset_n(matrix, vosc_1, "v", 0.500);
    pset_n(matrix, vosc_1, "damt", 0.000);
    pset_n(matrix, vosc_1, "det", 0.000);
    pset_n(matrix, vosc_1, "freq", 0.000);
    pset_n(matrix, vosc_1, "vs", 0.000);
}

#[test]
fn check_node_vosc_sine() {
    init_test!(matrix, node_exec, 3);

    setup_vosc(matrix);
    matrix.sync().unwrap();

    let fft = run_and_get_fft4096_2(node_exec, 500);
    dump_table!(fft);
    assert_eq!(fft, vec![ (431, 614), (441, 1012) ]);
}

#[test]
fn check_node_vosc_d_v() {
    init_test!(matrix, node_exec, 3);
    let vosc_1 = NodeId::VOsc(0);

    setup_vosc(matrix);
    matrix.sync().unwrap();

    // d=0.25, v=0.5
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.25);
    let fft = run_and_get_fft4096_2(node_exec, 50);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 589),
        (441, 972),
        (452, 395),
        (872, 178),
        (883, 244),
        (894, 79),
        (1314, 90),
        (1324, 103)
    ]);

    // d=0.0, v=0.5
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.0);
    let fft = run_and_get_fft4096_2(node_exec, 100);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 521),
        (441, 859),
        (452, 349),
        (872, 242),
        (883, 332),
        (894, 107),
        (1314, 175),
        (1324, 201),
        (1755, 143),
        (1766, 137),
        (2196, 122),
        (2638, 106)
    ]);

    // d=1.0, v=0.5 (symmetric to above)
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.0);
    let fft = run_and_get_fft4096_2(node_exec, 100);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 521),
        (441, 859),
        (452, 349),
        (872, 242),
        (883, 332),
        (894, 107),
        (1314, 175),
        (1324, 201),
        (1755, 143),
        (1766, 137),
        (2196, 122),
        (2638, 106)
    ]);

    // d=0.5, v=0.25
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.5);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.25);
    let fft = run_and_get_fft4096_2(node_exec, 100);
    dump_table!(fft);
    assert_eq!(fft, vec![
        (  0, 434),
        ( 11, 217),
        (431, 554),
        (441, 913),
        (452, 371),
        (872, 215),
        (883, 294)
    ]);

    // d=0.25, v=0.25
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.25);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.25);
    let fft = run_and_get_fft4096_2(node_exec, 50);
    dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 614), (441, 1012), (452, 411)
    ]);

    // d=0.1, v=0.25
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.1);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.25);
    let fft = run_and_get_fft4096_2(node_exec, 50);
    dump_table!(fft);
    assert_eq!(fft, vec![
        (  0, 260),
        ( 11, 130),
        (431, 593),
        (441, 977),
        (452, 397),
        (872, 105),
        (883, 144),
        (1314, 64),
        (1324, 74)
    ]);
}

#[test]
fn check_node_vosc_ovrsmpl() {
    init_test!(matrix, node_exec, 3);
    let vosc_1 = NodeId::VOsc(0);

    setup_vosc(matrix);
    matrix.sync().unwrap();

    // d=0.0, v=0.5, with oversampling
    pset_s(matrix, vosc_1, "ovrsmpl", 1);
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.0);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.5);
    let fft = run_and_get_fft4096_2(node_exec, 25);
    dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 521),
        (441, 859),
        (452, 349),
        (872, 242),
        (883, 332),
        (894, 107),
        (1303, 27),
        (1314, 175),
        (1324, 201),
        (1335, 50),
        (1744, 30),
        (1755, 143),
        (1766, 137),
        (1776, 25),
        (2186, 34),
        (2196, 122),
        (2207, 98),
        (2627, 38),
        (2638, 106),
        (2649, 71),
        (3068, 41),
        (3079, 93),
        (3090, 51),
        (3510, 44),
        (3521, 81),
        (3531, 37),
        (3951, 46),
        (3962, 70),
        (3973, 25),
        (4393, 48),
        (4404, 60),
        (4834, 48),
        (4845, 51),
        (5276, 48),
        (5286, 42),
        (5717, 47),
        (5728, 34),
        (6158, 45),
        (6169, 27),
        (6600, 42),
        (7041, 39),
        (7472, 25),
        (7483, 35),
        (7913, 27),
        (7924, 31),
        (8355, 28),
        (8366, 27),
        (8796, 28),
        (9238, 28),
        (9679, 27),
        (10121, 26)
    ]);

    // d=0.0, v=0.5, without oversampling
    pset_s(matrix, vosc_1, "ovrsmpl", 0);
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.0);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.5);
    let fft = run_and_get_fft4096_2(node_exec, 25);
    dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 521),
        (441, 859),
        (452, 349),
        (872, 242),
        (883, 332),
        (894, 108),
        (1303, 26),
        (1314, 175),
        (1324, 201),
        (1335, 51),
        (1744, 30),
        (1755, 143),
        (1766, 137),
        (1776, 26),
        (2186, 34),
        (2196, 122),
        (2207, 98),
        (2627, 38),
        (2638, 106),
        (2649, 71),
        (3068, 41),
        (3079, 93),
        (3090, 52),
        (3510, 44),
        (3521, 81),
        (3531, 37),
        (3951, 46),
        (3962, 70),
        (3973, 26),
        (4393, 48),
        (4404, 61),
        (4834, 48),
        (4845, 51),
        (5276, 48),
        (5286, 43),
        (5717, 47),
        (5728, 35),
        (6158, 45),
        (6169, 28),
        (6600, 43),
        (7041, 40),
        (7472, 26),
        (7483, 36),
        (7913, 28),
        (7924, 32),
        (8355, 29),
        (8366, 28),
        (8796, 30),
        (9238, 30),
        (9679, 29),
        (10121, 28),
        (10562, 26)
    ]);
}

#[test]
fn check_node_vosc_dist() {
    init_test!(matrix, node_exec, 3);
    let vosc_1 = NodeId::VOsc(0);

    setup_vosc(matrix);
    matrix.sync().unwrap();

    // dist=TanH
    pset_s(matrix, vosc_1, "dist", 1);
    pset_n_wait(matrix, node_exec, vosc_1, "damt", 0.25);
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.5);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.5);
    let fft = run_and_get_fft4096_2(node_exec, 100);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 781),
        (441, 1287),
        (452, 523),
        (1314, 340),
        (1324, 389),
        (2196, 238),
        (2207, 191),
        (3079, 179),
        (3962, 133)
    ]);

    // dist=B.D.Jong (very similar to the TanH)
    pset_s(matrix, vosc_1, "dist", 2);
    pset_n_wait(matrix, node_exec, vosc_1, "damt", 0.25);
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.5);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.5);
    let fft = run_and_get_fft4096_2(node_exec, 100);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (431, 759),
        (441, 1251),
        (452, 509),
        (1314, 282),
        (1324, 323),
        (2196, 172),
        (2207, 138),
        (3079, 115)
    ]);

    // dist=Fold
    pset_s(matrix, vosc_1, "dist", 3);
    pset_n_wait(matrix, node_exec, vosc_1, "damt", 0.25);
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.5);
    pset_n_wait(matrix, node_exec, vosc_1, "v", 0.5);
    let fft = run_and_get_fft4096_2(node_exec, 100);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (   0, 381),
        (  11, 190),
        ( 431, 370),
        ( 441, 611),
        ( 452, 248),
        ( 872, 242),
        ( 883, 331),
        ( 894, 107),
        (1314, 313),
        (1324, 359),
        (1755, 221),
        (1766, 212),
        (2196, 250),
        (2207, 201),
        (2638, 129),
        (3079, 123)
    ]);
}

#[test]
fn check_node_vosc_vs() {
    init_test!(matrix, node_exec, 3);
    let vosc_1 = NodeId::VOsc(0);

    setup_vosc(matrix);
    matrix.sync().unwrap();

    // d=0.3, vs=2.0
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.3);
    pset_d_wait(matrix, node_exec, vosc_1, "vs", 2.0);
    let fft = run_and_get_fft4096_2(node_exec, 150);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (872, 470),
        (883, 644),
        (894, 208),
        (1314, 356),
        (1324, 408),
        (1755, 194),
        (1766, 186),
        (2638, 190),
        (3079, 308),
        (3090, 171),
        (3510, 176),
        (3521, 321),
        (3951, 163),
        (3962, 246)
    ]);

    // d=0.3, vs=3.0
    pset_n_wait(matrix, node_exec, vosc_1, "d", 0.3);
    pset_d_wait(matrix, node_exec, vosc_1, "vs", 3.0);
    let fft = run_and_get_fft4096_2(node_exec, 150);
    //d// dump_table!(fft);
    assert_eq!(fft, vec![
        (1314, 378),
        (1324, 433),
        (1755, 578),
        (1766, 554),
        (2638, 163),
        (3079, 168),
        (4393, 188),
        (4404, 237),
        (4834, 261),
        (4845, 275),
        (5276, 257),
        (5286, 226),
        (5717, 182)
    ]);
}
