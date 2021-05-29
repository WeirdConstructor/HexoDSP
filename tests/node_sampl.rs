mod common;
use common::*;

#[test]
fn check_node_sampl_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let freq_p   = smpl.inp_param("freq").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));

    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!((rms, min, max), (0.505, -0.9998, 1.0));

    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (441, 940));

    matrix.set_param(freq_p, SAtom::param(0.1));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (894, 988));

    matrix.set_param(freq_p, SAtom::param(-0.1));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (226, 966));

    matrix.set_param(freq_p, SAtom::param(-0.2));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (108, 953));

    matrix.set_param(freq_p, SAtom::param(-0.4));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (22, 818));

    matrix.set_param(freq_p, SAtom::param(-0.5));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (11, 964));

    matrix.set_param(freq_p, SAtom::param(0.2));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (1776, 877));

    matrix.set_param(freq_p, SAtom::param(0.4));
    let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
    assert_eq!(fft[0], (7127, 1029));
}

#[test]
fn check_node_sampl_reload() {
    {
        let (node_conf, _node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        let smpl = NodeId::Sampl(0);
        let out  = NodeId::Out(0);
        matrix.place(0, 0, Cell::empty(smpl)
                           .out(None, None, smpl.out("sig")));
        matrix.place(0, 1, Cell::empty(out)
                           .input(out.inp("ch1"), None, None));
        matrix.sync().unwrap();

        let sample_p = smpl.inp_param("sample").unwrap();
        matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));

        hexodsp::save_patch_to_file(&mut matrix, "check_matrix_serialize.hxy")
            .unwrap();
    }

    {
        let (node_conf, mut node_exec) = new_node_engine();
        let mut matrix = Matrix::new(node_conf, 3, 3);

        hexodsp::load_patch_from_file(
            &mut matrix, "check_matrix_serialize.hxy").unwrap();


        let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
        assert_rmsmima!(rmsmima, (0.505, -0.9998, 1.0));

        let fft = run_and_get_fft4096(&mut node_exec, 800, 20.0);
        assert_eq!(fft[0], (441, 940));
    }
}

#[test]
fn check_node_sampl_load_err() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_NOSIN.wav"));

    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(
        (rms, min, max),
        (0.0, 0.0, 0.0));

    let err = matrix.pop_error();
    assert_eq!(err.unwrap(), "Sample Loading Error\nCouldn't load sample 'tests/sample_NOSIN.wav':\nLoadError(IoError(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))");
}

#[test]
fn check_node_sampl_trigger() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();
    let trig_p   = smpl.inp_param("trig").unwrap();
    matrix.set_param(sample_p, SAtom::audio_unloaded("tests/sample_sin.wav"));
    matrix.set_param(pmode_p, SAtom::setting(1));

    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 10.0);
    assert_rmsmima!(rmsmima, (0.0, 0.0, 0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsmima = run_and_get_first_rms_mimax(&mut node_exec, 10.0);
    assert_rmsmima!(rmsmima, (0.1136, -0.9998, 1.0));

    let rmsmima = run_and_get_l_rms_mimax(&mut node_exec, 20.0);
    assert_rmsmima!(rmsmima, (0.0, 0.0, 0.0));
}

fn create_1sec_const(s: f32) -> SAtom {
    let mut test_sample_ramp = vec![s; (SAMPLE_RATE_US) + 1];
    test_sample_ramp[0] = SAMPLE_RATE;

    SAtom::audio(
        "1second_const.wav",
        std::sync::Arc::new(test_sample_ramp))
}

fn create_1sec_ramp() -> SAtom {
    let mut test_sample_ramp = vec![0.0; (SAMPLE_RATE_US) + 1];
    test_sample_ramp[0] = SAMPLE_RATE;
    for i in 0..(test_sample_ramp.len() - 1) {
        test_sample_ramp[i + 1] =
            (i as f32) / ((test_sample_ramp.len() - 2) as f32)
    }

    SAtom::audio(
        "1second_ramp.wav",
        std::sync::Arc::new(test_sample_ramp))
}

#[test]
fn check_node_sampl_trigger_reset_phase() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();
    let trig_p   = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    // One Shot Mode
    matrix.set_param(pmode_p, SAtom::setting(1));

    let (rms, min, max) = run_and_get_l_rms_mimax(&mut node_exec, 10.0);
    assert_float_eq!(rms, 0.0);
    assert_float_eq!(min, 0.0);
    assert_float_eq!(max, 0.0);

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0,      0.092496));
    assert_minmax_of_rms!(rmsvec[2], (0.19252,  0.29250));

    // lower trigger level, for retrigger later
    matrix.set_param(trig_p, (0.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 10.0);
    assert_minmax_of_rms!(rmsvec[2], (0.31252, 0.32250));

    // retrigger the phase sample
    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);

    let (_rms, min, max) = rmsvec[0];
    // this is the start of the phase
    assert_float_eq!(min, 0.0);
    // this is the last value of the previous triggering
    assert_float_eq!(max, 0.32998);

    assert_minmax_of_rms!(rmsvec[1], (0.09251, 0.19249));
    assert_minmax_of_rms!(rmsvec[2], (0.19252, 0.29250));
}

#[test]
fn check_node_sampl_trigger_loop_reset_phase() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();
    let trig_p   = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    // Loop mode:
    matrix.set_param(pmode_p, SAtom::setting(0));

    matrix.set_param(trig_p, (0.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0999));
    assert_minmax_of_rms!(rmsvec[2], (0.2, 0.2999));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 100.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0,      0.3074));
    assert_minmax_of_rms!(rmsvec[2], (0.1925,   0.2925));
}

#[test]
fn check_node_sampl_offs_len() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();
    let offs_p   = smpl.inp_param("offs").unwrap();
    let len_p    = smpl.inp_param("len").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    matrix.set_param(pmode_p, SAtom::setting(0));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(0.5));
    matrix.set_param(len_p,  SAtom::param(0.5));

    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.001113, 0.54999));
    assert_minmax_of_rms!(rmsvec[2], (0.6,      0.65));

    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.65, 0.6999));
    assert_minmax_of_rms!(rmsvec[1], (0.70, 0.75));
    assert_minmax_of_rms!(rmsvec[2], (0.5,  0.55));
}


#[test]
fn check_node_sampl_offs_len_zero_crash() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();
    let offs_p   = smpl.inp_param("offs").unwrap();
    let len_p    = smpl.inp_param("len").unwrap();
    let trig_p   = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_ramp());
    matrix.set_param(pmode_p, SAtom::setting(0));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(1.0));
    matrix.set_param(len_p,  SAtom::param(0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 1.0));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(0.9));
    matrix.set_param(len_p,  SAtom::param(0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0));

    // Select part 0.5 to 0.75 of the sample:
    matrix.set_param(offs_p, SAtom::param(1.0));
    matrix.set_param(len_p,  SAtom::param(0.0));

    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 50.0);
    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0));
}

#[test]
fn check_node_sampl_offs_modulated_crash() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin  = NodeId::Sin(0);
    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(smpl)
                       .input(smpl.inp("offs"), None, None)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();

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
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();
    let dclick_p = smpl.inp_param("dclick").unwrap();
    let trig_p   = smpl.inp_param("trig").unwrap();

    matrix.set_param(sample_p, create_1sec_const(1.0));
    // One Shot Mode
    matrix.set_param(pmode_p,  SAtom::setting(1));
    matrix.set_param(dclick_p, SAtom::setting(0));
    matrix.set_param(trig_p, (1.0).into());
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 5.0);

    assert_minmax_of_rms!(rmsvec[0], (0.0, 0.0));
    assert_minmax_of_rms!(rmsvec[1], (0.0, 1.0));
    assert_minmax_of_rms!(rmsvec[2], (1.0, 1.0));

    // reset trigger:
    matrix.set_param(trig_p, (0.0).into());
    run_for_ms(&mut node_exec, 1000.0);

    matrix.set_param(dclick_p, SAtom::setting(1));
    matrix.set_param(trig_p, (1.0).into());
    // let the trigger appear in the sampler:
    run_for_ms(&mut node_exec, 7.5);
    // now the de-click should run:
    let rmsvec = run_and_get_each_rms_mimax(&mut node_exec, 1.0);

    assert_minmax_of_rms!(rmsvec[0], (0.0,    0.3105));
    assert_minmax_of_rms!(rmsvec[1], (0.3177, 0.6282));
    assert_minmax_of_rms!(rmsvec[2], (0.6354, 0.9460));
}


#[test]
fn check_node_sampl_declick_offs_len() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smpl = NodeId::Sampl(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smpl)
                       .out(None, None, smpl.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let sample_p = smpl.inp_param("sample").unwrap();
    let pmode_p  = smpl.inp_param("pmode").unwrap();
    let dclick_p = smpl.inp_param("dclick").unwrap();
    let trig_p   = smpl.inp_param("trig").unwrap();
    let offs_p   = smpl.inp_param("offs").unwrap();
    let len_p    = smpl.inp_param("len").unwrap();

    matrix.set_param(sample_p, create_1sec_const(1.0));
    // One Shot Mode
    matrix.set_param(pmode_p,  SAtom::setting(1));
    matrix.set_param(dclick_p, SAtom::setting(1));
    matrix.set_param(trig_p, (1.0).into());
    matrix.set_param(offs_p, SAtom::param(0.9));
    matrix.set_param(len_p,  SAtom::param(0.08));

    // trigger:
    run_for_ms(&mut node_exec, 7.5);

    let res = run_for_ms(&mut node_exec, 12.0);

    assert_decimated_feq!(res.0, 15, vec![
        0.0, 0.10832358, 0.21664716, 0.32497075, 0.43329433, 0.54161793,
        0.6499415, 0.7582651, 0.86658865, 0.9749123,
        1.0, 1.0, 1.0, 1.0, 1.0,
        0.9243612, 0.81603765, 0.7077141, 0.5993905, 0.4910669,
        0.38274333, 0.27441975, 0.16609617, 0.057772577,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
    ]);
}
