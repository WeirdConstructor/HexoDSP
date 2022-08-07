mod common;
use common::*;

#[test]
fn check_normalized_if_freq_lower_than_formant_freq() {
    let (node_conf, mut node_exec) = new_node_engine();

    let mut matrix = Matrix::new(node_conf, 3, 3);

    let mut chain = MatrixCellChain::new(CellDir::B);
    chain.node_out("formant", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::Formant(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let atk_p = formant.inp_param("atk").unwrap();
    let dcy_p = formant.inp_param("dcy").unwrap();

    // set params to reasonable values
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(atk_p, SAtom::param(0.2));
    matrix.set_param(dcy_p, SAtom::param(-0.2));

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
    chain.node_out("formant", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::Formant(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let atk_p = formant.inp_param("atk").unwrap();
    let dcy_p = formant.inp_param("dcy").unwrap();

    // set params to reasonable values
    matrix.set_param(freq_p, SAtom::param(0.0));
    matrix.set_param(form_p, SAtom::param(-0.2));
    matrix.set_param(atk_p, SAtom::param(0.2));
    matrix.set_param(dcy_p, SAtom::param(-0.2));

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
    chain.node_out("formant", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::Formant(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let atk_p = formant.inp_param("atk").unwrap();
    let dcy_p = formant.inp_param("dcy").unwrap();

    // set params to non-reasonable values here
    // base freq 0
    matrix.set_param(freq_p, SAtom::param(-1.0));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(atk_p, SAtom::param(0.2));
    matrix.set_param(dcy_p, SAtom::param(-0.2));

    // run
    let res = run_for_ms(&mut node_exec, 100.0);

    // and check there's no NaN
    assert!(res.0.iter().all(|x| !x.is_nan()));

    // set params to non-reasonable values here
    // base freq attack freq 0
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(atk_p, SAtom::param(-1.0));
    matrix.set_param(dcy_p, SAtom::param(-0.2));

    // run
    let res = run_for_ms(&mut node_exec, 100.0);

    // and check there's no NaN
    assert!(res.0.iter().all(|x| !x.is_nan()));

    // set params to non-reasonable values here
    // decay freq freq 0
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(atk_p, SAtom::param(0.2));
    matrix.set_param(dcy_p, SAtom::param(-1.0));

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
    chain.node_out("formant", "sig").node_inp("out", "ch1").place(&mut matrix, 0, 0).unwrap();

    matrix.sync().unwrap();

    let formant = NodeId::Formant(0);

    // params
    let freq_p = formant.inp_param("freq").unwrap();
    let form_p = formant.inp_param("form").unwrap();
    let atk_p = formant.inp_param("atk").unwrap();
    let dcy_p = formant.inp_param("dcy").unwrap();

    // set params to reasonable values
    matrix.set_param(freq_p, SAtom::param(-0.2));
    matrix.set_param(form_p, SAtom::param(0.0));
    matrix.set_param(atk_p, SAtom::param(0.2));
    matrix.set_param(dcy_p, SAtom::param(-0.2));

    // run
    let fft = run_and_get_avg_fft4096_now(&mut node_exec, 180);
    assert_eq!(fft, vec![(334, 191), (431, 331), (441, 546), (452, 222), (549, 209)]);
}
