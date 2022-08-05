// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;
#[cfg(feature="synfx-dsp-jit")]
use hexodsp::wblockdsp::*;

#[cfg(feature="synfx-dsp-jit")]
#[test]
fn check_wblockdsp_init() {
    let mut engine = CodeEngine::new();

    let backend = engine.get_backend();
}

#[cfg(feature="synfx-dsp-jit")]
#[test]
fn check_wblockdsp_code_node() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("code", "sig")
        .node_io("code", "in1", "sig")
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain
        .node_out("code", "sig")
        .node_io("code", "in1", "sig")
        .node_inp("out", "ch1")
        .place(&mut matrix, 0, 0)
        .unwrap();
    matrix.sync().unwrap();
}
