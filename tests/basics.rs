// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_matrix_sine() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(2);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(out).input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    let (mut out_l, out_r) = run_no_input(&mut node_exec, 4.0);

    let sum_l: f32 = out_l.iter().map(|v| v.abs()).sum();
    let sum_r: f32 = out_r.iter().map(|v| v.abs()).sum();
    assert_float_eq!(sum_l.floor(), 112301.0);
    assert_float_eq!(sum_r, 0.0);

    save_wav("check_matrix_sine.wav", &out_l);

    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 1000.0);
    for i in 0..4 {
        assert_float_eq!(rms_mimax[i].0, 0.5);
        assert_float_eq!(rms_mimax[i].1, -0.9999999);
        assert_float_eq!(rms_mimax[i].2, 0.9999999);
    }

    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 100, 0.0);
    assert_eq!(fft_res[0], (431, 248));
    assert_eq!(fft_res[1], (474, 169));

    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 100, 1000.0);
    assert_eq!(fft_res[0], (431, 248));
    assert_eq!(fft_res[1], (474, 169));

    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 100, 1500.0);
    assert_eq!(fft_res[0], (431, 248));
    assert_eq!(fft_res[1], (474, 169));

    let sin_led_val = matrix.led_value_for(&sin);
    let out_led_val = matrix.led_value_for(&out);

    assert_float_eq!(sin_led_val, 0.54018);
    assert_float_eq!(out_led_val, 0.54018);
}

#[test]
fn check_matrix_atom_set() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(2);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(out).input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    let mono_param = out.inp_param("mono").unwrap();

    matrix.set_param(mono_param, SAtom::setting(1));

    let (out_l, out_r) = run_no_input(&mut node_exec, 4.0);

    let sum_l: f32 = out_l.iter().map(|v| v.abs()).sum();
    let sum_r: f32 = out_r.iter().map(|v| v.abs()).sum();
    assert_float_eq!(sum_l.floor(), 112301.0);
    assert_float_eq!(sum_r.floor(), 112301.0);

    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 1000.0);
    for i in 0..4 {
        assert_float_eq!(rms_mimax[i].0, 0.5);
        assert_float_eq!(rms_mimax[i].1, -0.9999999);
        assert_float_eq!(rms_mimax[i].2, 0.9999999);
    }

    let rms_mimax = calc_rms_mimax_each_ms(&out_r[..], 1000.0);
    for i in 0..4 {
        assert_float_eq!(rms_mimax[i].0, 0.5);
        assert_float_eq!(rms_mimax[i].1, -0.9999999);
        assert_float_eq!(rms_mimax[i].2, 0.9999999);
    }
}

#[test]
fn check_sine_pitch_change() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(out).input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    let (mut out_l, _out_r) = run_no_input(&mut node_exec, 0.2);

    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 200, 0.0);
    assert_eq!(fft_res[0], (431, 248));

    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F64, 20, 100.0);
    assert_eq!(fft_res[0], (0, 22));

    let freq_param = sin.inp_param("freq").unwrap();

    matrix.set_param(freq_param, SAtom::param(freq_param.norm(4400.0)));

    let (mut out_l, _out_r) = run_no_input(&mut node_exec, 1.0);

    // Test at the start of the slope (~ 690 Hz):
    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F64, 15, 0.0);
    assert_eq!(fft_res[0], (0, 18));
    assert_eq!(fft_res[1], (689, 15));

    // In the middle (~ 2067 Hz):
    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F64, 10, 5.0);
    assert_eq!(fft_res[0], (1378, 14));
    assert_eq!(fft_res[1], (2067, 12));

    // Goal (~ 4134 Hz)
    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F64, 14, 10.0);
    assert_eq!(fft_res[0], (4134, 14));

    // Test the freq after the slope in high res (closer to 4400 Hz):
    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 200, 400.0);
    assert_eq!(fft_res[0], (4393, 251));
}

#[test]
fn check_detune_parameter() {
    let sin = NodeId::Sin(0);
    let det_param = sin.inp_param("det").unwrap();
    assert_float_eq!(det_param.norm(12.0), 0.1);
    assert_float_eq!(det_param.norm(-12.0), -0.1);
    assert_float_eq!(det_param.norm(24.0), 0.2);
    assert_float_eq!(det_param.norm(-24.0), -0.2);
}

#[test]
fn check_sine_freq_detune() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(out).input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    let det_param = sin.inp_param("det").unwrap();

    run_no_input(&mut node_exec, 50.0);

    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 440.0);

    matrix.set_param(freq_param, SAtom::param(freq_param.norm(4400.0)));
    run_no_input(&mut node_exec, 50.0);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 2000.0);
    assert_float_eq!(cfreq.floor(), 4400.0);

    matrix.set_param(freq_param, SAtom::param(freq_param.norm(50.0)));
    run_no_input(&mut node_exec, 50.0);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 50.0);

    matrix.set_param(freq_param, SAtom::param(freq_param.norm(440.0)));
    matrix.set_param(det_param, SAtom::param(det_param.norm(12.0)));
    run_no_input(&mut node_exec, 50.0);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 880.0);

    matrix.set_param(det_param, SAtom::param(det_param.norm(1.0)));
    run_no_input(&mut node_exec, 50.0);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 466.0);

    matrix.set_param(det_param, SAtom::param(det_param.norm(-1.0)));
    run_no_input(&mut node_exec, 50.0);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 2000.0);
    assert_float_eq!(cfreq.floor(), 415.0);

    matrix.set_param(det_param, SAtom::param(det_param.norm(-14.0)));
    run_no_input(&mut node_exec, 50.0);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 196.0);
}

#[test]
fn check_matrix_monitor() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(2);
    let out = NodeId::Out(0);
    matrix.place(
        0,
        0,
        Cell::empty(sin).input(sin.inp("freq"), sin.inp("freq"), sin.inp("freq")).out(
            sin.out("sig"),
            sin.out("sig"),
            sin.out("sig"),
        ),
    );
    matrix.place(1, 0, Cell::empty(out).input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    // Go to 220Hz
    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(-0.1));

    matrix.monitor_cell(*matrix.get(0, 0).unwrap());

    let (mut out_l, _out_r) = run_realtime_no_input(&mut node_exec, 0.3, true);

    // Give the MonitorProcessor some time to work on the buffers.
    std::thread::sleep(std::time::Duration::from_millis(100));

    //assert!(false);
    for i in 0..3 {
        let sl = matrix.get_minmax_monitor_samples(i);
        //d// println!("SL={:?}", sl);
        //d// println!("=> {}", i);

        assert_eq!((sl[sl.len() - 1].0 * 10000.0) as i64, -1000);
        assert_eq!((sl[sl.len() - 1].1 * 10000.0) as i64, -1000);
        assert_eq!((sl[sl.len() - 13].0 * 10000.0) as i64, -1000);
        // Here we see that the paramter is smoothed in:
        assert_eq!((sl[sl.len() - 14].1 * 10000.0) as i64, -2);
        assert_eq!((sl[sl.len() - 15].0 * 10000.0) as i64, 0);
        assert_eq!((sl[sl.len() - 15].1 * 10000.0) as i64, 0);
    }

    for i in 3..6 {
        let sl = matrix.get_minmax_monitor_samples(i);
        //d// println!("SL={:?}", sl);
        //d// println!("=> {}", i);

        assert_eq!((sl[sl.len() - 1].0 * 10000.0) as i64, -9999);
        assert_eq!((sl[sl.len() - 1].1 * 10000.0) as i64, 9999);
        assert_eq!((sl[sl.len() - 14].0 * 10000.0) as i64, -9999);
        assert_eq!((sl[sl.len() - 14].1 * 10000.0) as i64, 9999);
        assert_eq!((sl[sl.len() - 15].0 * 10000.0) as i64, 0);
        assert_eq!((sl[sl.len() - 15].1 * 10000.0) as i64, 0);
    }

    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 50.0);
    assert_float_eq!(rms_mimax[0].0, 0.49901);

    // let ta = std::time::Instant::now();

    // Test the freq after the slope in high res (closer to 4400 Hz):
    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 200, 50.0);

    // let ta = std::time::Instant::now().duration_since(ta);
    // println!("ta Elapsed: {:?}", ta);
    // assert!(false);

    // 220Hz is one Octave below 440Hz
    assert_eq!(fft_res[0], (215, 253));
}

#[test]
fn check_matrix_monitor_bug_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let amp = NodeId::Amp(1);
    matrix.place(0, 0, Cell::empty(sin).out(None, sin.out("sig"), None));
    matrix.place(
        1,
        0,
        Cell::empty(amp).out(None, None, amp.out("sig")).input(None, amp.inp("inp"), None),
    );
    matrix.sync().unwrap();

    matrix.monitor_cell(*matrix.get(1, 0).unwrap());

    let (_out_l, _out_r) = run_realtime_no_input(&mut node_exec, 0.2, true);

    std::thread::sleep(std::time::Duration::from_millis(100));

    for i in [0, 2, 3, 4].iter() {
        let sl = matrix.get_minmax_monitor_samples(*i);
        assert_eq!((sl[sl.len() - 1].0 * 10000.0) as i64, 0);
        assert_eq!((sl[sl.len() - 1].1 * 10000.0) as i64, 0);
    }

    for i in [1, 5].iter() {
        let sl = matrix.get_minmax_monitor_samples(*i);
        assert_eq!((sl[sl.len() - 1].0 * 10000.0) as i64, -9999);
        assert_eq!((sl[sl.len() - 1].1 * 10000.0) as i64, 9999);
    }
}

#[test]
fn check_matrix_out_config_bug1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
    matrix.place(
        1,
        0,
        Cell::empty(NodeId::Out(0)).input(None, Some(0), None).out(None, None, Some(0)),
    );

    matrix.place(0, 1, Cell::empty(NodeId::Sin(1)).out(None, Some(0), None));
    matrix.place(
        1,
        2,
        Cell::empty(NodeId::Sin(0)).input(None, Some(0), None).out(None, None, Some(0)),
    );
    matrix.place(
        1,
        1,
        Cell::empty(NodeId::Out(0)).input(Some(1), Some(0), None).out(None, None, Some(0)),
    );

    assert!(matrix.sync().is_err());

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

#[test]
fn check_matrix_out_config_bug1_reduced() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(
        1,
        0,
        Cell::empty(NodeId::Out(0)).input(Some(0), None, None).out(None, None, Some(0)),
    );
    matrix.place(
        1,
        2,
        Cell::empty(NodeId::Out(0)).input(Some(0), None, None).out(None, None, None),
    );

    matrix.sync().unwrap();

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

#[test]
fn check_matrix_out_config_bug1b_reduced() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(1, 0, Cell::empty(NodeId::Out(0)).out(None, None, Some(0)));
    matrix.place(1, 1, Cell::empty(NodeId::Out(0)).input(Some(0), None, None));

    assert!(matrix.sync().is_err());

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

#[test]
fn check_matrix_out_config_bug1c_reduced() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(1, 0, Cell::empty(NodeId::Sin(0)).out(None, None, Some(0)));
    matrix.place(1, 1, Cell::empty(NodeId::Out(0)).input(Some(9), None, None));

    matrix.sync().unwrap();

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

macro_rules! simple_sine_output_test {
    ($matrix: ident, $block: tt) => {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut $matrix = Matrix::new(node_conf, 7, 7);

        $block;

        $matrix.sync().unwrap();

        let (out_l, _out_r) = run_no_input(&mut node_exec, 0.2);

        let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 50.0);
        assert_float_eq!(rms_mimax[0].0, 0.5);
        assert_float_eq!(rms_mimax[0].1, -0.9999999);
        assert_float_eq!(rms_mimax[0].2, 0.9999999);
    };
}

#[test]
fn check_matrix_connect_even_top_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(1, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(2, 1, Cell::empty(NodeId::Out(0)).input(None, Some(0), None));
    });
}

#[test]
fn check_matrix_connect_even_bottom_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(1, 1, Cell::empty(NodeId::Sin(0)).out(Some(0), None, None));
        matrix.place(2, 1, Cell::empty(NodeId::Out(0)).input(None, None, Some(0)));
    });
}

#[test]
fn check_matrix_connect_even_top() {
    simple_sine_output_test!(matrix, {
        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, None, Some(0)));
        matrix.place(0, 1, Cell::empty(NodeId::Out(0)).input(Some(0), None, None));
    });
}

#[test]
fn check_matrix_connect_odd_top_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
        matrix.place(1, 0, Cell::empty(NodeId::Out(0)).input(None, Some(0), None));
    });
}

#[test]
fn check_matrix_connect_odd_bottom_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(0, 1, Cell::empty(NodeId::Sin(0)).out(Some(0), None, None));
        matrix.place(1, 0, Cell::empty(NodeId::Out(0)).input(None, None, Some(0)));
    });
}

#[test]
fn check_matrix_connect_odd_top() {
    simple_sine_output_test!(matrix, {
        matrix.place(1, 0, Cell::empty(NodeId::Sin(0)).out(None, None, Some(0)));
        matrix.place(1, 1, Cell::empty(NodeId::Out(0)).input(Some(0), None, None));
    });
}

#[test]
fn check_matrix_adj_odd() {
    let (node_conf, _node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    /*
            _____
        I2 / I1  \ O1
          /       \
          \       /
        I3 \_____/  O2
             O3

          0     1    2      3
         ___         ___
       0/   \  ___ 0/   \  ___
        \___/0/S2 \ \___/0/   \
         ___  \___/       \___/
       1/S1 \        ___
        \___/  ___ 1/S3 \  ___
         ___ 1/S0 \ \___/1/   \
       2/S6 \ \___/       \___/
        \___/        ___
               ___ 2/S4 \  ___
             2/S5 \ \___/2/   \
              \___/       \___/
    */

    matrix.place(
        1,
        1,
        Cell::empty(NodeId::Sin(0)).out(Some(0), Some(0), Some(0)).input(Some(0), Some(0), Some(0)),
    );

    matrix.place(0, 1, Cell::empty(NodeId::Sin(1)).out(None, Some(0), None));
    matrix.place(1, 0, Cell::empty(NodeId::Sin(2)).out(None, None, Some(0)));
    matrix.place(2, 1, Cell::empty(NodeId::Sin(3)).input(None, None, Some(0)));
    matrix.place(2, 2, Cell::empty(NodeId::Sin(4)).input(None, Some(0), None));
    matrix.place(1, 2, Cell::empty(NodeId::Sin(5)).input(Some(0), None, None));
    matrix.place(0, 2, Cell::empty(NodeId::Sin(6)).out(Some(0), None, None));
    matrix.sync().unwrap();

    assert_eq!(matrix.get_adjacent(1, 1, CellDir::B).unwrap().node_id(), NodeId::Sin(5));
    assert_eq!(matrix.get_adjacent(1, 1, CellDir::BR).unwrap().node_id(), NodeId::Sin(4));
    assert_eq!(matrix.get_adjacent(1, 1, CellDir::TR).unwrap().node_id(), NodeId::Sin(3));

    assert_eq!(matrix.get_adjacent(1, 1, CellDir::T).unwrap().node_id(), NodeId::Sin(2));
    assert_eq!(matrix.get_adjacent(1, 1, CellDir::TL).unwrap().node_id(), NodeId::Sin(1));
    assert_eq!(matrix.get_adjacent(1, 1, CellDir::BL).unwrap().node_id(), NodeId::Sin(6));
}

#[test]
fn check_matrix_adj_even() {
    let (node_conf, _node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    /*
            _____
        I2 / I1  \ O1
          /       \
          \       /
        I3 \_____/  O2
             O3

          0     1    2      3
         ___         ___
       0/   \  ___ 0/S2 \  ___
        \___/0/S1 \ \___/0/S3 \
         ___  \___/       \___/
       1/   \        ___
        \___/  ___ 1/S0 \  ___
         ___ 1/S6 \ \___/1/S4 \
       2/   \ \___/       \___/
        \___/        ___
               ___ 2/S5 \  ___
             2/   \ \___/2/   \
              \___/       \___/
    */

    matrix.place(
        2,
        1,
        Cell::empty(NodeId::Sin(0)).out(Some(0), Some(0), Some(0)).input(Some(0), Some(0), Some(0)),
    );

    matrix.place(1, 0, Cell::empty(NodeId::Sin(1)).out(None, Some(0), None));
    matrix.place(2, 0, Cell::empty(NodeId::Sin(2)).out(None, None, Some(0)));
    matrix.place(3, 0, Cell::empty(NodeId::Sin(3)).input(None, None, Some(0)));
    matrix.place(3, 1, Cell::empty(NodeId::Sin(4)).input(None, Some(0), None));
    matrix.place(2, 2, Cell::empty(NodeId::Sin(5)).input(Some(0), None, None));
    matrix.place(1, 1, Cell::empty(NodeId::Sin(6)).out(Some(0), None, None));
    matrix.sync().unwrap();

    assert_eq!(matrix.get_adjacent(2, 1, CellDir::B).unwrap().node_id(), NodeId::Sin(5));
    assert_eq!(matrix.get_adjacent(2, 1, CellDir::BR).unwrap().node_id(), NodeId::Sin(4));
    assert_eq!(matrix.get_adjacent(2, 1, CellDir::TR).unwrap().node_id(), NodeId::Sin(3));

    assert_eq!(matrix.get_adjacent(2, 1, CellDir::T).unwrap().node_id(), NodeId::Sin(2));
    assert_eq!(matrix.get_adjacent(2, 1, CellDir::TL).unwrap().node_id(), NodeId::Sin(1));
    assert_eq!(matrix.get_adjacent(2, 1, CellDir::BL).unwrap().node_id(), NodeId::Sin(6));
}

#[test]
fn check_matrix_out_twice_assignment() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(0, 0, Cell::empty(NodeId::Sin(0)).out(None, Some(0), None));
    matrix.place(0, 1, Cell::empty(NodeId::Sin(0)).out(Some(0), None, None));
    matrix.place(
        1,
        0,
        Cell::empty(NodeId::Out(0)).input(None, Some(0), Some(0)).out(None, None, None),
    );

    matrix.sync().unwrap();

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

#[test]
fn check_matrix_amp() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let amp = NodeId::Amp(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(amp).input(out.inp("ch1"), None, None).out(None, None, sin.out("sig")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let att_param = amp.inp_param("att").unwrap();
    matrix.set_param(att_param, SAtom::param(0.5));

    let (rms, _, _) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_float_eq!(rms, 0.031249225);

    matrix.set_param(att_param, SAtom::param(1.0));
    let (rms, _, _) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_float_eq!(rms, 0.49998704);

    matrix.set_param(att_param, SAtom::param(0.0));
    let (rms, _, _) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_float_eq!(rms, 0.0);

    let gain_param = amp.inp_param("gain").unwrap();

    matrix.set_param(att_param, SAtom::param(1.0));
    matrix.set_param(gain_param, SAtom::param(0.5));
    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_float_eq!(rms, 0.12499);
    assert_float_eq!(min, -0.5);
    assert_float_eq!(max, 0.5);
}

#[test]
fn check_matrix_clear() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(-0.2));

    let fft = run_and_get_fft4096(&mut node_exec, 800, 0.0);
    // slightly lower counts than later, because we have a slight
    // frequency slope after setting the frequency to 110Hz
    assert_eq!(fft[0], (108, 989));

    let fft = run_and_get_fft4096(&mut node_exec, 800, 10.0);
    assert_eq!(fft[0], (108, 993));

    matrix.clear();

    let fft = run_and_get_fft4096(&mut node_exec, 1, 50.0);
    assert_eq!(fft.len(), 0);

    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let fft = run_and_get_fft4096(&mut node_exec, 800, 50.0);
    assert_eq!(fft[0], (441, 1012));
}

#[test]
fn check_matrix_serialize() {
    {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        let sin = NodeId::Sin(0);
        let out = NodeId::Out(0);
        matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
        matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
        matrix.sync().unwrap();

        let freq_param = sin.inp_param("freq").unwrap();
        matrix.set_param(freq_param, SAtom::param(-0.2));

        let fft = run_and_get_fft4096(&mut node_exec, 800, 10.0);
        assert_eq!(fft[0], (108, 993));

        hexodsp::save_patch_to_file(&mut matrix, "check_matrix_serialize.hxy").unwrap();
    }

    {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        hexodsp::load_patch_from_file(&mut matrix, "check_matrix_serialize.hxy").unwrap();

        let fft = run_and_get_fft4096(&mut node_exec, 800, 10.0);
        assert_eq!(fft[0], (108, 993));
    }
}

#[test]
fn check_matrix_tseq() {
    use hexodsp::dsp::tracker::UIPatternModel;

    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let tsq = NodeId::TSeq(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(tsq).input(tsq.inp("clock"), None, None).out(None, None, tsq.out("trk1")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, sin, "freq", -0.978);
    pset_s(&mut matrix, tsq, "cmode", 1);

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.lock().unwrap();
        pr.set_rows(16);
        pr.set_cell_value(0, 0, 0xFFF);
        pr.set_cell_value(15, 0, 0x000);
    }

    for _ in 0..10 {
        matrix.check_pattern_data(0);
    }

    // We let the clock mode tune in:
    run_and_undersample(&mut node_exec, 10000.0, 1);

    // Take some real samples:
    let samples = run_and_undersample(&mut node_exec, 2000.0, 10);

    assert_vec_feq!(
        samples,
        vec![
            0.5322106,
            0.4255343,
            0.318858,
            0.21218172,
            0.105505496,
            0.017571526,
            // then start at the beginning:
            0.958819,
            0.8521427,
            0.7454664,
            0.63879013
        ]
    );

    // switch to row trigger:
    pset_s(&mut matrix, tsq, "cmode", 0);
    let samples = run_and_undersample(&mut node_exec, 2000.0, 5);

    assert_vec_feq!(samples, vec![0.5011433, 0.7011613, 0.9011793, 0.9932535, 0.97991896]);

    // set to phase mode:
    pset_s(&mut matrix, tsq, "cmode", 2);
    let samples = run_and_undersample(&mut node_exec, 1000.0, 5);

    assert_float_eq!(samples[0], 0.2491);
    assert_float_eq!(samples[1], 0.0026);
    assert_float_eq!(samples[2], 0.1616);
    assert_float_eq!(samples[3], 0.6655);
    assert_float_eq!(samples[4], 0.8104);
}

#[test]
fn check_matrix_tseq_trig() {
    use hexodsp::dsp::tracker::UIPatternModel;

    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let tsq = NodeId::TSeq(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(tsq).input(tsq.inp("clock"), None, None).out(None, None, tsq.out("trk1")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    pset_n(&mut matrix, sin, "freq", -0.978);
    pset_s(&mut matrix, tsq, "cmode", 1);

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.lock().unwrap();
        pr.set_rows(16);
        pr.set_cell_value(0, 0, 0xFFF);
        pr.set_cell_value(15, 0, 0x000);
    }

    for _ in 0..10 {
        matrix.check_pattern_data(0);
    }

    // We let the clock mode tune in:
    run_and_undersample(&mut node_exec, 10000.0, 1);

    // Take some real samples:
    let samples = run_and_undersample(&mut node_exec, 2000.0, 10);

    assert_vec_feq!(
        samples,
        vec![
            0.5322106,
            0.4255343,
            0.318858,
            0.21218172,
            0.105505496,
            0.017571526,
            0.958819,
            0.8521427,
            0.7454664,
            0.63879013
        ]
    );

    pset_n(&mut matrix, tsq, "trig", 1.0);

    // Take some real samples:
    let samples = run_and_undersample(&mut node_exec, 2000.0, 10);

    assert_vec_feq!(
        samples,
        vec![
            0.5321138,
            // Then trigger happens:
            0.96263915,
            0.8559629,
            0.74928665,
            0.6426103,
            0.53593403,
            0.42925775,
            0.32258147,
            0.21590519,
            0.109228894
        ]
    );
}

#[test]
fn check_matrix_tseq_gate() {
    use hexodsp::dsp::tracker::UIPatternModel;

    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let tsq = NodeId::TSeq(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(tsq).input(tsq.inp("clock"), None, None).out(None, None, tsq.out("trk1")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(-0.978));
    let cmode_param = tsq.inp_param("cmode").unwrap();
    matrix.set_param(cmode_param, SAtom::setting(1));

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.lock().unwrap();
        pr.set_rows(16);
        pr.set_col_gate_type(0);
        // pulse_width:
        //      0xF  - Gate is on for full row
        //      0x0  - Gate is on for a very short burst
        // row_div:
        //      0xF  - Row has 1  Gate
        //      0x0  - Row is divided up into 16 Gates
        // probability:
        //      0xF  - Row is always triggered
        //      0x7  - Row fires only in 50% of the cases
        //      0x0  - Row fires only in ~6% of the cases
        pr.set_cell_value(5, 0, 0xFFF);
        pr.set_cell_value(7, 0, 0xFF0);
        pr.set_cell_value(9, 0, 0xF00);
    }

    for _ in 0..10 {
        matrix.check_pattern_data(0);
    }

    // We let the clock mode tune in:
    run_and_undersample(&mut node_exec, 11100.0, 1);

    // Take some real samples:
    let samples = run_and_undersample(&mut node_exec, 2000.0, 2000);
    let changes = collect_gates(&samples[..]);

    assert_eq!(
        changes,
        vec![
            (524, 126),
            (775, 8),
            (1033, 1),
            (1041, 1),
            (1049, 1),
            (1080, 1),
            (1088, 1),
            (1119, 1),
            (1127, 1),
            (1135, 1)
        ]
    );
}

#[test]
fn check_matrix_tseq_2col_gate_bug() {
    use hexodsp::dsp::tracker::UIPatternModel;

    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let tsq = NodeId::TSeq(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(tsq).input(tsq.inp("clock"), None, None).out(None, None, tsq.out("trk2")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(0.0));

    let cmode_param = tsq.inp_param("cmode").unwrap();
    matrix.set_param(cmode_param, SAtom::setting(1));

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.lock().unwrap();
        pr.set_rows(2);
        pr.set_col_value_type(0);
        pr.set_col_gate_type(1);

        // pulse_width:
        //      0xF  - Gate is on for full row
        //      0x0  - Gate is on for a very short burst
        // row_div:
        //      0xF  - Row has 1  Gate
        //      0x0  - Row is divided up into 16 Gates
        // probability:
        //      0xF  - Row is always triggered
        //      0x7  - Row fires only in 50% of the cases
        //      0x0  - Row fires only in ~6% of the cases
        pr.set_cell_value(0, 0, 0xFFF);
        pr.set_cell_value(1, 0, 0x000);

        pr.set_cell_value(0, 1, 0x0FF);
        pr.set_cell_value(1, 1, 0x000);
    }

    for _ in 0..10 {
        matrix.check_pattern_data(0);
    }

    let samples = run_and_undersample(&mut node_exec, 10000.0, 100000);

    let mut any_non_zero = false;
    for s in samples.iter() {
        if *s > 0.0 {
            any_non_zero = true;
        }
    }

    assert!(any_non_zero);
}

#[test]
fn check_matrix_output_feedback() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let amp = NodeId::Amp(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(amp).input(amp.inp("inp"), None, None));
    matrix.sync().unwrap();

    let gain_p = amp.inp_param("gain").unwrap();
    matrix.set_param(gain_p, SAtom::param(0.25));

    for _ in 0..10 {
        node_exec.test_run(0.11, true);
        matrix.update_filters();
        matrix.filtered_out_fb_for(&sin, sin.out("sig").unwrap());
        matrix.filtered_out_fb_for(&amp, amp.out("sig").unwrap());
    }

    let o_sin = matrix.out_fb_for(&sin, sin.out("sig").unwrap()).unwrap();
    let o_amp = matrix.out_fb_for(&amp, amp.out("sig").unwrap()).unwrap();
    let fo_sin = matrix.filtered_out_fb_for(&sin, sin.out("sig").unwrap());
    let fo_amp = matrix.filtered_out_fb_for(&amp, amp.out("sig").unwrap());

    assert_float_eq!(o_sin, -0.061266);
    assert_float_eq!(o_amp, -0.007658);

    assert_float_eq!(fo_sin.0, 0.96846);
    assert_float_eq!(fo_sin.1, 0.9302191);
    assert_float_eq!(fo_amp.0, 0.12105);
    assert_float_eq!(fo_amp.1, 0.11627);
}

#[test]
fn check_matrix_node_feedback() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    let sin = NodeId::Sin(0);
    let sin2 = NodeId::Sin(1);
    let wr = NodeId::FbWr(0);
    let rd = NodeId::FbRd(0);
    let wr2 = NodeId::FbWr(1);
    let rd2 = NodeId::FbRd(1);
    let out = NodeId::Out(0);

    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(wr).input(wr.inp("inp"), None, None));
    matrix.place(1, 0, Cell::empty(rd).out(None, None, rd.out("sig")));
    matrix.place(1, 1, Cell::empty(out).input(out.inp("ch1"), None, None));

    matrix.place(0, 2, Cell::empty(sin2).out(None, None, sin2.out("sig")));
    matrix.place(0, 3, Cell::empty(wr2).input(wr2.inp("inp"), None, None));
    matrix.place(1, 2, Cell::empty(rd2).out(None, None, rd2.out("sig")));
    matrix.place(1, 3, Cell::empty(out).input(out.inp("ch2"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin2.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(freq_param.norm(880.0)));

    let (out_l, out_r) = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(
        out_l,
        15,
        vec![
            // The initial zeros are the feedback delays:
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
            0.68328893,
            0.9925844,
            0.48698083,
            -0.4184115,
            -0.9803018,
            -0.73738277,
            0.110905044,
            0.8681419,
            0.9126584,
            0.20790927,
            -0.6675302,
            -0.99494797,
            -0.50553185,
            0.39891028,
            0.97586703,
            0.7516482,
            -0.089641616,
            -0.8573498,
            -0.9211795,
            -0.22875604
        ]
    );
    assert_decimated_feq!(
        out_r,
        15,
        vec![
            // The initial zeros are the feedback delays:
            // The frequency will be established a bit later because
            // the parameter setting of 880 Hz will be smoothed:
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
            0.8791775,
            0.8898413,
            0.10330327,
            -0.79178804,
            -0.92698133,
            -0.11967586,
            0.8259115,
            0.86836,
            -0.09246742,
            -0.9534301,
            -0.62676203,
            0.5235326,
            0.9718173,
            0.04517236,
            -0.9560416,
            -0.49554884,
            0.7601789,
            0.75973713,
            -0.5529301,
            -0.8783003
        ]
    );

    // Let the frequency settle...
    run_for_ms(&mut node_exec, 80.0);

    let (mut out_l, mut out_r) = run_for_ms(&mut node_exec, 50.0);
    let fft_res_l = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 100, 0.0);
    assert_eq!(fft_res_l[0], (431, 245));
    assert_eq!(fft_res_l[1], (474, 170));

    let fft_res_r = fft_thres_at_ms(&mut out_r[..], FFT::F1024, 100, 0.0);
    assert_eq!(fft_res_r[0], (861, 224));
    assert_eq!(fft_res_r[1], (904, 206));
}

#[test]
fn check_matrix_tseq_perf() {
    use hexodsp::dsp::tracker::UIPatternModel;

    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let tsq = NodeId::TSeq(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(tsq).input(tsq.inp("clock"), None, None).out(None, None, tsq.out("trk1")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    //    matrix.set_param(freq_param, SAtom::param(-0.978));
    matrix.set_param(freq_param, SAtom::param(0.0));
    let cmode_param = tsq.inp_param("cmode").unwrap();
    matrix.set_param(cmode_param, SAtom::setting(0));
    //    matrix.set_param(cmode_param, SAtom::setting(2));

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.lock().unwrap();
        pr.set_rows(16);
        pr.set_col_note_type(0);
        pr.set_col_gate_type(1);
        pr.set_col_gate_type(2);

        pr.set_cell_value(0, 0, 0x0F7);
        pr.set_cell_value(4, 0, 0x100);
        pr.set_cell_value(8, 0, 0x10F);
        pr.set_cell_value(12, 0, 0x0F7);

        pr.set_cell_value(0, 1, 0xFF1);
        pr.set_cell_value(4, 1, 0xFF1);
        pr.set_cell_value(8, 1, 0xFF1);
        pr.set_cell_value(12, 1, 0xFF1);

        pr.set_cell_value(0, 2, 0xFF1);
        pr.set_cell_value(2, 2, 0xFF1);
        pr.set_cell_value(4, 2, 0xFF1);
        pr.set_cell_value(6, 2, 0xFF1);
        pr.set_cell_value(8, 2, 0xFF1);
        pr.set_cell_value(10, 2, 0xFF1);
        pr.set_cell_value(12, 2, 0xFF1);
        pr.set_cell_value(14, 2, 0xFF1);
    }

    for _ in 0..100 {
        matrix.check_pattern_data(0);
    }

    let mut prev: i64 = 0;
    let mut first: i64 = 0;
    for _ in 0..10 {
        let ta = std::time::Instant::now();
        run_for_ms(&mut node_exec, 10000.0);
        let dur = std::time::Instant::now().duration_since(ta);
        if prev > 0 {
            let now = dur.as_millis() as i64;
            if first <= 0 {
                first = now;
            }

            //d// println!("{},{}", prev, now);
            assert!((first - now).abs() < (first / 2));
        }
        prev = dur.as_millis() as i64;
    }
}
