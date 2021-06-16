mod common;
use common::*;

#[test]
fn check_node_test_1() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let test = NodeId::Test(0);
    let out  = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(test)
                       .out(None, None, test.out("sig")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.sync().unwrap();

    let p = test.inp_param("p").unwrap();

    matrix.set_param(p, SAtom::param(1.0));
    let res = run_for_ms(&mut node_exec, 2.0);
    assert_decimated_feq!(res.0, 1, vec![ 1.0; 10 ]);

    matrix.set_param(p, SAtom::param(0.5));
    let res = run_for_ms(&mut node_exec, 2.0);
    assert_decimated_feq!(res.0, 1, vec![ 0.5; 10 ]);

    matrix.set_param(p, SAtom::param(0.0));
    let res = run_for_ms(&mut node_exec, 1.0);
    assert_decimated_feq!(res.0, 1, vec![ 0.0; 10 ]);
}
