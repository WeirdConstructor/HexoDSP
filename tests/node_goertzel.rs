// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

//WRITEME: setup sin into gzfilt, assert that gzfilt at matched freq has const output, and at unmatched has no output
fn setup_gnode_matrix() -> (Matrix, NodeExecutor) {
    let (node_conf, node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let goertzel = NodeId::GzFilt(0);
    let sin    = NodeId::Sin(0);
    let out   = NodeId::Out(0);
    
    matrix.place(0, 0, Cell::empty(sin)
                        //    Top   TopLeft BottomLeft
                       .input(None, None,   None)
                       // TopRight  BottomRight     Bottom
                       .out(None,   sin.out("sig"), None));

    matrix.place(1, 0, Cell::empty(goertzel)
                       .input(None, goertzel.inp("inp"), None)
                       .out(None, goertzel.out("sig"), None));

    matrix.place(2, 1, Cell::empty(out)
                       .input(None, out.inp("ch1"), None));

    matrix.sync().unwrap();

    (matrix, node_exec)
}

//WRITEME: expect signal to be > 0.2 for 880, change goertzel param to 600 target -> expect it to be < 0.2
#[test]
fn check_node_goertzel() {
    let (matrix, mut node_exec) = setup_gnode_matrix();

    let fft = run_and_get_fft4096_now(&mut node_exec, 500);
    // nice formatted debug printing, execute with:
    //    cargo test goert -- --nocapture
    eprintln!("FFT: {:#?}", fft);
    assert!(fft[0].0 == 0); // it should be a const value so a dom frequency should be 0

    let (out_l, _) = run_for_ms(&mut node_exec, 25.0);
    let rms_minmax = calc_rms_mimax_each_ms(&out_l[..], 10.0);
    eprintln!("RMS: {:?}", rms_minmax);
    assert!(rms_minmax[1].2 - rms_minmax[1].1 < 0.01); // the output should be const for const freq input

}