mod common;
use common::*;

#[test]
fn check_matrix_sine() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(2);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(out)
                       .input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    let (mut out_l, out_r) = run_no_input(&mut node_exec, 4.0);

    let sum_l : f32 = out_l.iter().map(|v| v.abs()).sum();
    let sum_r : f32 = out_r.iter().map(|v| v.abs()).sum();
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

    assert_float_eq!(sin_led_val, -0.057622954);
    assert_float_eq!(out_led_val, -0.057622954);
}

#[test]
fn check_matrix_atom_set() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(2);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(out)
                       .input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    let mono_param = out.inp_param("mono").unwrap();

    matrix.set_param(mono_param, SAtom::setting(1));

    let (out_l, out_r) = run_no_input(&mut node_exec, 4.0);

    let sum_l : f32 = out_l.iter().map(|v| v.abs()).sum();
    let sum_r : f32 = out_r.iter().map(|v| v.abs()).sum();
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

    let sin = NodeId::Sin(2);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(out)
                       .input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    let (mut out_l, _out_r) = run_no_input(&mut node_exec, 0.2);

    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F1024, 200, 0.0);
    assert_eq!(fft_res[0], (431, 248));

    let fft_res = fft_thres_at_ms(&mut out_l[..], FFT::F64, 20, 100.0);
    assert_eq!(fft_res[0], (0, 22));

    let freq_param = sin.inp_param("freq").unwrap();

    matrix.set_param(
        freq_param,
        SAtom::param(freq_param.norm(4400.0)));

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
fn check_matrix_monitor() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(2);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .input(sin.inp("freq"), sin.inp("freq"), sin.inp("freq"))
                       .out(sin.out("sig"), sin.out("sig"), sin.out("sig")));
    matrix.place(1, 0, Cell::empty(out)
                       .input(None, out.inp("ch1"), None));
    matrix.sync().unwrap();

    // Go to 220Hz
    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(-0.1));

    matrix.monitor_cell(*matrix.get(0, 0).unwrap());

    let (mut out_l, _out_r) =
        run_realtime_no_input(&mut node_exec, 0.2, true);

    // Give the MonitorProcessor some time to work on the buffers.
    std::thread::sleep(std::time::Duration::from_millis(100));

//assert!(false);
    for i in 0..3 {
        let sl = matrix.get_minmax_monitor_samples(i);
        //d// println!("SL={:?}", sl);
        //d// println!("=> {}", i);

        assert_eq!((sl[sl.len() - 1].0  * 10000.0) as i64, -1000);
        assert_eq!((sl[sl.len() - 1].1  * 10000.0) as i64, -1000);
        assert_eq!((sl[sl.len() - 11].0 * 10000.0) as i64, -1000);
        // Here we see that the paramter is smoothed in:
        assert_eq!((sl[sl.len() - 11].1 * 10000.0) as i64,    -2);
        assert_eq!((sl[sl.len() - 12].0 * 10000.0) as i64,     0);
        assert_eq!((sl[sl.len() - 12].1 * 10000.0) as i64,     0);
    }

    for i in 3..6 {
        let sl = matrix.get_minmax_monitor_samples(i);
        //d// println!("SL={:?}", sl);
        //d// println!("=> {}", i);

        assert_eq!((sl[sl.len() - 1].0  * 10000.0) as i64, -9999);
        assert_eq!((sl[sl.len() - 1].1  * 10000.0) as i64,  9999);
        assert_eq!((sl[sl.len() - 11].0 * 10000.0) as i64, -9999);
        assert_eq!((sl[sl.len() - 11].1 * 10000.0) as i64,  9999);
        assert_eq!((sl[sl.len() - 12].0 * 10000.0) as i64,     0);
        assert_eq!((sl[sl.len() - 12].1 * 10000.0) as i64,     0);
    }

    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 50.0);
    assert_float_eq!(rms_mimax[0].0, 0.5013241);

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
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, sin.out("sig"), None));
    matrix.place(1, 0, Cell::empty(amp)
                       .out(None, None, amp.out("sig"))
                       .input(None, amp.inp("inp"), None));
    matrix.sync().unwrap();

    matrix.monitor_cell(*matrix.get(1, 0).unwrap());

    let (_out_l, _out_r) =
        run_realtime_no_input(&mut node_exec, 0.2, true);

    std::thread::sleep(std::time::Duration::from_millis(100));

    for i in [0, 2, 3, 4].iter() {
        let sl = matrix.get_minmax_monitor_samples(*i);
        assert_eq!((sl[sl.len() - 1].0  * 10000.0) as i64, 0);
        assert_eq!((sl[sl.len() - 1].1  * 10000.0) as i64, 0);
    }

    for i in [1, 5].iter() {
        let sl = matrix.get_minmax_monitor_samples(*i);
        assert_eq!((sl[sl.len() - 1].0  * 10000.0) as i64, -9999);
        assert_eq!((sl[sl.len() - 1].1  * 10000.0) as i64, 9999);
    }
}

#[test]
fn check_matrix_out_config_bug1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(0, 0, Cell::empty(NodeId::Sin(0))
                       .out(None, Some(0), None));
    matrix.place(1, 0, Cell::empty(NodeId::Out(0))
                       .input(None, Some(0), None)
                       .out(None, None, Some(0)));

    matrix.place(0, 1, Cell::empty(NodeId::Sin(1))
                       .out(None, Some(0), None));
    matrix.place(1, 2, Cell::empty(NodeId::Sin(0))
                       .input(None, Some(0), None)
                       .out(None, None, Some(0)));
    matrix.place(1, 1, Cell::empty(NodeId::Out(0))
                       .input(Some(1), Some(0), None)
                       .out(None, None, Some(0)));

    assert!(matrix.sync().is_err());

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

#[test]
fn check_matrix_out_config_bug1_reduced() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(1, 0, Cell::empty(NodeId::Out(0))
                       .input(Some(0), None, None)
                       .out(None, None, Some(0)));
    matrix.place(1, 2, Cell::empty(NodeId::Out(0))
                       .input(Some(0), None, None)
                       .out(None, None, None));

    matrix.sync().unwrap();

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

#[test]
fn check_matrix_out_config_bug1b_reduced() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(1, 0, Cell::empty(NodeId::Out(0))
                       .out(None, None, Some(0)));
    matrix.place(1, 1, Cell::empty(NodeId::Out(0))
                       .input(Some(0), None, None));

    assert!(matrix.sync().is_err());

    let (_out_l, _out_r) = run_no_input(&mut node_exec, 0.2);
}

#[test]
fn check_matrix_out_config_bug1c_reduced() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(1, 0, Cell::empty(NodeId::Sin(0))
                       .out(None, None, Some(0)));
    matrix.place(1, 1, Cell::empty(NodeId::Out(0))
                       .input(Some(9), None, None));

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
    }
}

#[test]
fn check_matrix_connect_even_top_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(1, 0, Cell::empty(NodeId::Sin(0))
                           .out(None, Some(0), None));
        matrix.place(2, 1, Cell::empty(NodeId::Out(0))
                           .input(None, Some(0), None));
    });
}


#[test]
fn check_matrix_connect_even_bottom_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(1, 1, Cell::empty(NodeId::Sin(0))
                           .out(Some(0), None, None));
        matrix.place(2, 1, Cell::empty(NodeId::Out(0))
                           .input(None, None, Some(0)));
    });
}

#[test]
fn check_matrix_connect_even_top() {
    simple_sine_output_test!(matrix, {
        matrix.place(0, 0, Cell::empty(NodeId::Sin(0))
                           .out(None, None, Some(0)));
        matrix.place(0, 1, Cell::empty(NodeId::Out(0))
                           .input(Some(0), None, None));
    });
}

#[test]
fn check_matrix_connect_odd_top_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(0, 0, Cell::empty(NodeId::Sin(0))
                           .out(None, Some(0), None));
        matrix.place(1, 0, Cell::empty(NodeId::Out(0))
                           .input(None, Some(0), None));
    });
}

#[test]
fn check_matrix_connect_odd_bottom_left() {
    simple_sine_output_test!(matrix, {
        matrix.place(0, 1, Cell::empty(NodeId::Sin(0))
                           .out(Some(0), None, None));
        matrix.place(1, 0, Cell::empty(NodeId::Out(0))
                           .input(None, None, Some(0)));
    });
}

#[test]
fn check_matrix_connect_odd_top() {
    simple_sine_output_test!(matrix, {
        matrix.place(1, 0, Cell::empty(NodeId::Sin(0))
                           .out(None, None, Some(0)));
        matrix.place(1, 1, Cell::empty(NodeId::Out(0))
                           .input(Some(0), None, None));
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

    matrix.place(1, 1, Cell::empty(NodeId::Sin(0))
                       .out(Some(0), Some(0), Some(0))
                       .input(Some(0), Some(0), Some(0)));

    matrix.place(0, 1, Cell::empty(NodeId::Sin(1))
                       .out(None, Some(0), None));
    matrix.place(1, 0, Cell::empty(NodeId::Sin(2))
                       .out(None, None, Some(0)));
    matrix.place(2, 1, Cell::empty(NodeId::Sin(3))
                       .input(None, None, Some(0)));
    matrix.place(2, 2, Cell::empty(NodeId::Sin(4))
                       .input(None, Some(0), None));
    matrix.place(1, 2, Cell::empty(NodeId::Sin(5))
                       .input(Some(0), None, None));
    matrix.place(0, 2, Cell::empty(NodeId::Sin(6))
                       .out(Some(0), None, None));
    matrix.sync().unwrap();

    assert_eq!(
        matrix.get_adjacent(1, 1, CellDir::B).unwrap().node_id(),
        NodeId::Sin(5));
    assert_eq!(
        matrix.get_adjacent(1, 1, CellDir::BR).unwrap().node_id(),
        NodeId::Sin(4));
    assert_eq!(
        matrix.get_adjacent(1, 1, CellDir::TR).unwrap().node_id(),
        NodeId::Sin(3));

    assert_eq!(
        matrix.get_adjacent(1, 1, CellDir::T).unwrap().node_id(),
        NodeId::Sin(2));
    assert_eq!(
        matrix.get_adjacent(1, 1, CellDir::TL).unwrap().node_id(),
        NodeId::Sin(1));
    assert_eq!(
        matrix.get_adjacent(1, 1, CellDir::BL).unwrap().node_id(),
        NodeId::Sin(6));
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

    matrix.place(2, 1, Cell::empty(NodeId::Sin(0))
                       .out(Some(0), Some(0), Some(0))
                       .input(Some(0), Some(0), Some(0)));

    matrix.place(1, 0, Cell::empty(NodeId::Sin(1))
                       .out(None, Some(0), None));
    matrix.place(2, 0, Cell::empty(NodeId::Sin(2))
                       .out(None, None, Some(0)));
    matrix.place(3, 0, Cell::empty(NodeId::Sin(3))
                       .input(None, None, Some(0)));
    matrix.place(3, 1, Cell::empty(NodeId::Sin(4))
                       .input(None, Some(0), None));
    matrix.place(2, 2, Cell::empty(NodeId::Sin(5))
                       .input(Some(0), None, None));
    matrix.place(1, 1, Cell::empty(NodeId::Sin(6))
                       .out(Some(0), None, None));
    matrix.sync().unwrap();

    assert_eq!(
        matrix.get_adjacent(2, 1, CellDir::B).unwrap().node_id(),
        NodeId::Sin(5));
    assert_eq!(
        matrix.get_adjacent(2, 1, CellDir::BR).unwrap().node_id(),
        NodeId::Sin(4));
    assert_eq!(
        matrix.get_adjacent(2, 1, CellDir::TR).unwrap().node_id(),
        NodeId::Sin(3));

    assert_eq!(
        matrix.get_adjacent(2, 1, CellDir::T).unwrap().node_id(),
        NodeId::Sin(2));
    assert_eq!(
        matrix.get_adjacent(2, 1, CellDir::TL).unwrap().node_id(),
        NodeId::Sin(1));
    assert_eq!(
        matrix.get_adjacent(2, 1, CellDir::BL).unwrap().node_id(),
        NodeId::Sin(6));
}

#[test]
fn check_matrix_out_twice_assignment() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 7, 7);

    matrix.place(0, 0, Cell::empty(NodeId::Sin(0))
                       .out(None, Some(0), None));
    matrix.place(0, 1, Cell::empty(NodeId::Sin(0))
                       .out(Some(0), None, None));
    matrix.place(1, 0, Cell::empty(NodeId::Out(0))
                       .input(None, Some(0), Some(0))
                       .out(None, None, None));

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
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(amp)
                       .input(out.inp("ch1"), None, None)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let att_param  = amp.inp_param("att").unwrap();
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
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
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

    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
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
        matrix.place(0, 0, Cell::empty(sin)
                           .out(None, None, sin.out("sig")));
        matrix.place(0, 1, Cell::empty(out)
                           .input(out.inp("ch1"), None, None));
        matrix.sync().unwrap();

        let freq_param = sin.inp_param("freq").unwrap();
        matrix.set_param(freq_param, SAtom::param(-0.2));

        let fft = run_and_get_fft4096(&mut node_exec, 800, 10.0);
        assert_eq!(fft[0], (108, 993));

        hexodsp::save_patch_to_file(&mut matrix, "check_matrix_serialize.hxy")
            .unwrap();
    }

    {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        hexodsp::load_patch_from_file(
            &mut matrix, "check_matrix_serialize.hxy").unwrap();

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
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(tsq)
                       .input(tsq.inp("clock"), None, None)
                       .out(None, None, tsq.out("trk1")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(-0.978));
    let cmode_param = tsq.inp_param("cmode").unwrap();
    matrix.set_param(cmode_param, SAtom::setting(1));

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.borrow_mut();
        pr.set_rows(16);
        pr.set_cell_value(0,  0, 0xFFF);
        pr.set_cell_value(15, 0, 0x000);
    }

    for _ in 0..10 {
        matrix.check_pattern_data(0);
    }

    // We let the clock mode tune in:
    run_and_undersample(&mut node_exec, 10000.0, 1);

    // Take some real samples:
    let samples = run_and_undersample(&mut node_exec, 2000.0, 10);

    assert_float_eq!(samples[0], 0.3157);
    assert_float_eq!(samples[1], 0.209);
    assert_float_eq!(samples[2], 0.1024);
    assert_float_eq!(samples[3], 0.0648);
    assert_float_eq!(samples[4], 0.95566);
    assert_float_eq!(samples[5], 0.84899);
    assert_float_eq!(samples[6], 0.74231);
    assert_float_eq!(samples[7], 0.6356);
    assert_float_eq!(samples[8], 0.5289);
    assert_float_eq!(samples[9], 0.42228);

    // switch to row trigger:
    matrix.set_param(cmode_param, SAtom::setting(0));
    let samples = run_and_undersample(&mut node_exec, 2000.0, 5);

    assert_float_eq!(samples[0], 0.4863);
    assert_float_eq!(samples[1], 0.4731);
    assert_float_eq!(samples[2], 0.4597);
    assert_float_eq!(samples[3], 0.4463);
    assert_float_eq!(samples[4], 0.4331);

    // set to phase mode:
    matrix.set_param(cmode_param, SAtom::setting(2));
    let samples = run_and_undersample(&mut node_exec, 1000.0, 5);

    assert_float_eq!(samples[0], 0.2491);
    assert_float_eq!(samples[1], 0.0026);
    assert_float_eq!(samples[2], 0.1616);
    assert_float_eq!(samples[3], 0.6655);
    assert_float_eq!(samples[4], 0.8104);
}

#[test]
fn check_matrix_tseq_gate() {
    use hexodsp::dsp::tracker::UIPatternModel;

    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let tsq = NodeId::TSeq(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(tsq)
                       .input(tsq.inp("clock"), None, None)
                       .out(None, None, tsq.out("trk1")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(-0.978));
    let cmode_param = tsq.inp_param("cmode").unwrap();
    matrix.set_param(cmode_param, SAtom::setting(1));

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.borrow_mut();
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

    assert_float_eq!(samples[117], 0.0);
    for i in 118..243 {
        assert_float_eq!(samples[i], 1.0);
    }
    assert_float_eq!(samples[243], 0.0);

    assert_float_eq!(samples[367], 0.0);
    for i in 368..376 {
        assert_float_eq!(samples[i], 1.0);
    }
    assert_float_eq!(samples[376], 0.0);

    assert_float_eq!(samples[680], 0.0);
    assert_float_eq!(samples[681], 1.0);
    assert_float_eq!(samples[682], 0.0);

    assert_float_eq!(samples[688], 0.0);
    assert_float_eq!(samples[689], 1.0);
    assert_float_eq!(samples[690], 0.0);
}


#[test]
fn check_matrix_tseq_2col_gate_bug() {
    use hexodsp::dsp::tracker::UIPatternModel;

    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let tsq = NodeId::TSeq(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(tsq)
                       .input(tsq.inp("clock"), None, None)
                       .out(None, None, tsq.out("trk2")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let freq_param = sin.inp_param("freq").unwrap();
    matrix.set_param(freq_param, SAtom::param(0.0));

    let cmode_param = tsq.inp_param("cmode").unwrap();
    matrix.set_param(cmode_param, SAtom::setting(1));

    let pat = matrix.get_pattern_data(0).unwrap();
    {
        let mut pr = pat.borrow_mut();
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
        if *s > 0.0 { any_non_zero = true; }
    }

    assert!(any_non_zero);
}


#[test]
fn check_matrix_output_feedback() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let amp = NodeId::Amp(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(amp)
                       .input(amp.inp("inp"), None, None));
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

