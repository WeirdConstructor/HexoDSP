mod common;
use common::*;

#[test]
fn check_node_mix3_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 5, 5);

    let amp_1   = NodeId::Amp(0);
    let amp_3   = NodeId::Amp(2);
    let amp_2   = NodeId::Amp(1);
    let mix3_1  = NodeId::Mix3(0);
    let out_1   = NodeId::Out(0);
    matrix.place(1, 1,
        Cell::empty(amp_1)
        .input(None, None, None)
        .out(None, amp_1.out("sig"), None));
    matrix.place(1, 2,
        Cell::empty(amp_3)
        .input(None, None, None)
        .out(amp_3.out("sig"), None, None));
    matrix.place(2, 1,
        Cell::empty(amp_2)
        .input(None, None, None)
        .out(None, None, amp_2.out("sig")));
    matrix.place(2, 2,
        Cell::empty(mix3_1)
        .input(mix3_1.inp("ch1"), mix3_1.inp("ch2"), mix3_1.inp("ch3"))
        .out(None, mix3_1.out("sig"), None));
    matrix.place(3, 2,
        Cell::empty(out_1)
        .input(None, out_1.inp("ch1"), None)
        .out(None, None, None));

    pset_d(&mut matrix, amp_1, "inp", 0.200);
    pset_d(&mut matrix, mix3_1, "gain2", 0.2); // 0.04

    pset_d(&mut matrix, amp_2, "inp", -0.300);
    pset_d(&mut matrix, mix3_1, "gain1", 0.1); // -0.03

    pset_d(&mut matrix, amp_3, "inp", 0.500);
    pset_d(&mut matrix, mix3_1, "gain3", 0.5); // 0.25

    pset_d(&mut matrix, mix3_1, "ogain", 1.0);
    matrix.sync().unwrap();

    // hexodsp::save_patch_to_file(&mut matrix, "check_matrix_ser_mix3.hxy")
    //     .unwrap();

    let res = run_for_ms(&mut node_exec, 25.0);
    // The sum is 0.26
    assert_float_eq!(res.0[100], 0.26);

    pset_d_wait(&mut matrix, &mut node_exec, mix3_1, "gain1", 1.0);

    let res = run_for_ms(&mut node_exec, 25.0);
    // The sum is now (0.25 + 0.04) - 0.3 == -0.01
    assert_float_eq!(res.0[100], -0.01);
}
