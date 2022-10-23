use crate::dsp::NodeId;
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
            SetSetting(String, i32),
            Input(String, ConstructorNode, String),
            InputDefault(String, ConstructorNode),
            InputModAmt(String, f32, f32, ConstructorNode, String),
        }

        #[derive(Clone)]
        pub struct ConstructorNode {
            node_id: crate::dsp::NodeId,
            ops: Rc<RefCell<Vec<crate::dsp::build::ConstructorOp>>>,
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
                        ConstructorOp::SetSetting(port, v) => {
                            f.debug_tuple(&node_id_str).field(port).field(v).finish()?;
                        },
                        ConstructorOp::Input(port, constr, output) => {
                            f.debug_struct(&node_id_str).field("port", port).field("output", output).field("input", constr).finish()?;
                        },
                        ConstructorOp::InputDefault(port, constr) => {
                            f.debug_struct(&node_id_str).field("port", port).field("input", constr).finish()?;
                        },
                        _ => {
                            writeln!(f, "   - {:?}", op)?;
                        }
                    }
                }
                writeln!(f, "")
            }
        }

        pub trait ConstructorNodeBuilder {
            fn id(&self) -> crate::dsp::NodeId;
            fn build(&self) -> crate::dsp::build::ConstructorNode;
        }

        pub trait ConstructorNodeOutputPort: ConstructorNodeBuilder {
            fn port(&self) -> (ConstructorNode, String);
        }

        pub mod OutputPort {
            $(
                #[derive(Debug, Clone)]
                pub struct $variant {
                    pub node_id: crate::dsp::NodeId,
                    pub node: crate::dsp::build::ConstructorNode,
                    pub port: Option<String>
                }

                impl super::ConstructorNodeOutputPort for $variant {
                    fn port(&self) -> (super::ConstructorNode, String) {
                        use super::ConstructorNodeBuilder;
                        (self.node.clone(), self.port.clone().unwrap_or_else(|| "".to_string()))
                    }
                }

                impl super::ConstructorNodeBuilder for $variant {
                    fn id(&self) -> crate::dsp::NodeId {
                        self.node_id
                    }

                    fn build(&self) -> crate::dsp::build::ConstructorNode {
                        self.node.clone()
                    }
                }

                impl $variant {
                    $(
                        pub fn $out(mut self) -> Self {
                            self.port = Some(stringify!($out).to_string());
                            self
                        }
                    )*
                }
            )*
        }

        pub mod InputPort {
            $(
                pub struct $variant { pub node: super::$variant }
                impl $variant {
                    $(
                        pub fn $para(mut self, node: &dyn super::ConstructorNodeOutputPort) -> super::$variant {
                            let (node, portname) = node.port();

                            if portname.is_empty() {
                                self.node.ops.borrow_mut().push(
                                    super::ConstructorOp::InputDefault(
                                        stringify!($para).to_string(), node));
                            } else {
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

        pub mod SetPara {
            $(
                pub struct $variant { pub node: super::$variant }
                impl $variant {
                    $(
                        pub fn $para(mut self, v: f32) -> super::$variant {
                            self.node.ops.borrow_mut().push(
                                super::ConstructorOp::SetDenorm(
                                    stringify!($para).to_string(), v));
                            self.node
                        }
                    )*
                    $(
                        pub fn $atom(mut self, v: i32) -> super::$variant {
                            self.node.ops.borrow_mut().push(
                                super::ConstructorOp::SetSetting(
                                    stringify!($atom).to_string(), v));
                            self.node
                        }
                    )*
                }
            )*
        }

        $(
            #[derive(Debug, Clone)]
            pub struct $variant {
                node_id: crate::dsp::NodeId,
                ops: Rc<RefCell<Vec<ConstructorOp>>>,
            }

            impl $variant {
                pub fn set(self) -> SetPara::$variant {
                    SetPara::$variant { node: self }
                }

                pub fn input(self) -> InputPort::$variant {
                    InputPort::$variant { node: self }
                }

                pub fn output(&self) -> OutputPort::$variant {
                    OutputPort::$variant {
                        node_id: self.id(),
                        node: self.build(),
                        port: None
                    }
                }
            }

            pub fn $str(instance: u8) -> crate::dsp::build::$variant {
                $variant {
                    node_id: crate::dsp::NodeId::$variant(instance),
                    ops: Rc::new(RefCell::new(vec![])),
                }
            }

            impl ConstructorNodeBuilder for $variant {
                fn id(&self) -> crate::dsp::NodeId {
                    self.node_id
                }

                fn build(&self) -> crate::dsp::build::ConstructorNode {
                    ConstructorNode {
                        node_id: self.node_id,
                        ops: self.ops.clone()
                    }
                }
            }
        )+
    }
}

#[allow(allow_unused)]
node_list! {make_node_constructor}
