mod common;
use common::*;

#[test]
fn check_normalized_if_freq_lower_than_formant_freq() {
    let (node_conf, mut node_exec) = new_node_engine();

    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("formfm", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::FormFM(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let side_p = formant.inp_param("side").unwrap();
    let peak_p = formant.inp_param("peak").unwrap();

    // set params to reasonable values
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(side_p, SAtom::param(0.2));
    matrix.set_param(peak_p, SAtom::param(0.4));

    // run
    let res = run_for_ms(&mut node_exec, 100.0);

    // and check it's normalized
    let max = res.0.iter().fold(0.0 as f32, |acc, x| acc.max(*x));
    let min = res.0.iter().fold(0.0 as f32, |acc, x| acc.min(*x));
    assert!(max > 0.8 && max < 1.01 && min < -0.8 && min > -1.01);
}

#[test]
fn check_no_dc_bias_at_formant_freq_lower_than_freq() {
    let (node_conf, mut node_exec) = new_node_engine();

    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("formfm", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::FormFM(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let side_p = formant.inp_param("side").unwrap();
    let peak_p = formant.inp_param("peak").unwrap();

    // set params to reasonable values
    matrix.set_param(freq_p, SAtom::param(0.0));
    matrix.set_param(form_p, SAtom::param(-0.2));
    matrix.set_param(side_p, SAtom::param(0.2));
    matrix.set_param(peak_p, SAtom::param(0.4));

    // run
    let res = run_for_ms(&mut node_exec, 100.0);

    // average should remain at ~0
    let sum = res.0.iter().sum::<f32>();
    let avg = sum / res.0.len() as f32;

    assert!(avg > -0.05 && avg < 0.05);
}

#[test]
fn check_no_nan() {
    let (node_conf, mut node_exec) = new_node_engine();

    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("formfm", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::FormFM(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let side_p = formant.inp_param("side").unwrap();
    let peak_p = formant.inp_param("peak").unwrap();

    // set params to non-reasonable values here
    // base freq 0
    matrix.set_param(freq_p, SAtom::param(-1.0));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(side_p, SAtom::param(0.2));
    matrix.set_param(peak_p, SAtom::param(0.4));

    // run
    let res = run_for_ms(&mut node_exec, 100.0);

    // and check there's no NaN
    assert!(res.0.iter().all(|x| !x.is_nan()));

    // set params to non-reasonable values here
    // side to 0
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(side_p, SAtom::param(-1.0));
    matrix.set_param(peak_p, SAtom::param(0.4));

    // run
    let res = run_for_ms(&mut node_exec, 100.0);

    // and check there's no NaN
    assert!(res.0.iter().all(|x| !x.is_nan()));

    // set params to non-reasonable values here
    // side to 1
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(peak_p, SAtom::param(1.0));
    matrix.set_param(side_p, SAtom::param(0.4));

    // run
    let res = run_for_ms(&mut node_exec, 100.0);

    // and check there's no NaN
    assert!(res.0.iter().all(|x| !x.is_nan()));
}

#[test]
fn check_formant_freq() {
    let (node_conf, mut node_exec) = new_node_engine();

    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("formfm", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::FormFM(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let side_p = formant.inp_param("side").unwrap();
    let peak_p = formant.inp_param("peak").unwrap();

    // set params to reasonable values
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(side_p, SAtom::param(0.2));
    matrix.set_param(peak_p, SAtom::param(0.4));

    // run
    let fft = run_and_get_avg_fft4096_now(&mut node_exec, 100);
    assert_eq!(fft, vec![(323, 106), (334, 131), (431, 430), (441, 708), (452, 288), (549, 140)]);
}
