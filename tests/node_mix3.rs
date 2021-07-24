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

    matrix.sync().unwrap();

    let res = run_for_ms(&mut node_exec, 25.0);
    assert_float_eq!(res.0[100], 0.0075585);
}
