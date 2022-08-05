// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::blocklang::*;
#[cfg(feature = "synfx-dsp-jit")]
use synfx_dsp_jit::{ASTNode, JITCompileError};

#[derive(Debug)]
struct JASTNode {
    id: usize,
    typ: String,
    lbl: String,
    nodes: Vec<(String, String, ASTNodeRef)>,
}

#[derive(Debug, Clone)]
pub struct ASTNodeRef(Rc<RefCell<JASTNode>>);

impl BlockASTNode for ASTNodeRef {
    fn from(id: usize, typ: &str, lbl: &str) -> ASTNodeRef {
        ASTNodeRef(Rc::new(RefCell::new(JASTNode {
            id,
            typ: typ.to_string(),
            lbl: lbl.to_string(),
            nodes: vec![],
        })))
    }

    fn add_node(&self, in_port: String, out_port: String, node: ASTNodeRef) {
        self.0.borrow_mut().nodes.push((in_port, out_port, node));
    }
}

impl ASTNodeRef {
    pub fn first_child_ref(&self) -> Option<ASTNodeRef> {
        self.0.borrow().nodes.get(0).map(|n| n.2.clone())
    }

    pub fn first_child(&self) -> Option<(String, String, ASTNodeRef)> {
        self.0.borrow().nodes.get(0).cloned()
    }

    pub fn nth_child(&self, i: usize) -> Option<(String, String, ASTNodeRef)> {
        self.0.borrow().nodes.get(i).cloned()
    }

    pub fn walk_dump(&self, input: &str, output: &str, indent: usize) -> String {
        let indent_str = "   ".repeat(indent + 1);

        let out_port = if output.len() > 0 { format!("(out: {})", output) } else { "".to_string() };
        let in_port = if input.len() > 0 { format!("(in: {})", input) } else { "".to_string() };

        let mut s = format!(
            "{}{}#{}[{}] {}{}\n",
            indent_str,
            self.0.borrow().id,
            self.0.borrow().typ,
            self.0.borrow().lbl,
            out_port,
            in_port
        );

        for (inp, out, n) in &self.0.borrow().nodes {
            s += &n.walk_dump(&inp, &out, indent + 1);
        }

        s
    }
}

type BlkASTRef = Rc<BlkASTNode>;

#[derive(Debug, Clone)]
enum BlkASTNode {
    Area {
        childs: Vec<BlkASTRef>,
    },
    Set {
        var: String,
        expr: BlkASTRef,
    },
    Get {
        id: usize,
        var: String,
    },
    Node {
        id: usize,
        out: Option<String>,
        typ: String,
        lbl: String,
        childs: Vec<(Option<String>, BlkASTRef)>,
    },
    Literal {
        value: f64,
    },
}

impl BlkASTNode {
    pub fn dump(&self, indent: usize, inp: Option<&str>) -> String {
        let mut indent_str = "   ".repeat(indent + 1);

        if let Some(inp) = inp {
            indent_str += &format!("{}<= ", inp);
        } else {
            indent_str += "<= ";
        }

        match self {
            BlkASTNode::Area { childs } => {
                let mut s = format!("{}Area\n", indent_str);
                for c in childs.iter() {
                    s += &c.dump(indent + 1, None);
                }
                s
            }
            BlkASTNode::Set { var, expr } => {
                format!("{}set '{}'=\n", indent_str, var) + &expr.dump(indent + 1, None)
            }
            BlkASTNode::Get { id, var } => {
                format!("{}get '{}' (id={})\n", indent_str, var, id)
            }
            BlkASTNode::Literal { value } => {
                format!("{}{}\n", indent_str, value)
            }
            BlkASTNode::Node { id, out, typ, lbl, childs } => {
                let lbl = if *typ == *lbl { "".to_string() } else { format!("[{}]", lbl) };

                let mut s = if let Some(out) = out {
                    format!("{}{}{} (id={}/{})\n", indent_str, typ, lbl, id, out)
                } else {
                    format!("{}{}{} (id={})\n", indent_str, typ, lbl, id)
                };
                for (inp, c) in childs.iter() {
                    s += &format!("{}", c.dump(indent + 1, inp.as_ref().map(|s| &s[..])));
                }
                s
            }
        }
    }

    pub fn new_area(childs: Vec<BlkASTRef>) -> BlkASTRef {
        Rc::new(BlkASTNode::Area { childs })
    }

    pub fn new_set(var: &str, expr: BlkASTRef) -> BlkASTRef {
        Rc::new(BlkASTNode::Set { var: var.to_string(), expr })
    }

    pub fn new_get(id: usize, var: &str) -> BlkASTRef {
        Rc::new(BlkASTNode::Get { id, var: var.to_string() })
    }

    pub fn new_literal(val: &str) -> Result<BlkASTRef, BlkJITCompileError> {
        if let Ok(value) = val.parse::<f64>() {
            Ok(Rc::new(BlkASTNode::Literal { value }))
        } else {
            Err(BlkJITCompileError::BadLiteralNumber(val.to_string()))
        }
    }

    pub fn new_node(
        id: usize,
        out: Option<String>,
        typ: &str,
        lbl: &str,
        childs: Vec<(Option<String>, BlkASTRef)>,
    ) -> BlkASTRef {
        Rc::new(BlkASTNode::Node { id, out, typ: typ.to_string(), lbl: lbl.to_string(), childs })
    }
}

#[derive(Debug, Clone)]
pub enum BlkJITCompileError {
    UnknownError,
    NoSynfxDSPJit,
    BadTree(ASTNodeRef),
    NoOutputAtIdx(String, usize),
    ASTMissingOutputLabel(usize),
    NoTmpVarForOutput(usize, String),
    BadLiteralNumber(String),
    NodeWithoutID(String),
    UnknownType(String),
    TooManyInputs(String, usize),
    WrongNumberOfChilds(String, usize, usize),
    UnassignedInput(String, usize, String),
    #[cfg(feature = "synfx-dsp-jit")]
    JITCompileError(JITCompileError),
}

pub struct Block2JITCompiler {
    idout_var_map: HashMap<String, String>,
    lang: Rc<RefCell<BlockLanguage>>,
    tmpvar_counter: usize,
}

// 1. compile the weird tree into a graph
//   - make references where IDs go
//   - add a use count to each node, so that we know when to make temporary variables

#[cfg(not(feature = "synfx-dsp-jit"))]
enum ASTNode {
    NoSynfxDSPJit
}

impl Block2JITCompiler {
    pub fn new(lang: Rc<RefCell<BlockLanguage>>) -> Self {
        Self { idout_var_map: HashMap::new(), lang, tmpvar_counter: 0 }
    }

    pub fn next_tmpvar_name(&mut self, extra: &str) -> String {
        self.tmpvar_counter += 1;
        format!("_tmp{}_{}_", self.tmpvar_counter, extra)
    }

    pub fn store_idout_var(&mut self, id: usize, out: &str, v: &str) {
        self.idout_var_map.insert(format!("{}/{}", id, out), v.to_string());
    }

    pub fn get_var_for_idout(&self, id: usize, out: &str) -> Option<&str> {
        self.idout_var_map.get(&format!("{}/{}", id, out)).map(|s| &s[..])
    }

    fn trans2bjit(
        &mut self,
        node: &ASTNodeRef,
        my_out: Option<String>,
    ) -> Result<BlkASTRef, BlkJITCompileError> {
        let id = node.0.borrow().id;

        if let Some(out) = &my_out {
            if let Some(tmpvar) = self.get_var_for_idout(id, out) {
                return Ok(BlkASTNode::new_get(0, tmpvar));
            }
        } else {
            if let Some(tmpvar) = self.get_var_for_idout(id, "") {
                return Ok(BlkASTNode::new_get(0, tmpvar));
            }
        }

        match &node.0.borrow().typ[..] {
            "<a>" => {
                let mut childs = vec![];

                let mut i = 0;
                while let Some((_in, out, child)) = node.nth_child(i) {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    let child = self.trans2bjit(&child, out)?;
                    childs.push(child);
                    i += 1;
                }

                Ok(BlkASTNode::new_area(childs))
            }
            // TODO: handle results properly, like remembering the most recent result
            // and append it to the end of the statements block. so that a temporary
            // variable is created.
            "<r>" => {
                if let Some((_in, out, first)) = node.first_child() {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    let childs =
                        vec![self.trans2bjit(&first, out)?, BlkASTNode::new_get(0, "_res_")];
                    Ok(BlkASTNode::new_area(childs))
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            "->" => {
                if let Some((_in, out, first)) = node.first_child() {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    self.trans2bjit(&first, out)
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            "value" => Ok(BlkASTNode::new_literal(&node.0.borrow().lbl)?),
            "set" | "<res>" => {
                if let Some((_in, out, first)) = node.first_child() {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    let expr = self.trans2bjit(&first, out)?;
                    if &node.0.borrow().typ[..] == "<res>" {
                        Ok(BlkASTNode::new_set("_res_", expr))
                    } else {
                        Ok(BlkASTNode::new_set(&node.0.borrow().lbl, expr))
                    }
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            "get" => Ok(BlkASTNode::new_get(id, &node.0.borrow().lbl)),
            "->2" | "->3" => {
                if let Some((_in, out, first)) = node.first_child() {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    let mut area = vec![];
                    let tmp_var = self.next_tmpvar_name("");
                    let expr = self.trans2bjit(&first, out)?;
                    area.push(BlkASTNode::new_set(&tmp_var, expr));
                    area.push(BlkASTNode::new_get(0, &tmp_var));
                    self.store_idout_var(id, "", &tmp_var);
                    Ok(BlkASTNode::new_area(area))
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            optype => {
                let mut childs = vec![];

                let mut i = 0;
                while let Some((inp, out, child)) = node.nth_child(i) {
                    let out = if out.len() > 0 { Some(out) } else { None };

                    let child = self.trans2bjit(&child, out)?;
                    if inp.len() > 0 {
                        childs.push((Some(inp.to_string()), child));
                    } else {
                        childs.push((None, child));
                    }
                    i += 1;
                }

                // TODO: Reorder the childs/arguments according to the input
                //       order in the BlockLanguage

                let cnt = self.lang.borrow().type_output_count(optype);
                if cnt > 1 {
                    let mut area = vec![];

                    let oname = self.lang.borrow().get_output_name_at_index(optype, 0);

                    if let Some(oname) = oname {
                        let tmp_var = self.next_tmpvar_name(&oname);

                        area.push(BlkASTNode::new_set(
                            &tmp_var,
                            BlkASTNode::new_node(
                                id,
                                my_out.clone(),
                                &node.0.borrow().typ,
                                &node.0.borrow().lbl,
                                childs,
                            ),
                        ));
                        self.store_idout_var(id, &oname, &tmp_var);
                    } else {
                        return Err(BlkJITCompileError::NoOutputAtIdx(optype.to_string(), 0));
                    }

                    for i in 1..cnt {
                        let oname = self.lang.borrow().get_output_name_at_index(optype, i);

                        if let Some(oname) = oname {
                            let tmp_var = self.next_tmpvar_name(&oname);

                            area.push(BlkASTNode::new_set(
                                &tmp_var,
                                BlkASTNode::new_get(0, &format!("%{}", i)),
                            ));

                            self.store_idout_var(id, &oname, &tmp_var);
                        } else {
                            return Err(BlkJITCompileError::NoOutputAtIdx(optype.to_string(), i));
                        }
                    }

                    if let Some(out) = &my_out {
                        if let Some(tmpvar) = self.get_var_for_idout(id, out) {
                            area.push(BlkASTNode::new_get(0, tmpvar));
                        } else {
                            return Err(BlkJITCompileError::NoTmpVarForOutput(id, out.to_string()));
                        }
                    } else {
                        return Err(BlkJITCompileError::ASTMissingOutputLabel(id));
                    }

                    Ok(BlkASTNode::new_area(area))
                } else {
                    Ok(BlkASTNode::new_node(
                        id,
                        my_out,
                        &node.0.borrow().typ,
                        &node.0.borrow().lbl,
                        childs,
                    ))
                }
            }
        }
    }

    #[cfg(feature = "synfx-dsp-jit")]
    fn bjit2jit(&mut self, ast: &BlkASTRef) -> Result<Box<ASTNode>, BlkJITCompileError> {
        use synfx_dsp_jit::build::*;

        match &**ast {
            BlkASTNode::Area { childs } => {
                let mut stmt = vec![];
                for c in childs.iter() {
                    stmt.push(self.bjit2jit(&c)?);
                }
                Ok(stmts(&stmt[..]))
            }
            BlkASTNode::Set { var, expr } => {
                let e = self.bjit2jit(&expr)?;
                Ok(assign(var, e))
            }
            BlkASTNode::Get { var: varname, .. } => Ok(var(varname)),
            BlkASTNode::Node { id, typ, childs, .. } => match &typ[..] {
                "if" => Err(BlkJITCompileError::UnknownError),
                "zero" => Ok(literal(0.0)),
                _ => {
                    if *id == 0 {
                        return Err(BlkJITCompileError::NodeWithoutID(typ.to_string()));
                    }

                    let lang = self.lang.clone();

                    let mut args = vec![];

                    if let Some(inputs) = lang.borrow().get_type_inputs(typ) {
                        if childs.len() > inputs.len() {
                            return Err(BlkJITCompileError::TooManyInputs(typ.to_string(), *id));
                        }

                        if inputs.len() > 0 && inputs[0] == Some("".to_string()) {
                            if inputs.len() != childs.len() {
                                return Err(BlkJITCompileError::WrongNumberOfChilds(
                                    typ.to_string(),
                                    *id,
                                    childs.len(),
                                ));
                            }

                            // We assume all inputs are unnamed:
                            for (_inp, c) in childs.iter() {
                                args.push(self.bjit2jit(&c)?);
                            }
                        } else {
                            // We assume all inputs are named:
                            for input_name in inputs.iter() {
                                let mut found = false;
                                for (inp, c) in childs.iter() {
                                    println!("FOFOFO '{:?}' = '{:?}'", inp, input_name);
                                    if inp == input_name {
                                        args.push(self.bjit2jit(&c)?);
                                        found = true;
                                        break;
                                    }
                                }

                                if !found {
                                    return Err(BlkJITCompileError::UnassignedInput(
                                        typ.to_string(),
                                        *id,
                                        format!("{:?}", input_name),
                                    ));
                                }
                            }
                        }
                    } else {
                        return Err(BlkJITCompileError::UnknownType(typ.to_string()));
                    }

                    match &typ[..] {
                        "+" | "*" | "-" | "/" => {
                            if args.len() != 2 {
                                return Err(BlkJITCompileError::WrongNumberOfChilds(
                                    typ.to_string(),
                                    *id,
                                    args.len(),
                                ));
                            }

                            let a = args.remove(0);
                            let b = args.remove(0);

                            match &typ[..] {
                                "+" => Ok(op_add(a, b)),
                                "*" => Ok(op_mul(a, b)),
                                "-" => Ok(op_sub(a, b)),
                                "/" => Ok(op_div(a, b)),
                                _ => Err(BlkJITCompileError::UnknownType(typ.to_string())),
                            }
                        }
                        _ => Ok(call(typ, *id as u64, &args[..])),
                    }
                }
            },
            BlkASTNode::Literal { value } => Ok(literal(*value)),
        }
    }

    pub fn compile(&mut self, fun: &BlockFun) -> Result<Box<ASTNode>, BlkJITCompileError> {
        #[cfg(feature = "synfx-dsp-jit")]
        {
            let tree = fun.generate_tree::<ASTNodeRef>("zero").unwrap();
            println!("{}", tree.walk_dump("", "", 0));

            let blkast = self.trans2bjit(&tree, None)?;
            println!("R: {}", blkast.dump(0, None));

            self.bjit2jit(&blkast)
        }
        #[cfg(not(feature = "synfx-dsp-jit"))]
        {
            Err(BlkJITCompileError::NoSynfxDSPJit)
        }
    }
}

#[cfg(feature = "synfx-dsp-jit")]
#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_float_eq {
        ($a:expr, $b:expr) => {
            if ($a - $b).abs() > 0.0001 {
                panic!(
                    r#"assertion failed: `(left == right)`
      left: `{:?}`,
     right: `{:?}`"#,
                    $a, $b
                )
            }
        };
    }

    use synfx_dsp_jit::{get_standard_library, ASTFun, DSPFunction, DSPNodeContext, JIT};

    fn new_jit_fun<F: FnMut(&mut BlockFun)>(
        mut f: F,
    ) -> (Rc<RefCell<DSPNodeContext>>, Box<DSPFunction>) {
        use crate::block_compiler::{BlkJITCompileError, Block2JITCompiler};
        use crate::blocklang::BlockFun;
        use crate::blocklang_def;

        let lib = get_standard_library();
        let lang = blocklang_def::setup_hxdsp_block_language(lib.clone());
        let mut bf = BlockFun::new(lang.clone());

        f(&mut bf);

        let mut compiler = Block2JITCompiler::new(bf.block_language());
        let ast = compiler.compile(&bf).expect("blk2jit compiles");
        let ctx = DSPNodeContext::new_ref();
        let jit = JIT::new(lib, ctx.clone());
        let mut fun = jit.compile(ASTFun::new(ast)).expect("jit compiles");

        fun.init(44100.0, None);

        (ctx, fun)
    }

    #[test]
    fn check_blocklang_sig1() {
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.3".to_string())).unwrap();
            bf.instanciate_at(0, 1, 1, "set", Some("&sig1".to_string())).unwrap();
            bf.instanciate_at(0, 0, 2, "value", Some("-0.3".to_string())).unwrap();
            bf.instanciate_at(0, 1, 2, "set", Some("&sig2".to_string())).unwrap();
            bf.instanciate_at(0, 0, 3, "value", Some("-1.3".to_string())).unwrap();
        });

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);

        assert_float_eq!(s1, 0.3);
        assert_float_eq!(s2, -0.3);
        assert_float_eq!(ret, -1.3);

        ctx.borrow_mut().free();
    }

    #[test]
    fn check_blocklang_accum_shift() {
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 1, 1, "accum", None);
            bf.shift_port(0, 1, 1, 1, false);
            bf.instanciate_at(0, 0, 2, "value", Some("0.01".to_string()));
            bf.instanciate_at(0, 0, 1, "get", Some("*reset".to_string()));
        });

        fun.exec_2in_2out(0.0, 0.0);
        fun.exec_2in_2out(0.0, 0.0);
        fun.exec_2in_2out(0.0, 0.0);
        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, 0.04);

        let reset_idx = ctx.borrow().get_persistent_variable_index_by_name("*reset").unwrap();
        fun.access_persistent_var(reset_idx).map(|reset| *reset = 1.0);

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, 0.0);

        fun.access_persistent_var(reset_idx).map(|reset| *reset = 0.0);

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, 0.05);

        ctx.borrow_mut().free();
    }

    #[test]
    fn check_blocklang_arithmetics() {
        // Check + and *
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.50".to_string()));
            bf.instanciate_at(0, 0, 2, "value", Some("0.01".to_string()));
            bf.instanciate_at(0, 1, 1, "+", None);
            bf.shift_port(0, 1, 1, 1, true);
            bf.instanciate_at(0, 1, 3, "value", Some("2.0".to_string()));
            bf.instanciate_at(0, 2, 2, "*", None);
        });

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, 1.02);
        ctx.borrow_mut().free();

        // Check - and /
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.50".to_string()));
            bf.instanciate_at(0, 0, 2, "value", Some("0.01".to_string()));
            bf.instanciate_at(0, 1, 1, "-", None);
            bf.shift_port(0, 1, 1, 1, true);
            bf.instanciate_at(0, 1, 3, "value", Some("2.0".to_string()));
            bf.instanciate_at(0, 2, 2, "/", None);
        });

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, (0.5 - 0.01) / 2.0);
        ctx.borrow_mut().free();

        // Check swapping inputs of "-"
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.50".to_string()));
            bf.instanciate_at(0, 0, 2, "value", Some("0.01".to_string()));
            bf.instanciate_at(0, 1, 1, "-", None);
            bf.shift_port(0, 1, 1, 1, false);
        });

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, 0.01 - 0.5);
        ctx.borrow_mut().free();

        // Check swapping inputs of "/"
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.50".to_string()));
            bf.instanciate_at(0, 0, 2, "value", Some("0.01".to_string()));
            bf.instanciate_at(0, 1, 1, "/", None);
            bf.shift_port(0, 1, 1, 1, false);
        });

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, 0.01 / 0.5);
        ctx.borrow_mut().free();

        // Check division of 0.0
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.50".to_string()));
            bf.instanciate_at(0, 0, 2, "value", Some("0.0".to_string()));
            bf.instanciate_at(0, 1, 1, "/", None);
        });

        let (s1, s2, ret) = fun.exec_2in_2out(0.0, 0.0);
        assert_float_eq!(ret, 0.5 / 0.0);
        ctx.borrow_mut().free();
    }

    #[test]
    fn check_blocklang_divrem() {
        // &sig1 on second output:
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.3".to_string())).unwrap();
            bf.instanciate_at(0, 0, 2, "value", Some("-0.4".to_string())).unwrap();
            bf.instanciate_at(0, 1, 1, "/%", None);
            bf.instanciate_at(0, 2, 2, "set", Some("&sig1".to_string())).unwrap();
        });

        let (s1, _, ret) = fun.exec_2in_2out(0.0, 0.0);

        assert_float_eq!(s1, 0.3);
        assert_float_eq!(ret, -0.75);
        ctx.borrow_mut().free();

        // &sig1 on first output:
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.3".to_string())).unwrap();
            bf.instanciate_at(0, 0, 2, "value", Some("-0.4".to_string())).unwrap();
            bf.instanciate_at(0, 1, 1, "/%", None);
            bf.instanciate_at(0, 2, 1, "set", Some("&sig1".to_string())).unwrap();
        });

        let (s1, _, ret) = fun.exec_2in_2out(0.0, 0.0);

        assert_float_eq!(ret, 0.3);
        assert_float_eq!(s1, -0.75);
        ctx.borrow_mut().free();

        // &sig1 on second output, but swapped outputs:
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.3".to_string())).unwrap();
            bf.instanciate_at(0, 0, 2, "value", Some("-0.4".to_string())).unwrap();
            bf.instanciate_at(0, 1, 1, "/%", None);
            bf.instanciate_at(0, 2, 2, "set", Some("&sig1".to_string())).unwrap();
            bf.shift_port(0, 1, 1, 0, true);
        });

        let (s1, _, ret) = fun.exec_2in_2out(0.0, 0.0);

        assert_float_eq!(ret, 0.3);
        assert_float_eq!(s1, -0.75);
        ctx.borrow_mut().free();

        // &sig1 on first output, but swapped outputs:
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.3".to_string())).unwrap();
            bf.instanciate_at(0, 0, 2, "value", Some("-0.4".to_string())).unwrap();
            bf.instanciate_at(0, 1, 1, "/%", None);
            bf.instanciate_at(0, 2, 1, "set", Some("&sig1".to_string())).unwrap();
            bf.shift_port(0, 1, 1, 0, true);
        });

        let (s1, _, ret) = fun.exec_2in_2out(0.0, 0.0);

        assert_float_eq!(s1, 0.3);
        assert_float_eq!(ret, -0.75);
        ctx.borrow_mut().free();

        // &sig1 on first output, but swapped inputs:
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.3".to_string())).unwrap();
            bf.instanciate_at(0, 0, 2, "value", Some("-0.4".to_string())).unwrap();
            bf.instanciate_at(0, 1, 1, "/%", None);
            bf.instanciate_at(0, 2, 1, "set", Some("&sig1".to_string())).unwrap();
            bf.shift_port(0, 1, 1, 0, false);
        });

        let (s1, _, ret) = fun.exec_2in_2out(0.0, 0.0);

        assert_float_eq!(s1, -1.33333);
        assert_float_eq!(ret, -0.1);
        ctx.borrow_mut().free();

        // &sig1 on first output, but swapped inputs and outputs:
        let (ctx, mut fun) = new_jit_fun(|bf| {
            bf.instanciate_at(0, 0, 1, "value", Some("0.3".to_string())).unwrap();
            bf.instanciate_at(0, 0, 2, "value", Some("-0.4".to_string())).unwrap();
            bf.instanciate_at(0, 1, 1, "/%", None);
            bf.instanciate_at(0, 2, 1, "set", Some("&sig1".to_string())).unwrap();
            bf.shift_port(0, 1, 1, 0, false);
            bf.shift_port(0, 1, 1, 0, true);
        });

        let (s1, _, ret) = fun.exec_2in_2out(0.0, 0.0);

        assert_float_eq!(ret, -1.33333);
        assert_float_eq!(s1, -0.1);
        ctx.borrow_mut().free();
    }
}
