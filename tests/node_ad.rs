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


    matrix.set_param(trig_p, SAtom::param(0.0));
    let res = run_for_ms(&mut node_exec, 0.1);
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 1.5);
    assert_decimated_feq!(res.0, 2, vec![
        0.0075585, 0.022675736, 0.03779289, 0.05291005, 0.068027206, 0.08314436,
        0.09826152, 0.113378674, 0.12849583, 0.143613, 0.15873015,
        0.1738473, 0.18896446, 0.20408161, 0.21919878, 0.23431593,
        0.24943309, 0.26455024, 0.2796674, 0.29478455, 0.3099017,
        0.32501888, 0.34013602, 0.3552532, 0.37037033, 0.3854875,
        0.40060467, 0.4157218, 0.43083897, 0.4459561, 0.46107328,
        0.47619045, 0.4913076 
    ]);

    // Reset trigger
    matrix.set_param(trig_p, SAtom::param(0.0));
    let res = run_for_ms(&mut node_exec, 0.1);
    assert_slope_feq!(res.0, vec![0.00755; 3]);

    // Retrigger attack (should do nothing)
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 0.1);
    assert_slope_feq!(res.0, vec![0.00755; 7]);

    // Wait into decay phase
    matrix.set_param(trig_p, SAtom::param(0.0));
    let res = run_for_ms(&mut node_exec, 1.4);
    let mut v = vec![0.00755; 57];
    v.append(&mut vec![0.002267, -0.002267, -0.002267]);
    assert_slope_feq!(res.0, v);

    // Decay some more
    let res = run_for_ms(&mut node_exec, 0.8);
    assert_slope_feq!(res.0, vec![-0.002267; 100]);

    // Retrigger right in the decay phase
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 1.0);
    assert_slope_feq!(res.0, vec![
        // Re-attack until we are at 1.0 again
        0.007558584, 0.007558584, 0.007558584, 0.0075585246, 0.007558584,
        0.007558584, 0.007558584, 0.007558584, 0.007558584, 0.007558584,
        0.0007558465,
        // Restart decay after 1.0 was reached:
        -0.002267599, -0.0022675395, -0.002267599,
        -0.0022675395, -0.002267599, -0.0022675395, -0.002267599,
        -0.002267599, -0.0022675395, -0.002267599, -0.0022675395,
        -0.002267599, -0.0022675395, -0.002267599, -0.002267599,
        -0.0022675395, -0.002267599, -0.0022675395, -0.002267599,
        -0.0022675395, -0.002267599, -0.002267599, -0.0022675395,
        -0.002267599, -0.0022675395, -0.002267599, -0.0022675395,
        -0.002267599, -0.002267599, -0.0022675395, -0.002267599, -0.0022675395
    ]);
}

#[test]
fn check_node_ad_inp_sin() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let sin = NodeId::Sin(0);
    let ad  = NodeId::Ad(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(sin)
                       .out(None, None, sin.out("sig")));
    matrix.place(0, 1, Cell::empty(ad)
                       .input(ad.inp("inp"), None, None)
                       .out(None, None, ad.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let trig_p = ad.inp_param("trig").unwrap();
    let atk_p  = ad.inp_param("atk").unwrap();
    let dcy_p  = ad.inp_param("dcy").unwrap();

    // check if we have any frequencies resembling 440Hz
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 7.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 6);
    assert_eq!(fft[0], (420, 6));
    assert_eq!(fft[1], (431, 6));
    assert_eq!(fft[2], (441, 6));
    assert_eq!(fft[3], (452, 6));
    assert_eq!(fft[4], (463, 6));

    // Next we test if lengthening the attack has
    // effect on the captured frequencies.
    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);

    matrix.set_param(atk_p, SAtom::param(atk_p.norm(40.0)));
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 7.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 300);
    assert_eq!(fft[0], (431, 322));
    assert_eq!(fft[1], (441, 360));

    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);

    // Next we test if lengthening the decay too has
    // effect on the captured frequencies.
    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);

    matrix.set_param(dcy_p, SAtom::param(dcy_p.norm(40.0)));
    matrix.set_param(trig_p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 7.0);

    let fft = run_and_get_fft4096_now(&mut node_exec, 300);
    assert_eq!(fft[0], (431, 489));
    assert_eq!(fft[1], (441, 647));
    assert_eq!(fft[2], (452, 398));

    matrix.set_param(trig_p, SAtom::param(0.0));
    run_for_ms(&mut node_exec, 8.0);
}
