mod common;
use common::*;

#[test]
fn check_param_mod_amt_no_input() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    matrix.set_param_modamt(
        sin.inp_param("freq").unwrap(), Some(0.2)).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (0.4999, -1.0, 1.0));
}

#[test]
fn check_param_mod_amt_with_input() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin  = NodeId::Sin(0);
    let sin2 = NodeId::Sin(1);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin2)
                       .out(None, None, sin2.out("sig")));
    matrix.place(0, 1, Cell::empty(sin)
                       .input(sin.inp("freq"), None, None)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    matrix.set_param_modamt(
        sin.inp_param("freq").unwrap(), Some(0.2)).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (0.4992, -1.0, 1.0));
}

#[test]
fn check_param_mod_amt_set() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let tst = NodeId::Test(0);
    let amp = NodeId::Amp(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(tst)
                       .out(None, None, tst.out("sig")));
    matrix.place(0, 1, Cell::empty(amp)
                       .input(amp.inp("inp"), None, None)
                       .out(None, None, amp.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_n(&mut matrix, tst, "p", 1.0);
    matrix.sync().unwrap();

    // Run with no modulation
    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (1.0, 1.0, 1.0));

    // Enable modulation
    matrix.set_param_modamt(
        amp.inp_param("inp").unwrap(), Some(0.2)).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (0.04, 0.2, 0.2));

    // Change modulation
    matrix.set_param_modamt(
        amp.inp_param("inp").unwrap(), Some(0.1)).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (0.01, 0.1, 0.1));

    // Remove modulation
    matrix.set_param_modamt(
        amp.inp_param("inp").unwrap(), None).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (1.0, 1.0, 1.0));
}


#[test]
fn check_param_mod_amt_set_bipol() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let tst = NodeId::Test(0);
    let amp = NodeId::Amp(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(tst)
                       .out(None, None, tst.out("sig")));
    matrix.place(0, 1, Cell::empty(amp)
                       .input(amp.inp("inp"), None, None)
                       .out(None, None, amp.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_n(&mut matrix, tst, "p", -1.0);
    matrix.sync().unwrap();

    // Run with no modulation
    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (1.0, -1.0, -1.0));

    // Enable modulation
    matrix.set_param_modamt(
        amp.inp_param("inp").unwrap(), Some(0.2)).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (0.04, -0.2, -0.2));

    // Change modulation
    matrix.set_param_modamt(
        amp.inp_param("inp").unwrap(), Some(0.1)).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (0.01, -0.1, -0.1));

    // Remove modulation
    matrix.set_param_modamt(
        amp.inp_param("inp").unwrap(), None).unwrap();

    let rms = run_and_get_first_rms_mimax(&mut node_exec, 50.0);
    assert_rmsmima!(rms, (1.0, -1.0, -1.0));
}
