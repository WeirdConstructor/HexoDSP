// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use hexodsp::build::*;
use hexodsp::synth_constructor::SynthConstructor;
mod common;
use common::*;

fn build_basic_api_test_graph() -> Out {
    let f = bosc(0).set().wtype(3).set().freq(440.0);
    let mix = mix3(0).set().ovol(0.39839).input().ch1(&f.output().sig());
    let mix = mix.input().ch2(&f.output().sig());
    let filt = sfilter(0).input().inp(&mix.output().sig());
    out(0).input().ch1(&filt.output().sig())
}

fn check_basic_api_rmsbefore_after<F: FnOnce()>(exec: &mut NodeExecutor, mut f: F) {
    let rmsmima = run_and_get_l_rms_mimax(exec, 100.0);
    assert_rmsmima!(rmsmima, (0.64348, -1.0887, 1.05413));
    f();
    let rmsmima = run_and_get_l_rms_mimax(exec, 100.0);
    assert_rmsmima!(rmsmima, (0.5083, -0.9676, 0.9369));
}

#[test]
fn check_basic_api() {
    let f = bosc(0).set().wtype(3).set().freq(440.0);
    let mix = mix3(0).set().ovol(0.39839).input().ch1(&f.output().sig());
    let mix = mix.input().ch2(&f.output().sig());
    let filt = sfilter(0).input().inp(&mix.output().sig());

    let mut sc = SynthConstructor::new();

    //d// println!("{:#?}", f.build());
    //d// println!("{:#?}", filt.build());
    let mut exec = sc.executor().unwrap();

    // Upload the graph:
    sc.upload(&out(0).input().ch1(&filt.output().sig())).unwrap();

    check_basic_api_rmsbefore_after(&mut exec, || {
        mix.set_mod().ch2(0.0, 0.7776);
        assert!(sc.update_params(&out(0).input().ch1(&filt.output().sig())).unwrap());
    });
}

#[test]
fn check_basic_api_update_params() {
    let graph = build_basic_api_test_graph();

    let mut sc = SynthConstructor::new();
    sc.upload(&graph).unwrap();

    let mut exec = sc.executor().unwrap();
    check_basic_api_rmsbefore_after(&mut exec, || {
        let updated_graph = sc.update_params(&mix3(0).set_mod().ch2(0.0, 0.7776)).unwrap();
        assert!(updated_graph);
    });
}

#[test]
fn check_basic_api_update_params_no_graph() {
    let graph = build_basic_api_test_graph();

    let mut sc = SynthConstructor::new();
    sc.upload(&graph).unwrap();

    let mut exec = sc.executor().unwrap();
    println!("START");
    check_basic_api_rmsbefore_after(&mut exec, || {
        // Change the graph a bit:
        let updated_graph = sc.update_params(&mix3(0).set().vol2(0.7776)).unwrap();
        assert!(!updated_graph);
    });
}
