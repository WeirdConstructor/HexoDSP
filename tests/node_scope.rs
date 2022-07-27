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
        let (mi, ma) = handle.read(sig_idx, i);
        min.push(mi);
        max.push(ma);
        total_min = total_min.min(mi);
        total_max = total_max.max(ma);
    }

    (min, max, total_min, total_max)
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

        let (minv, maxv, min, max) = read_scope_buf(&matrix, sig_idx);
        assert_decimated_feq!(minv, 80, vec![0.0022, 0.1836, 0.3650, 0.5464, 0.7278, 0.9093, 1.0]);
        assert_decimated_feq!(maxv, 80, vec![0.0022, 0.1836, 0.3650, 0.5464, 0.7278, 0.9093, 1.0]);
        assert_float_eq!(min, 0.0);
        assert_float_eq!(max, 1.0);
    }
}
