mod common;
use common::*;

#[test]
fn check_node_ad_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let ad   = NodeId::Ad(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(ad)
                       .out(None, None, ad.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let trig_p = ad.inp_param("trig").unwrap();

    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_slope_feq!(res.0, 50, vec![
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
        0.007558584, 0.007558584, 0.007558584,
        // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
        -0.002267599, -0.0022675395, -0.002267599, -0.0022675395,
        -0.0022675693, -0.0022675693, -0.0022675842, -0.0022675693,
        -0.0022675726,
        0.0, 0.0, 0.0, 0.0
    ]);

    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 10.0);
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 25.0);
    //d// println!("RES: {:?}", res);
    let start = res.0[330];
    assert_float_eq!(start, 0.0075585);
    let peak = res.0[330 + ((44.1_f64 * 3.0).floor() as usize)];
    assert_float_eq!(peak, 1.0);
}


#[test]
fn check_node_ad_retrig() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let test = NodeId::Test(0);
    let ad   = NodeId::Ad(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(test)
                       .out(None, None, test.out("sig")));
    matrix.place(0, 1, Cell::empty(ad)
                       .input(ad.inp("trig"), None, None)
                       .out(None, None, ad.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let trig_p = test.inp_param("p").unwrap();

    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 25.0);
    assert_decimated_slope_feq!(res.0, 50, vec![
        // XXX: Direct trigger!
        // Due to Test node outputting an unsmoothed value!

        // 44.1 per ms, attack is default 3.0ms (roughly 3 * 50 samples):
        0.007558584, 0.007558584, 0.007558584,
        // 44.1 per ms, decay is default 10.0ms (=> roughly 9 * 50 samples):
        -0.002267599, -0.0022675395, -0.002267599, -0.0022675395,
        -0.0022675693, -0.0022675693, -0.0022675842, -0.0022675693,
        -0.0022675726,
        0.0, 0.0, 0.0, 0.0
    ]);
}
