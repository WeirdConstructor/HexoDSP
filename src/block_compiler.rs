// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::blocklang::*;
use synfx_dsp_jit::ASTNode;

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

type BlkASTRef = Rc<RefCell<BlkASTNode>>;

#[derive(Debug, Clone)]
enum BlkASTNode {
    Root {
        child: BlkASTRef,
    },
    Area {
        childs: Vec<BlkASTRef>,
    },
    Set {
        var: String,
        expr: BlkASTRef,
    },
    Get {
        id: usize,
        use_count: usize,
        var: String,
    },
    Node {
        id: usize,
        out: Option<String>,
        use_count: usize,
        typ: String,
        lbl: String,
        childs: Vec<(Option<String>, BlkASTRef)>,
    },
}

impl BlkASTNode {
    pub fn dump(&self, indent: usize, inp: Option<&str>) -> String {
        let mut indent_str = "   ".repeat(indent + 1);

        if let Some(inp) = inp {
            indent_str += &format!("{}<= ", inp);
        }

        match self {
            BlkASTNode::Root { child } => {
                format!("{}* Root\n", indent_str) + &child.borrow().dump(indent + 1, None)
            }
            BlkASTNode::Area { childs } => {
                let mut s = format!("{}* Area\n", indent_str);
                for c in childs.iter() {
                    s += &c.borrow().dump(indent + 1, None);
                }
                s
            }
            BlkASTNode::Set { var, expr } => {
                format!("{}set '{}'=\n", indent_str, var) + &expr.borrow().dump(indent + 1, None)
            }
            BlkASTNode::Get { id, use_count, var } => {
                format!("{}get '{}' (id={}, use={})\n", indent_str, var, id, use_count)
            }
            BlkASTNode::Node { id, out, use_count, typ, lbl, childs } => {
                let lbl = if *typ == *lbl { "".to_string() } else { format!("[{}]", lbl) };

                let mut s = if let Some(out) = out {
                    format!(
                        "{}{}{} (id={}/{}, use={})\n",
                        indent_str, typ, lbl, id, out, use_count
                    )
                } else {
                    format!("{}{}{} (id={}, use={})\n", indent_str, typ, lbl, id, use_count)
                };
                for (inp, c) in childs.iter() {
                    s += &format!("{}", c.borrow().dump(indent + 1, inp.as_ref().map(|s| &s[..])));
                }
                s
            }
        }
    }

    pub fn new_root(child: BlkASTRef) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Root { child }))
    }

    pub fn new_area(childs: Vec<BlkASTRef>) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Area { childs }))
    }

    pub fn new_set(var: &str, expr: BlkASTRef) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Set { var: var.to_string(), expr }))
    }

    pub fn new_get(id: usize, var: &str) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Get { id, var: var.to_string(), use_count: 1 }))
    }

    pub fn new_node(
        id: usize,
        out: Option<String>,
        typ: &str,
        lbl: &str,
        childs: Vec<(Option<String>, BlkASTRef)>,
    ) -> BlkASTRef {
        Rc::new(RefCell::new(BlkASTNode::Node {
            id,
            out,
            typ: typ.to_string(),
            lbl: lbl.to_string(),
            use_count: 1,
            childs,
        }))
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
        Self { id_node_map: HashMap::new() }
    }

    pub fn trans2bjit(
        &self,
        node: &ASTNodeRef,
        my_out: Option<String>,
    ) -> Result<BlkASTRef, BlkJITCompileError> {
        // TODO: Deal with multiple outputs.
        // If we encounter a node with multiple outputs, assign each output
        // to a temporary variable and save that.
        // Store the name of the temporary in a id+output mapping.
        // => XXX
        // That means: If we have a single output, things are easy, just plug them into
        //             the JIT ast:
        //                  outer(inner())
        //             But if we have multiple outputs:
        //                  assign(a = inner())
        //                  assign(b = %1)
        //                  outer_x(a)
        //                  outer_y(b)

        // TODO: Filter out -> nodes from the AST
        // TODO: For ->2 and ->3, save the input in some variable
        //       and reserve a id+output variable for this.

        // XXX: SSA form of cranelift should take care of the rest!

        match &node.0.borrow().typ[..] {
            "<r>" => {
                if let Some((_in, out, first)) = node.first_child() {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    let child = self.trans2bjit(&first, out)?;
                    Ok(BlkASTNode::new_root(child))
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
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
            "<res>" => {
                // TODO: handle results properly, like remembering the most recent result
                // and append it to the end of the statements block. so that a temporary
                // variable is created.
                if let Some((_in, out, first)) = node.first_child() {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    self.trans2bjit(&first, out)
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            "set" => {
                if let Some((_in, out, first)) = node.first_child() {
                    let out = if out.len() > 0 { Some(out) } else { None };
                    let expr = self.trans2bjit(&first, out)?;
                    Ok(BlkASTNode::new_set(&node.0.borrow().lbl, expr))
                } else {
                    Err(BlkJITCompileError::BadTree(node.clone()))
                }
            }
            "get" => Ok(BlkASTNode::new_get(node.0.borrow().id, &node.0.borrow().lbl)),
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

                // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
                // TODO: Check here if the optype has multiple outputs.
                //       when it has, make a sub-collection of statements
                //       and make temporary variables with ::Set
                //       then return the output with a final ::Get to the
                //       output "my_out".
                //       If no output is given in "my_out" it's an error!
                //""""""""""""""""""""""""""""""""""""""""""""""""""""""

                // TODO: Reorder the childs/arguments according to the input
                //       order in the BlockLanguage

                Ok(BlkASTNode::new_node(
                    node.0.borrow().id,
                    my_out,
                    &node.0.borrow().typ,
                    &node.0.borrow().lbl,
                    childs,
                ))
            }
        }
    }

    pub fn compile(&self, fun: &BlockFun) -> Result<ASTNode, BlkJITCompileError> {
        let tree = fun.generate_tree::<ASTNodeRef>("zero").unwrap();
        println!("{}", tree.walk_dump("", "", 0));

        let blkast = self.trans2bjit(&tree, None);
        println!("R: {}", blkast.unwrap().borrow().dump(0, None));

        Err(BlkJITCompileError::UnknownError)
    }
}
