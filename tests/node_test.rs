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

#[test]
fn check_node_test_out_connected() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 6, 3);

    let test = NodeId::Test(0);
    let out  = NodeId::Out(0);
    let sin  = NodeId::Sin(0);
    let sin2 = NodeId::Sin(1);
    let sin3 = NodeId::Sin(2);
    matrix.place(0, 0, Cell::empty(test)
                       .out(None, None, test.out("outc")));
    matrix.place(0, 1, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));

    matrix.place(1, 0, Cell::empty(test)
                       .out(None, None, test.out("out2")));
    matrix.place(1, 1, Cell::empty(out)
                       .input(out.inp("ch2"), None, None));

    matrix.place(2, 0, Cell::empty(test)
                       .out(None, None, test.out("out3")));
    matrix.place(2, 1, Cell::empty(sin)
                       .input(sin.inp("freq"), None, None));

    matrix.place(3, 0, Cell::empty(test)
                       .out(None, None, test.out("out4")));
    matrix.place(3, 1, Cell::empty(sin2)
                       .input(sin2.inp("freq"), None, None));

    matrix.place(4, 0, Cell::empty(test)
                       .out(None, None, test.out("sig")));
    matrix.place(4, 1, Cell::empty(sin3)
                       .input(sin3.inp("freq"), None, None));
    matrix.sync().unwrap();

    let res = run_for_ms(&mut node_exec, 2.0);
    let mask = 0x01 | 0x04 | 0x08 | 0x10 | 0x20;
    assert_decimated_feq!(res.0, 1, vec![ mask as f32; 10 ]);
    assert_decimated_feq!(res.1, 1, vec![ 1.0; 10 ]);

    // Remove a connection for testing:
    matrix.place(1, 1, Cell::empty(NodeId::Nop));
    matrix.sync().unwrap();

    let res = run_for_ms(&mut node_exec, 2.0);
    let mask = 0x01 | 0x08 | 0x10 | 0x20;
    assert_decimated_feq!(res.0, 1, vec![ mask as f32; 10 ]);
    assert_decimated_feq!(res.1, 1, vec![ 0.0; 10 ]);
}
