// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_sampl_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let freq_p = smpl.inp_param("freq").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));

    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!((rms, min, max), (0.5004, -0.9997, 0.9997));

    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (441, 1023));

    matrix.set_param(freq_p, SAtom::param(0.1));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (883, 1020));

    matrix.set_param(freq_p, SAtom::param(-0.1));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (215, 881));

    matrix.set_param(freq_p, SAtom::param(-0.2));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (108, 987));

    matrix.set_param(freq_p, SAtom::param(-0.4));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (22, 831));

    matrix.set_param(freq_p, SAtom::param(-0.5));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (11, 1000));

    matrix.set_param(freq_p, SAtom::param(0.2));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (1766, 1008));

    matrix.set_param(freq_p, SAtom::param(0.4));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (7052, 942));
}

#[test]
fn check_node_sampl_long_freq() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin_long.wav"));

    run_no_input(&mut node_exec, 0.05);

    let fft = run_and_get_fft4096(&mut node_exec, 800, 100.0);
    assert_eq!(fft[0], (441, 1014));

    let cfreq = run_and_get_counted_freq(&mut node_exec, 4000.0);
    // The slight wrong tune might be from the limited number of samples?
    assert_float_eq!(cfreq.floor(), 440.0);
}

#[test]
fn check_node_sampl_detune() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let det_p = smpl.inp_param("det").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));

    run_no_input(&mut node_exec, 0.05);

    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    // The slight wrong tune comes from the very very short sample of a sine wave!
    // See also the check_node_sampl_long_freq() test above!
    assert_float_eq!(cfreq.floor(), 441.0);

    matrix.set_param(det_p, SAtom::param(det_p.norm(1.0)));
    run_no_input(&mut node_exec, 0.05);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 2000.0);
    assert_float_eq!(cfreq.floor(), 467.0);

    matrix.set_param(det_p, SAtom::param(det_p.norm(-1.0)));
    run_no_input(&mut node_exec, 0.05);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 417.0);

    matrix.set_param(det_p, SAtom::param(det_p.norm(-12.0)));
    run_no_input(&mut node_exec, 0.05);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1200.0);
    assert_float_eq!(cfreq.floor(), 220.0);

    matrix.set_param(det_p, SAtom::param(det_p.norm(-24.0)));
    run_no_input(&mut node_exec, 0.05);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1200.0);
    assert_float_eq!(cfreq.floor(), 110.0);

    matrix.set_param(det_p, SAtom::param(det_p.norm(24.0)));
    run_no_input(&mut node_exec, 0.05);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 1764.0);

    matrix.set_param(det_p, SAtom::param(det_p.norm(36.0)));
    run_no_input(&mut node_exec, 0.05);
    let cfreq = run_and_get_counted_freq(&mut node_exec, 1000.0);
    assert_float_eq!(cfreq.floor(), 3529.0);

    //d// let (out_l, _) = run_no_input(&mut node_exec, 1.0);
    //d// save_wav("check_node_sampl_detune.wav", &out_l);
}

#[test]
fn check_node_sampl_reverse() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let freq_p = smpl.inp_param("freq").unwrap();
    let dir_p = smpl.inp_param("dir").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));
    matrix.set_param(dir_p, SAtom::setting(1));

    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!((rms, min, max), (0.5003373, -0.9997, 0.9997));

    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (441, 1023));

    matrix.set_param(freq_p, SAtom::param(0.1));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (883, 1020));

    matrix.set_param(freq_p, SAtom::param(-0.1));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (215, 881));

    matrix.set_param(freq_p, SAtom::param(-0.2));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (108, 987));

    matrix.set_param(freq_p, SAtom::param(-0.4));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (22, 831));

    matrix.set_param(freq_p, SAtom::param(-0.5));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (11, 1000));

    matrix.set_param(freq_p, SAtom::param(0.2));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (1766, 1008));

    matrix.set_param(freq_p, SAtom::param(0.4));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (7052, 942));
}

#[test]
fn check_node_sampl_reload() {
    {
        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        let smpl = NodeId::Sampl(0);
        let out = NodeId::Out(0);
        matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
        matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
        matrix.sync().unwrap();

        let sample_p = smpl.inp_param("sample").unwrap();
        matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));

        hexodsp::save_patch_to_file(&mut matrix, "check_matrix_serialize.hxy").unwrap();
    }

    {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        hexodsp::load_patch_from_file(&mut matrix, "check_matrix_serialize.hxy").unwrap();

        let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
        assert_rmsmima!(rmsmima, (0.5004, -0.9997, 0.9997));

        let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
        assert_eq!(fft[0], (441, 1023));
    }
}

#[test]
fn check_node_sampl_load_err() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_NOSIN.wav"));

    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!((rms, min, max), (0.0, 0.0, 0.0));

    let err = matrix.pop_error();
    assert_eq!(err.unwrap(), "Sample Loading Error\nCouldn't load sample 'tests/sample_NOSIN.wav':\nLoadError(IoError(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))");
}

#[test]
fn check_node_sampl_trigger() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();
    let trig_p = smpl.inp_param("trig").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));
    matrix.set_param(pmode_p, SAtom::setting(1));

    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 10.0);
    assert_rmsmima!(rmsmima, (0.0, 0.0, 0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsmima = run_and_get_first_rms_mimax(&mut node_exec, 10.0);
    assert_rmsmima!(rmsmima, (0.1136, -0.9997, 0.9997));

    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 20.0);
    assert_rmsmima!(rmsmima, (0.0, 0.0, 0.0));
}

fn create_1sec_const(s: f32) -> SAtom {
    let mut test_sample_ramp = vec![s; (SAMPLE_RATE_US) + 1];
    test_sample_ramp[0] = SAMPLE_RATE;

    SAtom::audio("1second_const.wav", std::sync::Arc::new(test_sample_ramp))
}

fn create_1sec_ramp() -> SAtom {
    let mut test_sample_ramp = vec![0.0; (SAMPLE_RATE_US) + 1];
    test_sample_ramp[0] = SAMPLE_RATE;
    for i in 0..(test_sample_ramp.len() - 1) {
        test_sample_ramp[i + 1] = (i as f32) / ((test_sample_ramp.len() - 2) as f32)
    }

    SAtom::audio("1second_ramp.wav", std::sync::Arc::new(test_sample_ramp))
}

#[test]
fn check_node_sampl_trigger_reset_phase() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();
    let trig_p = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    // One Shot Mode
    matrix.set_param(pmode_p, SAtom::setting(1));

    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 10.0);
    assert_float_eq!(rms, 0.0);
    assert_float_eq!(min, 0.0);
    assert_float_eq!(max, 0.0);

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.09499));
    assert_minmax_of_rms!(rmsvec[2], (0.1950, 0.2949));

    // lower trigger level, for retrigger later
    matrix.set_param(trig_p, (0.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 10.0);
    assert_minmax_of_rms!(rmsvec[2], (0.3150, 0.32499));

    // retrigger the phase sample
    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);

    let (_rms, min, max) = rmsvec[0];
    // this is the start of the phase
    assert_float_eq!(min, 0.0);
    // this is the last value of the previous triggering
    assert_float_eq!(max, 0.32998);

    assert_minmax_of_rms!(rmsvec[1], (0.0950, 0.19499));
    assert_minmax_of_rms!(rmsvec[2], (0.1950, 0.29499));
}

#[test]
fn check_node_sampl_trigger_loop_reset_phase() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();
    let trig_p = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    // Loop mode:
    matrix.set_param(pmode_p, SAtom::setting(0));

    matrix.set_param(trig_p, (0.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0999));
    assert_minmax_of_rms!(rmsvec[2], (0.2, 0.2999));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.3050));
    assert_minmax_of_rms!(rmsvec[2], (0.1950, 0.2949));
}

#[test]
fn check_node_sampl_offs_len() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();
    let offs_p = smpl.inp_param("offs").unwrap();
    let len_p = smpl.inp_param("len").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    matrix.set_param(pmode_p, SAtom::setting(0));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(0.5));
    matrix.set_param(len_p, SAtom::param(0.25));

    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.001113, 0.54999));
    assert_minmax_of_rms!(rmsvec[2], (0.6, 0.65));

    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.65, 0.6999));
    assert_minmax_of_rms!(rmsvec[1], (0.70, 0.75));
    assert_minmax_of_rms!(rmsvec[2], (0.5, 0.55));
}

#[test]
fn check_node_sampl_offs_len_zero_crash() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();
    let offs_p = smpl.inp_param("offs").unwrap();
    let len_p = smpl.inp_param("len").unwrap();
    let trig_p = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    matrix.set_param(pmode_p, SAtom::setting(0));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(1.0));
    matrix.set_param(len_p, SAtom::param(0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.9981));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(0.9));
    matrix.set_param(len_p, SAtom::param(0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(1.0));
    matrix.set_param(len_p, SAtom::param(0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0));
}

#[test]
fn check_node_sampl_offs_modulated_crash() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin).out(None, None, sin.out("sig")));
    matrix.place(
        0,
        1,
        Cell::empty(smpl).input(smpl.inp("offs"), None, None).out(None, None, smpl.out("sig")),
    );
    matrix.place(0, 2, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    matrix.set_param(pmode_p, SAtom::setting(0));

    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);
    assert_rmsmima!(rmsvec[0], (0.5008, 0.0, 1.0));
}

#[test]
fn check_node_sampl_declick() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();
    let dclick_p = smpl.inp_param("dclick").unwrap();
    let dcms_p = smpl.inp_param("dcms").unwrap();
    let trig_p = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_const(1.0));
    // One Shot Mode
    matrix.set_param(pmode_p, SAtom::setting(1));
    matrix.set_param(dclick_p, SAtom::setting(0));
    matrix.set_param(dcms_p, SAtom::param(dcms_p.norm(3.14)));
    matrix.set_param(trig_p, (1.0).into());

    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 3.0);

    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0));
    assert_minmax_of_rms!(rmsvec[1], (0.0, 1.0));
    assert_minmax_of_rms!(rmsvec[2], (1.0, 1.0));

    // reset trigger:
    matrix.set_param(trig_p, (0.0).into());
    run_for_ms(&mut node_exec, 1000.0);

    matrix.set_param(dclick_p, SAtom::setting(1));
    matrix.set_param(trig_p, (1.0).into());
    // let the trigger appear in the sampler:
    run_for_ms(&mut node_exec, 5.0);
    // now the de-click should run:
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 1.0);

    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.3105));
    assert_minmax_of_rms!(rmsvec[1], (0.3177, 0.6282));
    assert_minmax_of_rms!(rmsvec[2], (0.6354, 0.9460));
}

#[test]
fn check_node_sampl_declick_offs_len() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();
    let dclick_p = smpl.inp_param("dclick").unwrap();
    let dcms_p = smpl.inp_param("dcms").unwrap();
    let trig_p = smpl.inp_param("trig").unwrap();
    let offs_p = smpl.inp_param("offs").unwrap();
    let len_p = smpl.inp_param("len").unwrap();

    matrix.set_param(sample_p, create_1sec_const(1.0));
    // One Shot Mode
    matrix.set_param(pmode_p, SAtom::setting(1));
    matrix.set_param(dclick_p, SAtom::setting(1));
    matrix.set_param(dcms_p, SAtom::param(dcms_p.norm(3.14)));
    matrix.set_param(trig_p, (1.0).into());
    matrix.set_param(offs_p, SAtom::param(0.9));
    matrix.set_param(len_p, SAtom::param(0.008));

    // trigger:
    run_for_ms(&mut node_exec, 5.0);

    let res = run_for_ms(&mut node_exec, 12.0);

    assert_decimated_feq!(
        res.0,
        15,
        vec![
            0.0,
            0.11080169,
            0.22160338,
            0.33240506,
            0.44320676,
            0.5540084,
            0.6648101,
            0.7756118,
            0.8864135,
            0.98414195,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            0.9331123,
            0.82376325,
            0.7144141,
            0.59939045,
            0.49106687,
            0.3827433,
            0.27441972,
            0.16609615,
            0.057772573,
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
            0.0
        ]
    );
}

#[test]
fn check_node_sampl_rev_2() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let dir_p = smpl.inp_param("dir").unwrap();
    let pmode_p = smpl.inp_param("pmode").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    // Reverse:
    matrix.set_param(dir_p, SAtom::setting(1));
    // Loop mode:
    matrix.set_param(pmode_p, SAtom::setting(0));

    let res = run_for_ms(&mut node_exec, 1000.0);
    assert_decimated_feq!(
        res.0,
        5000,
        vec![
            0.0, 0.88664144, 0.7732602, 0.6598789, 0.54649764, 0.4331164, 0.31973514, 0.20635389,
            0.09297263
        ]
    );

    // Forward:
    matrix.set_param(dir_p, SAtom::setting(0));

    let res = run_for_ms(&mut node_exec, 1000.0);
    assert_decimated_feq!(
        res.0,
        5000,
        vec![
            0.0, 0.11338125, 0.2267625, 0.34014374, 0.453525, 0.5669062, 0.6802875, 0.79366875,
            0.90705
        ]
    );
}

#[test]
fn check_node_sampl_4() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl).out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("hr.wav"));

    let fft = run_and_get_fft4096(&mut node_exec, 0, 1000.0);
    assert_eq!(
        avg_fft_freqs(
            4.0,
            &[10, 100, 200, 300, 440, 800, 1000, 2000, 3000, 4000, 12000, 22050],
            &fft[..]
        ),
        vec![
            (0, 32),
            (10, 204),
            (100, 104),
            (200, 16),
            (300, 32),
            (440, 8),
            (800, 4),
            (1000, 0),
            (2000, 0),
            (3000, 0),
            (4000, 0),
            (12000, 0)
        ]
    );
}
