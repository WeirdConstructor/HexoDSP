// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use hexodsp::dsp::build::*;
mod common;

#[test]
fn check_basic_api() {
    let f = bosc(0).set().wtype(3).set().freq(440.0);
    let mix = mix3(0).input().ch1(&f.output().sig());
    let mix = mix.input().ch2(&f.output().sig());
    let filt = sfilter(0).input().inp(&mix.output().sig());


    println!("{:#?}", f.build());
    println!("{:#?}", filt.build());
    assert!(false);
}
