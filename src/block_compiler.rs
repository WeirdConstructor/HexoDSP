// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use synfx_dsp_jit::ASTNode;
use crate::blocklang::*;

#[derive(Debug)]
struct JASTNode {
    id:    usize,
    typ:   String,
    lbl:   String,
    nodes: Vec<(String, String, ASTNodeRef)>,
}

#[derive(Debug, Clone)]
pub struct ASTNodeRef(Rc<RefCell<JASTNode>>);

impl BlockASTNode for ASTNodeRef {
    fn from(id: usize, typ: &str, lbl: &str) -> ASTNodeRef {
        ASTNodeRef(Rc::new(RefCell::new(JASTNode {
            id,
            typ:    typ.to_string(),
            lbl:    lbl.to_string(),
            nodes:  vec![],
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

        let out_port =
            if output.len() > 0 { format!("(out: {})", output) }
            else { "".to_string() };
        let in_port =
            if input.len() > 0 { format!("(in: {})", input) }
            else { "".to_string() };

        let mut s = format!(
            "{}{}#{}[{}] {}{}\n",
            indent_str, self.0.borrow().id, self.0.borrow().typ,
            self.0.borrow().lbl, out_port, in_port);

        for (inp, out, n) in &self.0.borrow().nodes {
            s += &n.walk_dump(&inp, &out, indent + 1);
        }

        s
    }
}

type BlkASTRef = Rc<RefCell<BlkASTNode>>;

#[derive(Debug, Clone)]
enum BlkASTNode {
    Root { child: BlkASTRef },
    Area { childs: Vec<BlkASTRef> },
    Set { var: String, expr: BlkASTRef },
    Get { id: usize, use_count: usize, var: String, expr: BlkASTRef },
    Node { id: usize, use_count: usize, typ: String, lbl: String, childs: Vec<BlkASTRef> },
}

impl BlkASTNode {
    pub fn new_root(child: BlkASTRef) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Root { child }))
    }

    pub fn new_area(childs: Vec<BlkASTRef>) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Area { childs }))
    }

    pub fn new_set(var: &str, expr: BlkASTRef) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Set { var: var.to_string(), expr }))
    }

    pub fn new_node(id: usize, typ: &str, lbl: &str, childs: Vec<BlkASTRef>) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Node { id, typ: typ.to_string(), lbl: lbl.to_string(), use_count: 1, childs }))
    }
}


#[derive(Debug, Clone)]
pub enum BlkJITCompileError {
    UnknownError,
    BadTree(ASTNodeRef),
}

pub struct Block2JITCompiler {
    id_node_map: HashMap<usize, BlkASTRef>,
}

// 1. compile the weird tree into a graph
//   - make references where IDs go
//   - add a use count to each node, so that we know when to make temporary variables

impl Block2JITCompiler {
    pub fn new() -> Self {
        Self {
            id_node_map: HashMap::new(),
        }
    }

    pub fn trans2bjit(&self, node: &ASTNodeRef) -> Result<BlkASTRef, BlkJITCompileError> {
        match &node.0.borrow().typ[..] {
            "<r>" => {
                if let Some(first) = node.first_child_ref() {
                    let child = self.trans2bjit(&first)?;
                    Ok(BlkASTNode::new_root(child))
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            "<a>" => {
                let mut childs = vec![];

                let mut i = 0;
                while let Some((_in, _out, child)) = node.nth_child(i) {
                    let child = self.trans2bjit(&child)?;
                    childs.push(child);
                    i += 1;
                }

                Ok(BlkASTNode::new_area(childs))
            }
            "<res>" => {
                // TODO: handle results properly, like remembering the most recent result
                // and append it to the end of the statements block. so that a temporary
                // variable is created.
                if let Some(first) = node.first_child_ref() {
                    self.trans2bjit(&first)
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            "set" => {
                if let Some(first) = node.first_child_ref() {
                    let expr = self.trans2bjit(&first)?;
                    Ok(BlkASTNode::new_set(&node.0.borrow().lbl, expr))

                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            optype => {
                let mut childs = vec![];

                let mut i = 0;
                while let Some((_in, _out, child)) = node.nth_child(i) {
                    let child = self.trans2bjit(&child)?;
                    childs.push(child);
                    i += 1;
                }

                Ok(BlkASTNode::new_node(
                    node.0.borrow().id,
                    &node.0.borrow().typ,
                    &node.0.borrow().lbl,
                    childs))
            }
        }
    }

    pub fn compile(&self, fun: &BlockFun) -> Result<ASTNode, BlkJITCompileError> {
        let tree = fun.generate_tree::<ASTNodeRef>("zero").unwrap();
        println!("{}", tree.walk_dump("", "", 0));

        let blkast = self.trans2bjit(&tree);
        println!("R: {:#?}", blkast);

        Err(BlkJITCompileError::UnknownError)
    }
}
