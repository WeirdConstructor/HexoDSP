mod common;
use common::*;

#[test]
fn check_node_smap() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smap = NodeId::SMap(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smap)
                       .out(None, None, smap.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    pset_n(&mut matrix, smap, "inp",  0.5);

    pset_n(&mut matrix, smap, "min", -1.0);
    pset_n(&mut matrix, smap, "max", -0.9); // we expect -0.95

    pset_s(&mut matrix, smap, "mode", 0); // unipolar
    matrix.sync().unwrap();

    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-0.95; 50]);

    pset_s(&mut matrix, smap, "mode", 1); // bipolar
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-0.925; 50]);

    pset_s(&mut matrix, smap, "mode", 3); // bipolar inverted
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-0.975; 50]);

    pset_n(&mut matrix, smap, "inp",  1.0);
    run_for_ms(&mut node_exec, 10.0);

    pset_s(&mut matrix, smap, "mode", 2); // unipolar inverted
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-1.0; 50]);
}

#[test]
fn check_node_smap_clip() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let smap = NodeId::SMap(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(smap)
                       .out(None, None, smap.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));

    pset_s(&mut matrix, smap, "mode", 0); // unipolar
    pset_n(&mut matrix, smap, "inp",  -0.5);
    pset_n(&mut matrix, smap, "min",  0.1);
    pset_n(&mut matrix, smap, "max", -0.1);

    matrix.sync().unwrap();

    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![0.2; 50]);

    pset_s(&mut matrix, smap, "clip", 1); // enable clipping
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![0.1; 50]);

    pset_s(&mut matrix, smap, "mode", 1); // bipolar
    pset_s(&mut matrix, smap, "clip", 0); // disable clipping

    // go to -1.5 input here, which is very much below unipolar
    // and a bit below bipolar.
    pset_n(&mut matrix, smap, "inp",  -1.5); // out of range input

    run_for_ms(&mut node_exec, 10.0);

    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![0.15; 50]);

    pset_s(&mut matrix, smap, "clip", 1); // enable clipping
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![0.1; 50]);

    pset_s(&mut matrix, smap, "mode", 0); // unipolar
    pset_s(&mut matrix, smap, "clip", 0); // disable clipping
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![(0.1 * (1.0 + 1.5)) + (-0.1 * -1.5); 50]);

    pset_s(&mut matrix, smap, "clip", 1); // enable clipping
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![0.1; 50]);

    pset_s(&mut matrix, smap, "mode", 2); // unipolar inverse
    pset_s(&mut matrix, smap, "clip", 0); // disable clipping
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![(-0.1 * (1.0 + 1.5)) + (0.1 * -1.5); 50]);

    pset_s(&mut matrix, smap, "clip", 1); // enable clipping
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-0.1; 50]);

    pset_s(&mut matrix, smap, "mode", 3); // bipolar inverse
    pset_s(&mut matrix, smap, "clip", 0); // disable clipping

    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-0.15; 50]);

    pset_s(&mut matrix, smap, "clip", 1); // enable clipping
    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-0.1; 50]);
}
