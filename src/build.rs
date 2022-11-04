// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

/*! Node Reference and Graph Builder for [crate::SynthConstructor]

This module contains function bindings for all DSP nodes that are included
in HexoDSP. All nodes, their inputs, settings and outputs are accessible via this API.
You can write type safe and compile time checked graphs for HexoDSP using this API.

Below you will find a comprehensive reference of all HexoDSP nodes, their
parameters and some of the possible values.

## HexoDSP DSP Node Reference
*/
#![allow(rustdoc::invalid_rust_codeblocks)]
#![doc = include_str!("dsp_nodes_ref.md")]

use crate::node_list;

macro_rules! make_node_constructor {
    ($s1: ident => $v1: ident,
        $($str: ident => $variant: ident
            UIType:: $gui_type: ident
            UICategory:: $ui_cat: ident
            $(($in_idx: literal $para: ident
               $n_fun: ident $d_fun: ident $r_fun: ident $f_fun: ident
               $steps: ident $min: expr, $max: expr, $def: expr))*
            $({$in_at_idx: literal $at_idx: literal $atom: ident
               $at_fun: ident ($at_init: expr) $at_ui: ident $fa_fun: ident
               $amin: literal $amax: literal})*
            $([$out_idx: literal $out: ident])*
            ,)+
    ) => {
        use std::rc::Rc;
        use std::cell::RefCell;

        #[derive(Debug, Clone, PartialEq)]
        pub enum ConstructorOp {
            SetDenorm(String, f32),
            SetDenormModAmt(String, f32, f32),
            SetSetting(String, i64),
            Input(String, ConstructorNode, String),
        }

        #[derive(Clone)]
        pub struct ConstructorNode {
            pub node_id: crate::dsp::NodeId,
            pub ops: Rc<RefCell<Vec<crate::build::ConstructorOp>>>,
        }

        impl PartialEq for ConstructorNode {
            fn eq(&self, other: &Self) -> bool {
                self.node_id == other.node_id
            }
        }

        impl std::fmt::Display for ConstructorNode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!(self.node_id))
            }
        }

        impl std::fmt::Debug for ConstructorNode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let node_id_str = format!("[{}]", self.node_id);
                for op in self.ops.borrow().iter() {
                    match op {
                        ConstructorOp::SetDenorm(port, v) => {
                            f.debug_tuple(&node_id_str).field(port).field(v).finish()?;
                        },
                        ConstructorOp::SetDenormModAmt(port, v, ma) => {
                            f.debug_tuple(&node_id_str).field(port).field(v).field(ma).finish()?;
                        },
                        ConstructorOp::SetSetting(port, v) => {
                            f.debug_tuple(&node_id_str).field(port).field(v).finish()?;
                        },
                        ConstructorOp::Input(port, constr, output) => {
                            f.debug_struct(&node_id_str).field("port", port).field("output", output).field("input", constr).finish()?;
                        },
                    }
                }
                writeln!(f, "")
            }
        }

        pub trait ConstructorNodeBuilder {
            fn id(&self) -> crate::dsp::NodeId;
            fn build(&self) -> crate::build::ConstructorNode;
        }

        pub trait ConstructorNodeOutputPort: ConstructorNodeBuilder {
            fn port(&self) -> (ConstructorNode, String);
        }

        pub mod output_port {
            $(
                #[doc = concat!("Output Port API Struct for [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                #[derive(Debug, Clone)]
                pub struct $variant {
                    pub node_id: crate::dsp::NodeId,
                    pub node: crate::build::ConstructorNode,
                    pub port: Option<String>
                }

                impl super::ConstructorNodeOutputPort for $variant {
                    fn port(&self) -> (super::ConstructorNode, String) {
                        (self.node.clone(), self.port.clone().unwrap_or_else(|| "".to_string()))
                    }
                }

                impl super::ConstructorNodeBuilder for $variant {
                    fn id(&self) -> crate::dsp::NodeId {
                        self.node_id
                    }

                    fn build(&self) -> crate::build::ConstructorNode {
                        self.node.clone()
                    }
                }

                impl $variant {
                    $(
                        #[doc = concat!("See also [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ") about details to this output")]
                        pub fn $out(mut self) -> Self {
                            self.port = Some(stringify!($out).to_string());
                            self
                        }
                    )*
                }
            )*
        }

        pub mod input_port {
            $(
                #[doc = concat!("Input Port API Struct for [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                pub struct $variant { pub node: super::$variant }
                impl $variant {
                    $(
                        #[doc = concat!("Assign to input port for [Node ", stringify!($variant), " Input ", stringify!($para),"](../index.html#nodeid", stringify!($str), "-input-", stringify!($para),")")]
                        pub fn $para(self, node: &dyn super::ConstructorNodeOutputPort) -> super::$variant {
                            let (node, portname) = node.port();

                            if !portname.is_empty() {
                                self.node.ops.borrow_mut().push(
                                    super::ConstructorOp::Input(
                                        stringify!($para).to_string(), node, portname));
                            }

                            self.node
                        }
                    )*
                }
            )*
        }

        pub mod set_param {
            $(
                #[doc = concat!("Parameter Setter API Struct for [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                pub struct $variant { pub node: super::$variant }
                impl $variant {
                    $(
                        #[doc = concat!("Set input parameter for [Node ", stringify!($variant), " Input ", stringify!($para),"](../index.html#nodeid", stringify!($str), "-input-", stringify!($para),")")]
                        pub fn $para(self, v: f32) -> super::$variant {
                            self.node.ops.borrow_mut().push(
                                super::ConstructorOp::SetDenorm(
                                    stringify!($para).to_string(), v));
                            self.node
                        }
                    )*
                    $(
                        #[doc = concat!("Set setting for [Node ", stringify!($variant), " Setting ", stringify!($atom),"](../index.html#nodeid", stringify!($str), "-setting-", stringify!($atom),")")]
                        pub fn $atom(self, v: i64) -> super::$variant {
                            self.node.ops.borrow_mut().push(
                                super::ConstructorOp::SetSetting(
                                    stringify!($atom).to_string(), v));
                            self.node
                        }
                    )*
                }
            )*
        }

        pub mod set_param_mod {
            $(
                #[doc = concat!("Parameter and Modulation Amount Setter API Struct for [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                pub struct $variant { pub node: super::$variant }
                impl $variant {
                    $(
                        #[doc = concat!("Set input parameter and modulation amount for [Node ", stringify!($variant), " Input ", stringify!($para),"](../index.html#nodeid", stringify!($str), "-input-", stringify!($para),")")]
                        pub fn $para(self, v: f32, ma: f32) -> super::$variant {
                            self.node.ops.borrow_mut().push(
                                super::ConstructorOp::SetDenormModAmt(
                                    stringify!($para).to_string(), v, ma));
                            self.node
                        }
                    )*
                }
            )*
        }

        $(
            #[derive(Debug, Clone)]
            #[doc = concat!("Build API Struct for [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
            pub struct $variant {
                node_id: crate::dsp::NodeId,
                ops: Rc<RefCell<Vec<ConstructorOp>>>,
            }

            impl $variant {
                /// Returns a structure with all settable parameters and settings.
                #[doc = concat!("See also [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                pub fn set(self) -> set_param::$variant {
                    set_param::$variant { node: self }
                }

                /// Returns a structure with all settable parameters with modulation amount.
                #[doc = concat!("See also [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                pub fn set_mod(self) -> set_param_mod::$variant {
                    set_param_mod::$variant { node: self }
                }

                /// Returns a structure with all input ports/parameters.
                #[doc = concat!("See also [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                pub fn input(self) -> input_port::$variant {
                    input_port::$variant { node: self }
                }

                /// Returns a structure with all output ports/paramters.
                #[doc = concat!("See also [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
                pub fn output(&self) -> output_port::$variant {
                    output_port::$variant {
                        node_id: self.id(),
                        node: self.build(),
                        port: None
                    }
                }
            }

            #[doc = concat!("Build API Function for [Node ", stringify!($variant), "](../index.html#nodeid", stringify!($str), ")")]
            pub fn $str(instance: u8) -> crate::build::$variant {
                $variant {
                    node_id: crate::dsp::NodeId::$variant(instance),
                    ops: Rc::new(RefCell::new(vec![])),
                }
            }

            impl ConstructorNodeBuilder for $variant {
                fn id(&self) -> crate::dsp::NodeId {
                    self.node_id
                }

                fn build(&self) -> crate::build::ConstructorNode {
                    ConstructorNode {
                        node_id: self.node_id,
                        ops: self.ops.clone()
                    }
                }
            }
        )+
    }
}

node_list! {make_node_constructor}
