// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

use hexodsp::dsp::helpers::Quantizer;
use hexodsp::d_pit;

#[test]
fn check_quant_1() {
    let mut q = Quantizer::new();
    q.set_keys(0x0);

    let v =
        (0..=12).map(|i|
            d_pit!(
                q.process(
                    i as f32 * (0.1 / 12.0)))
        ).collect::<Vec<f32>>();

    assert_vec_feq!(v, vec![
        440.0,
        466.1638,
        493.8833,
        523.2511,
        554.3653,
        587.3295,
        622.25397,
        659.2551,
        698.4565,
        739.98883,
        783.9909,
        830.6094,
        880.0
    ]);

    let v =
        (0..=12).map(|i|
            d_pit!(q.process(i as f32 * (-0.1 / 12.0)))
        ).collect::<Vec<f32>>();

    assert_vec_feq!(v, vec![
        440.0,
        415.3047,
        391.99542,
        369.99442,
        349.22824,
        329.62756,
        311.12698,
        293.66476,
        277.18265,
        261.62555,
        246.94165,
        233.08186,
        220.0
    ]);
}
