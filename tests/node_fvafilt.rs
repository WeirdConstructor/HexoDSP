// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

fn setup_fvafilt_matrix() -> (Matrix, NodeExecutor) {
    let (node_conf, node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("noise", "sig")
        .node_io("fvafilt", "inp", "sig")
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    (matrix, node_exec)
}

#[test]
fn check_node_fvafilt_ladder_400hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 0);
    pset_s(&mut matrix, va, "lslope", 0);
    pset_d(&mut matrix, va, "freq", 400.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 4.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 6dB slope
    let out = fftr512_now_long(&mut node_exec, 3, 4);

    assert_vis_fft!(
        out,
        [
            (0, 21),
            (86, 15),
            (172, 15),
            (258, 21),
            (345, 21),
            (431, 12),
            (517, 9),
            (603, 9),
            (689, 15),
            (775, 6),
            (861, 9),
            (947, 9),
            (1120, 6),
            (1206, 6),
            (1378, 6),
            (1464, 6),
            (1637, 6)
        ]
    );

    // 24dB slope
    pset_s(&mut matrix, va, "lslope", 3);
    let out = fftr512_now_long(&mut node_exec, 3, 2);

    assert_vis_fft!(
        out,
        [(0, 15), (86, 18), (172, 15), (258, 9), (345, 6), (431, 3), (517, 3), (603, 3)]
    );

    // 24dB with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr512_now_long(&mut node_exec, 3, 6);
    assert_vis_fft!(
        out,
        [(0, 21), (86, 24), (172, 39), (258, 33), (345, 15), (431, 6), (517, 6), (603, 6)]
    );

    // 24dB with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr512_now_long(&mut node_exec, 3, 6);
    assert_vis_fft!(
        out,
        [
            (0, 39),
            (86, 30),
            (172, 21),
            (258, 99),
            (345, 354),
            (431, 375),
            (517, 120),
            (603, 12),
            (689, 15),
            (775, 9)
        ]
    );
}

#[test]
fn check_node_fvafilt_ladder_1000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 0);
    pset_s(&mut matrix, va, "lslope", 0);
    pset_d(&mut matrix, va, "freq", 1000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 4.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 6dB slope
    let out = fftr512_now_long(&mut node_exec, 3, 4);

    assert_vis_fft!(
        out,
        [
            (0, 21),
            (86, 15),
            (172, 18),
            (258, 27),
            (345, 27),
            (431, 18),
            (517, 12),
            (603, 15),
            (689, 21),
            (775, 12),
            (861, 15),
            (947, 15),
            (1034, 9),
            (1120, 12),
            (1206, 12),
            (1292, 9),
            (1378, 15),
            (1464, 15),
            (1550, 9),
            (1637, 12),
            (1723, 9),
            (1809, 9),
            (1981, 6),
            (2067, 6),
            (2239, 9),
            (2326, 6),
            (2412, 6),
            (2498, 6),
            (2584, 6),
            (2670, 6),
            (2756, 6),
            (2842, 6),
            (2929, 9),
            (3015, 6),
            (3187, 6),
            (3359, 6),
            (3445, 6),
            (3531, 6),
            (3704, 6),
            (3790, 6),
            (3876, 6),
            (3962, 6),
            (4307, 6),
            (4393, 6),
            (4823, 6),
            (5254, 6)
        ]
    );

    // 24dB slope
    pset_s(&mut matrix, va, "lslope", 3);
    let out = fftr512_now_long(&mut node_exec, 3, 4);

    assert_vis_fft!(
        out,
        [
            (0, 24),
            (86, 24),
            (172, 21),
            (258, 21),
            (345, 15),
            (431, 12),
            (517, 12),
            (603, 12),
            (689, 9),
            (775, 6),
            (861, 6),
            (947, 6),
            (1120, 6),
            (1206, 6),
        ]
    );

    // 24dB with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr512_now_long(&mut node_exec, 3, 9);
    assert_vis_fft!(
        out,
        [
            (0, 24),
            (86, 24),
            (172, 33),
            (258, 27),
            (345, 18),
            (431, 21),
            (517, 30),
            (603, 27),
            (689, 18),
            (775, 24),
            (861, 24),
            (947, 18),
            (1034, 9),
            (1120, 9),
            (1292, 9)
        ]
    );

    // 24dB with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr512_now_long(&mut node_exec, 3, 9);
    assert_vis_fft!(
        out,
        [
            (0, 39),
            (86, 30),
            (172, 15),
            (258, 30),
            (345, 27),
            (431, 36),
            (517, 45),
            (603, 60),
            (689, 81),
            (775, 117),
            (861, 114),
            (947, 396),
            (1034, 381),
            (1120, 135),
            (1206, 39),
            (1292, 36),
            (1378, 21),
            (1550, 9),
            (1637, 9),
            (1809, 9)
        ]
    );
}

// It is known, that driving the SVF filter too hard will cause
// weird numeric behavior in the SVF filter.
#[test]
fn check_overdriven_dc_svf_bug() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("bosc", "sig")
        .set_atom("wtype", SAtom::setting(3))
        .set_denorm("freq", 440.0)
        .node_io("fvafilt", "inp", "sig")
        .set_norm("drive", 1.0)
        .set_denorm("freq", 14000.0)
        .set_atom("ftype", SAtom::setting(1))
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 2000.0);
    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 100.0);
    println!("{:#?}", rmsmima);
    assert_rmsmima!(rmsmima, (1.0, -1.0, -1.0));
}

#[test]
fn check_overdriven_dc_sallen_key_ok() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("bosc", "sig")
        .set_atom("wtype", SAtom::setting(3))
        .set_denorm("freq", 440.0)
        .node_io("fvafilt", "inp", "sig")
        .set_norm("drive", 1.0)
        .set_denorm("freq", 14000.0)
        .set_atom("ftype", SAtom::setting(2))
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 2000.0);
    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 100.0);
    println!("{:#?}", rmsmima);
    assert_rmsmima!(rmsmima, (0.96078, -1.1445, 1.1434));
}

#[test]
fn check_overdriven_dc_ladder_ok() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("bosc", "sig")
        .set_atom("wtype", SAtom::setting(3))
        .set_denorm("freq", 440.0)
        .node_io("fvafilt", "inp", "sig")
        .set_norm("drive", 1.0)
        .set_denorm("freq", 14000.0)
        .set_atom("ftype", SAtom::setting(0))
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 2000.0);
    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 100.0);
    println!("{:#?}", rmsmima);
    assert_rmsmima!(rmsmima, (0.4004, -0.7787, 0.6732));
}
