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
            d_pit!(q.process(i as f32 * (0.1 / 12.0)))
        ).collect::<Vec<f32>>();

    assert_vec_feq!(v, vec![
        440.0,
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
    ]);
}
