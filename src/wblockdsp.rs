// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use wblockdsp::*;

use ringbuf::{Consumer, Producer, RingBuffer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const MAX_RINGBUF_SIZE: usize = 128;
const MAX_CONTEXTS: usize = 32;

enum CodeUpdateMsg {
    UpdateFun(Box<DSPFunction>),
}

enum CodeReturnMsg {
    DestroyFun(Box<DSPFunction>),
}

pub struct CodeEngine {
    dsp_ctx: Rc<RefCell<DSPNodeContext>>,
    lib: Rc<RefCell<DSPNodeTypeLibrary>>,
    update_prod: Producer<CodeUpdateMsg>,
    return_cons: Consumer<CodeReturnMsg>,
}

impl Clone for CodeEngine {
    fn clone(&self) -> Self {
        CodeEngine::new()
    }
}

impl CodeEngine {
    pub fn new() -> Self {
        let rb = RingBuffer::new(MAX_RINGBUF_SIZE);
        let (update_prod, update_cons) = rb.split();
        let rb = RingBuffer::new(MAX_RINGBUF_SIZE);
        let (return_prod, return_cons) = rb.split();

        let lib = get_default_library();

        Self {
            lib,
            dsp_ctx: DSPNodeContext::new_ref(),
            update_prod,
            return_cons,
        }
    }

    pub fn upload(
        &mut self,
        code_instance: usize,
        ast: Box<ASTNode>,
    ) -> Result<(), JITCompileError> {

        let jit = JIT::new(self.lib.clone(), self.dsp_ctx.clone());
        let fun = jit.compile(ASTFun::new(ast))?;
        self.update_prod.push(CodeUpdateMsg::UpdateFun(fun));

        Ok(())
    }

    pub fn cleanup(&self, fun: Box<DSPFunction>) {
        self.dsp_ctx.borrow_mut().cleanup_dsp_fun_after_user(fun);
    }

    pub fn query_returns(&mut self) {
        while let Some(msg) = self.return_cons.pop() {
            match msg {
                CodeReturnMsg::DestroyFun(fun) => {
                    self.cleanup(fun);
                }
            }
        }
    }

    pub fn get_backend(&mut self) -> CodeEngineBackend {
        let rb = RingBuffer::new(MAX_RINGBUF_SIZE);
        let (update_prod, update_cons) = rb.split();
        let rb = RingBuffer::new(MAX_RINGBUF_SIZE);
        let (return_prod, return_cons) = rb.split();

        self.update_prod = update_prod;
        self.return_cons = return_cons;

        let function = get_nop_function(self.lib.clone(), self.dsp_ctx.clone());
        CodeEngineBackend::new(function, update_cons, return_prod)
    }
}

impl Drop for CodeEngine {
    fn drop(&mut self) {
        self.dsp_ctx.borrow_mut().free();
    }
}


pub struct CodeEngineBackend {
    sample_rate: f32,
    function: Box<DSPFunction>,
    update_cons: Consumer<CodeUpdateMsg>,
    return_prod: Producer<CodeReturnMsg>,
}

impl CodeEngineBackend {
    fn new(function: Box<DSPFunction>, update_cons: Consumer<CodeUpdateMsg>, return_prod: Producer<CodeReturnMsg>) -> Self {
        Self { sample_rate: 0.0, function, update_cons, return_prod }
    }

    #[inline]
    pub fn process(
        &mut self,
        in1: f32,
        in2: f32,
        a: f32,
        b: f32,
        d: f32,
        g: f32,
    ) -> (f32, f32, f32) {
        let mut s1 = 0.0_f64;
        let mut s2 = 0.0_f64;
        let res = self
            .function
            .exec(in1 as f64, in2 as f64, a as f64, b as f64, d as f64, g as f64, &mut s1, &mut s2);
        (s1 as f32, s2 as f32, res as f32)
    }

    pub fn swap_fun(&mut self, srate: f32, mut fun: Box<DSPFunction>) -> Box<DSPFunction> {
        std::mem::swap(&mut self.function, &mut fun);
        self.function.init(srate as f64, Some(&fun));
        fun
    }

    pub fn set_sample_rate(&mut self, srate: f32) {
        self.sample_rate = srate;
        self.function.set_sample_rate(srate as f64);
    }

    pub fn clear(&mut self) {
        self.function.reset();
    }

    pub fn process_updates(&mut self) {
        while let Some(msg) = self.update_cons.pop() {
            match msg {
                CodeUpdateMsg::UpdateFun(mut fun) => {
                    std::mem::swap(&mut self.function, &mut fun);
                    self.function.init(self.sample_rate as f64, Some(&fun));
                    self.return_prod.push(CodeReturnMsg::DestroyFun(fun));
                }
            }
        }
    }
}
