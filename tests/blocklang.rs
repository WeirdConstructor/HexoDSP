// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

//#[test]
//fn check_blocklang_dir_1() {
//    use hexodsp::block_compiler::{BlkJITCompileError, Block2JITCompiler};
//    use hexodsp::blocklang::BlockFun;
//    use hexodsp::blocklang_def;
//
//    let lang = blocklang_def::setup_hxdsp_block_language();
//    let mut bf = BlockFun::new(lang.clone());
//    block_fun.instanciate_at(0, 0, 1, "value", Some("0.3".to_string()));
//    block_fun.instanciate_at(0, 1, 1, "set", Some("&sig1".to_string()));
//
//    let mut compiler = Block2JITCompiler::new(block_fun.block_language());
//    let ast = compiler.compile(&block_fun)?;
//    let lib = synfx_dsp_jit::get_standard_library();
//    let ctx = synfx_dsp_jit::DSPNodeContext::new_ref();
//    let jit = JIT::new(lib, dsp_ctx.clone());
//    let fun = jit.compile(ASTFun::new(ast))?;
//
//    fun.init(44100.0, None);
//
//    let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
//
//    ctx.borrow_mut().free();
//}
//
//// XXX: Test case with 3 outputs, where the first output writes a value used
////   by the computation after the first but before the third output.
//
//    0.3 ->3  set a
//             => ->    + set b
//                get a
//             => ->    - set a
//                get b
//    get a +
//    get b
//*/
//
////#[test]
////fn check_blocklang_2() {
////    let (mut matrix, mut node_exec) = setup();
////
////    let block_fun = matrix.get_block_function(0).expect("block fun exists");
////    {
////        let mut block_fun = block_fun.lock().expect("matrix lock");
////
////        block_fun.instanciate_at(0, 0, 0, "get", Some("in1".to_string()));
////        block_fun.instanciate_at(0, 0, 1, "value", Some("0.3".to_string()));
////        block_fun.instanciate_at(0, 1, 0, "+", None);
////        block_fun.instanciate_at(0, 2, 0, "set", Some("&sig1".to_string()));
////
////        block_fun.instanciate_at(0, 3, 0, "get", Some("in1".to_string()));
////        block_fun.instanciate_at(0, 3, 1, "get", Some("in2".to_string()));
////        block_fun.instanciate_at(0, 4, 0, "-", None);
////        block_fun.instanciate_at(0, 5, 0, "->3", None);
////
////        block_fun.instanciate_at(0, 3, 5, "get", Some("in1".to_string()));
////        block_fun.instanciate_at(0, 4, 5, "if", None);
////        block_fun.instanciate_at(1, 0, 0, "value", Some("0.5".to_string()));
////        block_fun.instanciate_at(2, 0, 0, "value", Some("-0.5".to_string()));
////
////        block_fun.instanciate_at(0, 6, 1, "set", Some("*a".to_string()));
////        block_fun.instanciate_at(0, 6, 2, "set", Some("x".to_string()));
////        block_fun.instanciate_at(0, 6, 0, "->", None);
////        block_fun.instanciate_at(0, 7, 0, "->2", None);
////
////        block_fun.instanciate_at(0, 0, 3, "get", Some("in1".to_string()));
////        block_fun.instanciate_at(0, 0, 4, "get", Some("in2".to_string()));
////        block_fun.instanciate_at(0, 1, 3, "/%", None);
////        block_fun.instanciate_at(0, 2, 3, "->", None);
////        block_fun.instanciate_at(0, 3, 3, "/%", None);
////        block_fun.instanciate_at(0, 4, 3, "set", Some("&sig2".to_string()));
////        block_fun.instanciate_at(0, 4, 4, "set", Some("*ap".to_string()));
////    }
////
////    matrix.check_block_function(0).expect("no compile error");
////
////    let res = run_for_ms(&mut node_exec, 25.0);
////    assert_decimated_feq!(res.0, 50, vec![0.2; 100]);
////}
