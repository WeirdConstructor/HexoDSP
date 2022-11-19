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
        .node_io("fvafilt", "in_l", "sig_l")
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    (matrix, node_exec)
}

fn setup_fvafilt_matrix_sig_r() -> (Matrix, NodeExecutor) {
    let (node_conf, node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("noise", "sig")
        .node_io("fvafilt", "in_r", "sig_r")
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
    pset_s(&mut matrix, va, "lmode", 0);
    pset_d(&mut matrix, va, "freq", 400.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 4.5);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 6dB slope
    let out = fftr512_now_peaks(&mut node_exec, 3, 4);

    assert_vis_fft!(
        out,
        [
            (0, 27),
            (86, 21),
            (172, 21),
            (258, 15),
            (345, 21),
            (431, 15),
            (517, 12),
            (603, 12),
            (689, 15),
            (775, 12),
            (861, 9),
            (947, 12),
            (1034, 9),
            (1120, 12),
            (1206, 12),
            (1292, 6),
            (1378, 6),
            (1464, 6),
            (1550, 6),
            (1637, 6),
            (1723, 6)
        ]
    );

    // 24dB slope
    pset_s(&mut matrix, va, "lmode", 3);
    let out = fftr512_now_peaks(&mut node_exec, 3, 2);

    assert_vis_fft!(
        out,
        [(0, 21), (86, 21), (172, 18), (258, 9), (345, 12), (431, 6), (517, 3), (603, 3)]
    );

    // 24dB with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr512_now_peaks(&mut node_exec, 3, 6);
    assert_vis_fft!(out, [(0, 24), (86, 15), (172, 15), (258, 18), (345, 18), (431, 9)]);

    // 24dB with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr512_now_peaks(&mut node_exec, 3, 6);
    assert_vis_fft!(out, [(0, 6), (86, 6), (172, 6), (258, 12), (345, 42), (431, 48), (517, 15)]);
}

#[test]
fn check_node_fvafilt_ladder_sig_r() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix_sig_r();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 0);
    pset_s(&mut matrix, va, "lmode", 0);
    pset_d(&mut matrix, va, "freq", 400.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 4.5);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 6dB slope
    let out = fftr512_now_peaks(&mut node_exec, 3, 4);

    assert_vis_fft!(
        out,
        [
            (0, 27),
            (86, 21),
            (172, 21),
            (258, 15),
            (345, 21),
            (431, 15),
            (517, 12),
            (603, 12),
            (689, 15),
            (775, 12),
            (861, 9),
            (947, 12),
            (1034, 9),
            (1120, 12),
            (1206, 12),
            (1292, 6),
            (1378, 6),
            (1464, 6),
            (1550, 6),
            (1637, 6),
            (1723, 6)
        ]
    );
}

#[test]
fn check_node_fvafilt_ladder_1000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 0);
    pset_s(&mut matrix, va, "lmode", 0);
    pset_d(&mut matrix, va, "freq", 1000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 3.2);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 6dB slope
    let out = fftr512_now_peaks(&mut node_exec, 3, 4);

    assert_vis_fft!(
        out,
        [
            (0, 18),
            (86, 18),
            (172, 18),
            (258, 15),
            (345, 21),
            (431, 15),
            (517, 12),
            (603, 12),
            (689, 18),
            (775, 15),
            (861, 12),
            (947, 15),
            (1034, 12),
            (1120, 18),
            (1206, 15),
            (1292, 9),
            (1378, 9),
            (1464, 12),
            (1550, 9),
            (1637, 9),
            (1723, 9),
            (1895, 6),
            (2067, 6),
            (2153, 6),
            (2239, 6),
            (2326, 6),
            (2412, 6),
            (2498, 6),
            (2584, 6),
            (2756, 6),
            (2929, 6),
            (3015, 6),
            (3101, 9),
            (3359, 6),
            (3445, 6),
            (3790, 6),
            (3876, 6),
            (3962, 6),
            (4307, 6)
        ]
    );

    // 24dB slope
    pset_s(&mut matrix, va, "lmode", 3);
    let out = fftr512_now_peaks(&mut node_exec, 3, 4);

    assert_vis_fft!(
        out,
        [
            (0, 24),
            (86, 21),
            (172, 21),
            (258, 15),
            (345, 21),
            (431, 12),
            (517, 9),
            (603, 9),
            (689, 6),
            (775, 6),
            (861, 9),
            (947, 6)
        ]
    );

    // 24dB with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr512_now_peaks(&mut node_exec, 3, 9);
    assert_vis_fft!(
        out,
        [
            (0, 18),
            (86, 12),
            (172, 9),
            (258, 9),
            (345, 18),
            (431, 12),
            (517, 15),
            (603, 12),
            (689, 12),
            (775, 9),
            (861, 9)
        ]
    );

    // 24dB with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr512_now_peaks(&mut node_exec, 3, 3);
    assert_vis_fft!(
        out,
        [
            (0, 3),
            (86, 3),
            (172, 3),
            (258, 3),
            (345, 3),
            (431, 3),
            (517, 3),
            (603, 6),
            (689, 6),
            (775, 6),
            (861, 21),
            (947, 30),
            (1034, 33),
            (1120, 12),
            (1206, 6),
            (1292, 3),
            (1378, 3)
        ]
    );
}

#[test]
fn check_node_fvafilt_ladder_hp() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 0);
    pset_s(&mut matrix, va, "lmode", 5);
    pset_d(&mut matrix, va, "freq", 4000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 4.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 12dB HP
    let out = fftr512_now_peaks(&mut node_exec, 3, 7);
    assert_vis_fft!(
        &out[0..40],
        [
            (2929, 9),
            (3015, 9),
            (3101, 12),
            (3359, 12),
            (3445, 9),
            (3618, 9),
            (3704, 9),
            (3790, 12),
            (3876, 12),
            (3962, 15),
            (4048, 9),
            (4134, 9),
            (4221, 9),
            (4307, 18),
            (4393, 12),
            (4479, 9),
            (4565, 9),
            (4651, 15),
            (4737, 12),
            (4823, 15),
            (4910, 12),
            (4996, 15),
            (5082, 18),
            (5168, 12),
            (5254, 18),
            (5340, 15),
            (5426, 21),
            (5513, 12),
            (5599, 9),
            (5685, 15),
            (5771, 15),
            (5857, 9),
            (5943, 15),
            (6029, 24),
            (6115, 27),
            (6202, 15),
            (6288, 24),
            (6374, 21),
            (6460, 15),
            (6546, 18)
        ]
    );

    // 24dB HP
    pset_s(&mut matrix, va, "lmode", 7);
    let out = fftr512_now_peaks(&mut node_exec, 3, 7);
    assert_vis_fft!(
        &out[0..40],
        [
            (4307, 9),
            (4996, 9),
            (5340, 9),
            (5426, 12),
            (5513, 12),
            (5599, 9),
            (5685, 9),
            (5771, 9),
            (5857, 9),
            (5943, 12),
            (6029, 9),
            (6115, 12),
            (6202, 15),
            (6288, 9),
            (6374, 9),
            (6460, 9),
            (6546, 15),
            (6632, 9),
            (6718, 9),
            (6804, 15),
            (6891, 21),
            (6977, 9),
            (7063, 9),
            (7149, 9),
            (7235, 12),
            (7321, 12),
            (7407, 15),
            (7494, 9),
            (7580, 18),
            (7666, 15),
            (7752, 15),
            (7838, 15),
            (7924, 21),
            (8010, 15),
            (8096, 18),
            (8183, 12),
            (8269, 15),
            (8355, 9),
            (8441, 15),
            (8527, 21)
        ]
    );

    // 24dB HP, res = 1.0
    pset_s(&mut matrix, va, "lmode", 7);
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr512_now_peaks(&mut node_exec, 3, 7);
    assert_vis_fft!(
        &out[0..30],
        [
            (3359, 9),
            (3445, 9),
            (3531, 9),
            (3618, 12),
            (3704, 21),
            (3790, 33),
            (3876, 33),
            (3962, 33),
            (4048, 24),
            (4134, 18),
            (4221, 15),
            (4307, 18),
            (4393, 12),
            (4479, 12),
            (4565, 9),
            (4651, 9),
            (4737, 12),
            (4823, 15),
            (4910, 9),
            (4996, 12),
            (5082, 18),
            (5168, 12),
            (5254, 12),
            (5340, 12),
            (5426, 9),
            (5513, 15),
            (5599, 15),
            (5685, 15),
            (5771, 18),
            (5857, 12)
        ]
    );
}

#[test]
fn check_node_fvafilt_ladder_bp() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 0);
    pset_s(&mut matrix, va, "lmode", 8);
    pset_d(&mut matrix, va, "freq", 4000.0);
    pset_d(&mut matrix, va, "res", 0.8);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 4.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 12dB BP
    let out = fftr512_now_peaks(&mut node_exec, 4, 13);
    assert_vis_fft!(
        out,
        [
            (2929, 24),
            (3015, 28),
            (3101, 36),
            (3187, 20),
            (3273, 24),
            (3359, 40),
            (3445, 36),
            (3531, 28),
            (3618, 28),
            (3704, 32),
            (3790, 36),
            (3876, 40),
            (3962, 32),
            (4048, 24),
            (4134, 20),
            (4221, 20),
            (4307, 36),
            (4393, 24),
            (4479, 16),
            (4565, 16),
            (4651, 20),
            (4737, 16),
            (4823, 20),
            (4996, 16),
            (5082, 16),
            (5254, 16),
            (5426, 16),
            (5685, 16),
            (6029, 16),
            (6115, 20),
            (6288, 16),
            (7752, 16),
            (9130, 16)
        ]
    );

    // 24dB BP
    pset_s(&mut matrix, va, "lmode", 9);
    let out = fftr512_now_peaks(&mut node_exec, 4, 13);
    assert_vis_fft!(
        out,
        [
            (3101, 16),
            (3273, 16),
            (3359, 20),
            (3445, 16),
            (3531, 16),
            (3876, 16),
            (3962, 16),
            (4307, 16),
            (4393, 16)
        ]
    );
}

#[test]
fn check_node_fvafilt_ladder_notch() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 0);
    pset_s(&mut matrix, va, "lmode", 10);
    pset_d(&mut matrix, va, "freq", 4000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 4.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // 12dB Notch
    let out = fftr512_now_peaks(&mut node_exec, 4, 13);
    assert_vis_fft!(
        &out[0..40],
        [
            (0, 28),
            (86, 24),
            (172, 24),
            (258, 24),
            (345, 36),
            (431, 28),
            (517, 20),
            (603, 24),
            (689, 32),
            (775, 28),
            (861, 24),
            (947, 32),
            (1034, 24),
            (1120, 36),
            (1206, 32),
            (1292, 16),
            (1378, 16),
            (1464, 20),
            (1550, 16),
            (1637, 20),
            (1723, 16),
            (6029, 16),
            (6115, 16),
            (6288, 16),
            (7063, 16),
            (7149, 16),
            (7407, 16),
            (7580, 16),
            (7666, 16),
            (7752, 24),
            (7838, 20),
            (7924, 20),
            (8269, 16),
            (8355, 16),
            (8613, 16),
            (8699, 16),
            (8786, 20),
            (8872, 16),
            (8958, 20),
            (9044, 24)
        ]
    );
}

#[test]
fn check_node_fvafilt_svf_lp_1000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 1);
    pset_d(&mut matrix, va, "freq", 1000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // resonance 0.0
    let out = fftr512_now_peaks(&mut node_exec, 3, 7);

    assert_vis_fft!(
        out,
        [
            (0, 21),
            (86, 18),
            (172, 21),
            (258, 18),
            (345, 27),
            (431, 21),
            (517, 15),
            (603, 18),
            (689, 27),
            (775, 21),
            (861, 18),
            (947, 24),
            (1034, 18),
            (1120, 27),
            (1206, 24),
            (1292, 12),
            (1378, 12),
            (1464, 15),
            (1550, 9),
            (1637, 12),
            (1723, 9)
        ]
    );

    // with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr512_now_peaks(&mut node_exec, 3, 9);
    assert_vis_fft!(
        out,
        [
            (0, 12),
            (86, 18),
            (172, 27),
            (258, 30),
            (345, 33),
            (431, 15),
            (517, 21),
            (603, 21),
            (689, 33),
            (775, 30),
            (861, 36),
            (947, 24),
            (1034, 24),
            (1120, 30),
            (1206, 27),
            (1292, 36),
            (1378, 36),
            (1464, 21),
            (1550, 18),
            (1637, 24),
            (1723, 18),
            (1809, 21),
            (1895, 18),
            (1981, 9),
            (2153, 9),
            (2239, 9)
        ]
    );

    // with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr512_now_peaks(&mut node_exec, 3, 14);
    assert_vis_fft!(
        out,
        [
            (0, 36),
            (86, 30),
            (172, 24),
            (258, 33),
            (345, 36),
            (431, 27),
            (517, 27),
            (603, 39),
            (689, 48),
            (775, 54),
            (861, 57),
            (947, 69),
            (1034, 189),
            (1120, 531),
            (1206, 714),
            (1292, 375),
            (1378, 102),
            (1464, 114),
            (1550, 54),
            (1637, 27),
            (1723, 24),
            (1809, 18),
            (1895, 18),
            (1981, 15),
            (3531, 21),
            (3618, 18)
        ]
    );
}

#[test]
fn check_node_fvafilt_svf_lp_sig_r() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix_sig_r();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 1);
    pset_d(&mut matrix, va, "freq", 1000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // resonance 0.0
    let out = fftr512_now_peaks(&mut node_exec, 3, 7);

    assert_vis_fft!(
        out,
        [
            (0, 21),
            (86, 18),
            (172, 21),
            (258, 18),
            (345, 27),
            (431, 21),
            (517, 15),
            (603, 18),
            (689, 27),
            (775, 21),
            (861, 18),
            (947, 24),
            (1034, 18),
            (1120, 27),
            (1206, 24),
            (1292, 12),
            (1378, 12),
            (1464, 15),
            (1550, 9),
            (1637, 12),
            (1723, 9)
        ]
    );
}

#[test]
fn check_node_fvafilt_svf_hp_6000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 1);
    pset_s(&mut matrix, va, "smode", 1);
    pset_d(&mut matrix, va, "freq", 6000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 4.0);

    // resonance 0.0
    let out = fftr64_now_peaks(&mut node_exec, 4, 5);

    assert_vis_fft!(
        out,
        [
            (5513, 8),
            (6202, 8),
            (6891, 8),
            (7580, 8),
            (8269, 12),
            (8958, 12),
            (9647, 16),
            (10336, 16),
            (11025, 12),
            (11714, 8),
            (12403, 12),
            (13092, 16),
            (13781, 16),
            (14470, 12),
            (15159, 16),
            (15848, 16),
            (16538, 16),
            (17227, 16),
            (17916, 20),
            (18605, 16),
            (19294, 12),
            (19983, 12),
            (20672, 12),
            (21361, 12),
            (22050, 12)
        ]
    );

    // with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr64_now_peaks(&mut node_exec, 4, 5);
    assert_vis_fft!(
        out,
        [
            (3445, 8),
            (4134, 8),
            (4823, 8),
            (5513, 12),
            (6202, 16),
            (6891, 24),
            (7580, 20),
            (8269, 24),
            (8958, 24),
            (9647, 20),
            (10336, 20),
            (11025, 20),
            (11714, 20),
            (12403, 24),
            (13092, 20),
            (13781, 16),
            (14470, 16),
            (15159, 20),
            (15848, 16),
            (16538, 24),
            (17227, 24),
            (17916, 16),
            (18605, 12),
            (19294, 16),
            (19983, 16),
            (20672, 16),
            (21361, 16),
            (22050, 16)
        ]
    );

    // with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr64_now_peaks(&mut node_exec, 4, 13);
    assert_vis_fft!(
        out,
        [
            (2756, 16),
            (3445, 24),
            (4134, 36),
            (4823, 44),
            (5513, 40),
            (6202, 36),
            (6891, 40),
            (7580, 24),
            (8269, 24),
            (8958, 20),
            (9647, 16),
            (10336, 16),
            (14470, 16),
            (15848, 16),
            (16538, 16),
            (17227, 16)
        ]
    );
}

#[test]
fn check_node_fvafilt_svf_bp1_6000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 1);
    pset_s(&mut matrix, va, "smode", 2);
    pset_d(&mut matrix, va, "freq", 6000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 4.0);

    // resonance 0.0
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);

    assert_vis_fft!(
        out,
        [
            (4134, 12),
            (4823, 12),
            (5513, 12),
            (6202, 16),
            (6891, 12),
            (7580, 16),
            (8269, 16),
            (8958, 16),
            (9647, 20),
            (10336, 16),
            (11025, 12),
            (12403, 12),
            (13092, 16),
            (13781, 12),
            (14470, 12),
            (15159, 12),
            (15848, 12),
            (16538, 12),
            (17916, 12),
        ]
    );

    // with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);
    assert_vis_fft!(
        out,
        [
            (2756, 12),
            (3445, 28),
            (4134, 28),
            (4823, 20),
            (5513, 28),
            (6202, 32),
            (6891, 36),
            (7580, 32),
            (8269, 36),
            (8958, 32),
            (9647, 24),
            (10336, 20),
            (11025, 24),
            (11714, 20),
            (12403, 20),
            (13092, 16),
            (13781, 12),
            (14470, 12),
            (15159, 12),
            (15848, 12),
            (16538, 16),
            (17227, 16)
        ]
    );

    // with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);
    assert_vis_fft!(
        out,
        [
            (1378, 16),
            (2067, 20),
            (2756, 48),
            (3445, 68),
            (4134, 88),
            (4823, 92),
            (5513, 84),
            (6202, 56),
            (6891, 68),
            (7580, 36),
            (8269, 32),
            (8958, 24),
            (9647, 16),
            (10336, 16),
            (11025, 12),
            (13092, 12),
            (14470, 12),
            (15848, 12)
        ]
    );
}

#[test]
fn check_node_fvafilt_svf_bp2_6000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 1);
    pset_s(&mut matrix, va, "smode", 3);
    pset_d(&mut matrix, va, "freq", 6000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 4.0);

    // resonance 0.0
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);

    assert_vis_fft!(
        out,
        [
            (0, 12),
            (689, 12),
            (1378, 28),
            (2067, 32),
            (2756, 28),
            (3445, 48),
            (4134, 68),
            (4823, 56),
            (5513, 68),
            (6202, 76),
            (6891, 56),
            (7580, 76),
            (8269, 80),
            (8958, 76),
            (9647, 92),
            (10336, 80),
            (11025, 64),
            (11714, 44),
            (12403, 64),
            (13092, 76),
            (13781, 68),
            (14470, 52),
            (15159, 56),
            (15848, 56),
            (16538, 52),
            (17227, 48),
            (17916, 56),
            (18605, 40),
            (19294, 32),
            (19983, 32),
            (20672, 28),
            (21361, 24),
            (22050, 28)
        ]
    );

    // with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);
    assert_vis_fft!(
        out,
        [
            (1378, 12),
            (2067, 20),
            (2756, 32),
            (3445, 68),
            (4134, 72),
            (4823, 56),
            (5513, 68),
            (6202, 84),
            (6891, 96),
            (7580, 80),
            (8269, 88),
            (8958, 84),
            (9647, 60),
            (10336, 52),
            (11025, 56),
            (11714, 44),
            (12403, 48),
            (13092, 40),
            (13781, 32),
            (14470, 32),
            (15159, 32),
            (15848, 28),
            (16538, 36),
            (17227, 36),
            (17916, 20),
            (18605, 16),
            (19294, 20),
            (19983, 20),
            (20672, 20),
            (21361, 16),
            (22050, 20)
        ]
    );

    // with resonance = 0.8
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.8);
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);
    assert_vis_fft!(
        out,
        [
            (2067, 12),
            (2756, 16),
            (3445, 28),
            (4134, 28),
            (4823, 44),
            (5513, 56),
            (6202, 52),
            (6891, 52),
            (7580, 44),
            (8269, 40),
            (8958, 28),
            (9647, 28),
            (10336, 28),
            (11025, 20),
            (11714, 12),
            (12403, 12),
            (13092, 12),
            (13781, 16),
            (14470, 16),
            (15159, 12),
            (15848, 12),
            (16538, 12),
            (17227, 12)
        ]
    );
}

#[test]
fn check_node_fvafilt_svf_no_6000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 1);
    pset_s(&mut matrix, va, "smode", 4);
    pset_d(&mut matrix, va, "freq", 6000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 4.0);

    // resonance 0.0
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);

    assert_vis_fft!(
        out,
        [
            (0, 100),
            (689, 88),
            (1378, 84),
            (2067, 64),
            (2756, 40),
            (3445, 44),
            (4134, 48),
            (4823, 36),
            (5513, 24),
            (6202, 16),
            (8269, 16),
            (8958, 24),
            (9647, 36),
            (10336, 36),
            (11025, 36),
            (11714, 32),
            (12403, 48),
            (13092, 68),
            (13781, 64),
            (14470, 56),
            (15159, 64),
            (15848, 72),
            (16538, 68),
            (17227, 72),
            (17916, 84),
            (18605, 64),
            (19294, 52),
            (19983, 60),
            (20672, 60),
            (21361, 56),
            (22050, 64)
        ]
    );

    // with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);
    assert_vis_fft!(
        out,
        [
            (0, 84),
            (689, 76),
            (1378, 76),
            (2067, 76),
            (2756, 76),
            (3445, 100),
            (4134, 88),
            (4823, 40),
            (5513, 28),
            (6202, 24),
            (6891, 20),
            (7580, 24),
            (8269, 40),
            (8958, 56),
            (9647, 52),
            (10336, 60),
            (11025, 72),
            (11714, 60),
            (12403, 84),
            (13092, 72),
            (13781, 68),
            (14470, 72),
            (15159, 84),
            (15848, 72),
            (16538, 104),
            (17227, 108),
            (17916, 68),
            (18605, 60),
            (19294, 72),
            (19983, 80),
            (20672, 84),
            (21361, 72),
            (22050, 80)
        ]
    );

    // with resonance = 0.8
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.8);
    let out = fftr64_now_peaks(&mut node_exec, 4, 9);
    assert_vis_fft!(
        out,
        [
            (0, 96),
            (689, 56),
            (1378, 64),
            (2067, 64),
            (2756, 68),
            (3445, 80),
            (4134, 76),
            (4823, 76),
            (5513, 48),
            (6202, 36),
            (6891, 56),
            (7580, 60),
            (8269, 72),
            (8958, 64),
            (9647, 68),
            (10336, 80),
            (11025, 68),
            (11714, 52),
            (12403, 52),
            (13092, 60),
            (13781, 76),
            (14470, 96),
            (15159, 84),
            (15848, 92),
            (16538, 100),
            (17227, 88),
            (17916, 80),
            (18605, 80),
            (19294, 72),
            (19983, 76),
            (20672, 88),
            (21361, 80),
            (22050, 84)
        ]
    );
}

#[test]
fn check_node_fvafilt_sallenkey_1000hz() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 2);
    pset_d(&mut matrix, va, "freq", 1000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // resonance 0.0
    let out = fftr512_now_peaks(&mut node_exec, 3, 7);

    assert_vis_fft!(
        out,
        [
            (0, 45),
            (86, 36),
            (172, 42),
            (258, 36),
            (345, 54),
            (431, 39),
            (517, 27),
            (603, 30),
            (689, 42),
            (775, 33),
            (861, 27),
            (947, 33),
            (1034, 24),
            (1120, 33),
            (1206, 30),
            (1292, 15),
            (1378, 15),
            (1464, 18),
            (1550, 12),
            (1637, 15),
            (1723, 9),
            (2067, 9),
            (2239, 9)
        ]
    );

    // with resonance = 0.5
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 0.5);
    let out = fftr512_now_peaks(&mut node_exec, 3, 9);
    assert_vis_fft!(
        out,
        [
            (0, 24),
            (86, 39),
            (172, 54),
            (258, 63),
            (345, 66),
            (431, 30),
            (517, 39),
            (603, 39),
            (689, 63),
            (775, 57),
            (861, 60),
            (947, 39),
            (1034, 33),
            (1120, 36),
            (1206, 33),
            (1292, 39),
            (1378, 39),
            (1464, 21),
            (1550, 18),
            (1637, 24),
            (1723, 18),
            (1809, 21),
            (1895, 18),
            (1981, 9),
            (2067, 9),
            (2153, 12),
            (2239, 9),
            (2326, 9),
            (2670, 9)
        ]
    );

    // with resonance = 1.0
    pset_d_wait(&mut matrix, &mut node_exec, va, "res", 1.0);
    let out = fftr512_now_peaks(&mut node_exec, 3, 14);
    assert_vis_fft!(
        out,
        [
            (0, 75),
            (86, 63),
            (172, 51),
            (258, 66),
            (345, 72),
            (431, 60),
            (517, 69),
            (603, 93),
            (689, 114),
            (775, 171),
            (861, 279),
            (947, 207),
            (1034, 285),
            (1120, 213),
            (1206, 105),
            (1292, 78),
            (1378, 48),
            (1464, 69),
            (1550, 42),
            (1637, 21),
            (1723, 24),
            (1809, 24),
            (1895, 15),
            (1981, 18),
            (2584, 15)
        ]
    );
}

#[test]
fn check_node_fvafilt_sallenkey_sig_r() {
    let (mut matrix, mut node_exec) = setup_fvafilt_matrix_sig_r();

    let va = NodeId::FVaFilt(0);

    pset_s(&mut matrix, va, "ftype", 2);
    pset_d(&mut matrix, va, "freq", 1000.0);
    pset_d(&mut matrix, va, "res", 0.0);
    pset_d(&mut matrix, NodeId::Out(0), "vol", 8.0);
    pset_d_wait(&mut matrix, &mut node_exec, va, "drive", 1.0);

    // resonance 0.0
    let out = fftr512_now_peaks(&mut node_exec, 3, 7);

    assert_vis_fft!(
        out,
        [
            (0, 45),
            (86, 36),
            (172, 42),
            (258, 36),
            (345, 54),
            (431, 39),
            (517, 27),
            (603, 30),
            (689, 42),
            (775, 33),
            (861, 27),
            (947, 33),
            (1034, 24),
            (1120, 33),
            (1206, 30),
            (1292, 15),
            (1378, 15),
            (1464, 18),
            (1550, 12),
            (1637, 15),
            (1723, 9),
            (2067, 9),
            (2239, 9)
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
        .node_io("fvafilt", "in_l", "sig_l")
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
        .node_io("fvafilt", "in_r", "sig_r")
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
        .node_io("fvafilt", "in_r", "sig_r")
        .set_norm("drive", 1.0)
        .set_denorm("freq", 14000.0)
        .set_atom("ftype", SAtom::setting(0))
        .node_inp("out", "ch1")
        .set_denorm("vol", 0.3219)
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    run_for_ms(&mut node_exec, 2000.0);
    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 100.0);
    println!("{:#?}", rmsmima);
    assert_rmsmima!(rmsmima, (0.70314854, -0.8921491, 1.0319022));
}
