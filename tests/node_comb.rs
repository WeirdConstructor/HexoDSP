mod common;
use common::*;

#[test]
fn check_node_comb_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise_1 = NodeId::Noise(0);
    let comb_1 = NodeId::Comb(0);
    let out_1 = NodeId::Out(0);
    matrix.place(
        0,
        1,
        Cell::empty(noise_1).input(None, None, None).out(None, noise_1.out("sig"), None),
    );
    matrix.place(
        1,
        1,
        Cell::empty(comb_1).input(None, comb_1.inp("inp"), None).out(None, comb_1.out("sig"), None),
    );
    matrix.place(
        2,
        2,
        Cell::empty(out_1).input(None, out_1.inp("ch1"), out_1.inp("ch1")).out(None, None, None),
    );

    pset_n(&mut matrix, comb_1, "g", 0.950);
    pset_n(&mut matrix, comb_1, "time", 0.014);
    matrix.sync().unwrap();

    let fft = run_and_get_avg_fft4096_now(&mut node_exec, 180);
    assert_eq!(
        fft,
        vec![
            (0, 216),
            (11, 221),
            (22, 216),
            (3381, 184),
            (3391, 189),
            (3402, 195),
            (3413, 213),
            (3424, 198),
            (3435, 203),
            (6815, 188),
            (13587, 196),
            (13598, 210),
            (13609, 207),
            (13620, 193)
        ]
    );

    pset_n_wait(&mut matrix, &mut node_exec, comb_1, "time", 0.030);

    let fft = run_and_get_avg_fft4096_now(&mut node_exec, 180);
    assert_eq!(fft, vec![(1001, 186), (3015, 186), (7031, 191)]);

    pset_n_wait(&mut matrix, &mut node_exec, comb_1, "g", 0.999);
    let fft = run_and_get_avg_fft4096_now(&mut node_exec, 1000);
    assert_eq!(fft, vec![(0, 2008), (11, 1017)]);
}

#[test]
fn check_node_comb_time() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let test = NodeId::Test(0);
    let comb_1 = NodeId::Comb(0);
    let out_1 = NodeId::Out(0);
    matrix.place(0, 1, Cell::empty(test).input(None, None, None).out(None, test.out("tsig"), None));
    matrix.place(
        1,
        1,
        Cell::empty(comb_1).input(None, comb_1.inp("inp"), None).out(None, comb_1.out("sig"), None),
    );
    matrix.place(
        2,
        2,
        Cell::empty(out_1).input(None, out_1.inp("ch1"), out_1.inp("ch1")).out(None, None, None),
    );

    pset_n(&mut matrix, comb_1, "g", 0.75);
    pset_d(&mut matrix, comb_1, "time", 100.0);
    matrix.sync().unwrap();

    pset_n(&mut matrix, test, "trig", 1.0);

    let (out_l, _) = run_for_ms(&mut node_exec, 300.0);
    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], 25.0);
    assert_minmax_of_rms!(rms_mimax[0], (0.0, 1.0));
    assert_minmax_of_rms!(rms_mimax[1], (0.0, 0.0));
    assert_minmax_of_rms!(rms_mimax[2], (0.0, 0.0));
    assert_minmax_of_rms!(rms_mimax[3], (0.0, 0.0));
    assert_minmax_of_rms!(rms_mimax[4], (-0.0001, 0.7502));
    assert_minmax_of_rms!(rms_mimax[5], (0.0, 0.0));
    assert_minmax_of_rms!(rms_mimax[6], (0.0, 0.0));
    assert_minmax_of_rms!(rms_mimax[7], (0.0, 0.0));
    assert_minmax_of_rms!(rms_mimax[8], (-0.00027, 0.56277));
}
