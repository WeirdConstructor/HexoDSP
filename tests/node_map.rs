// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_node_map() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let map = NodeId::Map(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(map).out(None, None, map.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    pset_n(&mut matrix, map, "inp", 0.5);
    pset_n(&mut matrix, map, "atv", 0.5); // => 0.25
    pset_n(&mut matrix, map, "offs", 0.1); // => 0.35

    pset_n(&mut matrix, map, "imin", 0.0);
    pset_n(&mut matrix, map, "imax", 0.7); // middle is at 0.35

    pset_n(&mut matrix, map, "min", -1.0);
    pset_n(&mut matrix, map, "max", -0.5); // we expect -0.75
    matrix.sync().unwrap();

    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![-0.75; 50]);
}

#[test]
fn check_node_map_clip() {
    let (node_conf, mut node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let map = NodeId::Map(0);
    let out = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(map).out(None, None, map.out("sig")));
    matrix.place(0, 1, Cell::empty(out).input(out.inp("ch1"), None, None));
    pset_n(&mut matrix, map, "inp", 0.5);
    pset_n(&mut matrix, map, "atv", 0.5); // => 0.25
    pset_n(&mut matrix, map, "offs", 0.80); // => 1.05

    pset_n(&mut matrix, map, "imin", 0.0);
    pset_n(&mut matrix, map, "imax", 0.7); // 1.05 is at 0.7 + 0.35

    pset_n(&mut matrix, map, "min", 0.0);
    pset_n(&mut matrix, map, "max", 0.1); // with 50% over imax => 0.15
    matrix.sync().unwrap();

    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![0.15; 50]);

    pset_s(&mut matrix, map, "clip", 1); // should clip at 0.1
    run_for_ms(&mut node_exec, 30.0);

    let res = run_for_ms(&mut node_exec, 10.0);
    assert_decimated_feq!(res.0, 10, vec![0.1; 50]);
}
