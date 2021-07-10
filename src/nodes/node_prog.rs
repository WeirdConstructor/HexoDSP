// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::dsp::{ProcBuf, SAtom};
use triple_buffer::{Input, Output, TripleBuffer};

#[derive(Debug, Clone)]
pub struct ModOp {
    amount:     f32,
    modbuf:     ProcBuf,
    outbuf:     ProcBuf,
    inbuf:      ProcBuf,
}

impl Drop for ModOp {
    fn drop(&mut self) {
        self.modbuf.free();
    }
}

impl ModOp {
    pub fn new() -> Self {
        Self {
            amount: 0.0,
            modbuf: ProcBuf::new(),
            outbuf: ProcBuf::null(),
            inbuf:  ProcBuf::null(),
        }
    }

    pub fn set_amt(&mut self, amt: f32) {
        self.amount = amt;
    }

    pub fn lock(&mut self, inbuf: ProcBuf, outbuf: ProcBuf) {
        self.inbuf  = inbuf;
        self.outbuf = outbuf;
    }

    pub fn unlock(&mut self) {
        self.outbuf = ProcBuf::null();
        self.inbuf  = ProcBuf::null();
    }

    #[inline]
    pub fn process(&mut self, nframes: usize) {
        let modbuf = &mut self.modbuf;
        let inbuf  = &mut self.inbuf;
        let outbuf = &mut self.outbuf;

        for frame in 0..nframes {
            modbuf.write(frame,
                modbuf.read(frame)
                * outbuf.read(frame)
                + inbuf.read(frame));
        }
    }
}

/// Step in a `NodeProg` that stores the to be
/// executed node and output operations.
#[derive(Debug, Clone)]
pub struct NodeOp {
    /// Stores the index of the node
    pub idx:  u8,
    /// Output index and length of the node:
    pub out_idxlen: (usize, usize),
    /// Input index and length of the node:
    pub in_idxlen: (usize, usize),
    /// Atom data index and length of the node:
    pub at_idxlen: (usize, usize),
    /// ModOp index and length of the node:
    pub mod_idxlen: (usize, usize),
    /// Input indices,
    /// (<out vec index>, <own node input index>,
    ///  (<mod index into NodeProg::modops>, <mod amt>))
    pub inputs: Vec<(usize, usize, Option<usize>)>,
}

impl std::fmt::Display for NodeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Op(i={} out=({}-{}) in=({}-{}) at=({}-{}) mod=({}-{})",
               self.idx,
               self.out_idxlen.0,
               self.out_idxlen.1,
               self.in_idxlen.0,
               self.in_idxlen.1,
               self.at_idxlen.0,
               self.at_idxlen.1,
               self.mod_idxlen.0,
               self.mod_idxlen.1)?;

        for i in self.inputs.iter() {
            write!(f, " cpy=(o{} => i{})", i.0, i.1)?;
        }

        for i in self.inputs.iter() {
            if let Some(idx) = i.2 {
                write!(f, " mod={}", idx)?;
            }
        }

        write!(f, ")")
    }
}

/// A node graph execution program. It comes with buffers
/// for the inputs, outputs and node parameters (knob values).
#[derive(Debug)]
pub struct NodeProg {
    /// The input vector stores the smoothed values of the params.
    /// It is not used directly, but will be merged into the `cur_inp`
    /// field together with the assigned outputs.
    pub inp:    Vec<ProcBuf>,

    /// The temporary input vector that is initialized from `inp`
    /// and is then merged with the associated outputs.
    pub cur_inp: Vec<ProcBuf>,

    /// The output vector, holding all the node outputs.
    pub out:    Vec<ProcBuf>,

    /// The param vector, holding all parameter inputs of the
    /// nodes, such as knob settings.
    pub params: Vec<f32>,

    /// The atom vector, holding all non automatable parameter inputs
    /// of the nodes, such as samples or integer settings.
    pub atoms:  Vec<SAtom>,

    /// The node operations that are executed in the order they appear in this
    /// vector.
    pub prog:   Vec<NodeOp>,

    /// The modulators for the input parameters.
    pub modops: Vec<ModOp>,

    /// A marker, that checks if we can still swap buffers with
    /// with other NodeProg instances. This is usally set if the ProcBuf pointers
    /// have been copied into `cur_inp`. You can call `unlock_buffers` to
    /// clear `locked_buffers`:
    pub locked_buffers: bool,

    /// Holds the input end of a triple buffer that is used
    /// to publish the most recent output values to the frontend.
    pub out_feedback: Input<Vec<f32>>,

    /// Temporary hold for the producer for the `out_feedback`:
    pub out_fb_cons: Option<Output<Vec<f32>>>,
}

impl Drop for NodeProg {
    fn drop(&mut self) {
        for buf in self.inp.iter_mut() {
            buf.free();
        }

        for buf in self.out.iter_mut() {
            buf.free();
        }
    }
}


impl NodeProg {
    pub fn empty() -> Self {
        let out_fb = vec![];
        let tb = TripleBuffer::new(out_fb);
        let (input_fb, output_fb) = tb.split();
        Self {
            out:     vec![],
            inp:     vec![],
            cur_inp: vec![],
            params:  vec![],
            atoms:   vec![],
            prog:    vec![],
            modops:  vec![],
            out_feedback:   input_fb,
            out_fb_cons:    Some(output_fb),
            locked_buffers: false,
        }
    }

    pub fn new(out_len: usize, inp_len: usize, at_len: usize, mod_len: usize) -> Self {
        let mut out = vec![];
        out.resize_with(out_len, ProcBuf::new);

        let out_fb = vec![0.0; out_len];
        let tb = TripleBuffer::new(out_fb);
        let (input_fb, output_fb) = tb.split();

        let mut inp = vec![];
        inp.resize_with(inp_len, ProcBuf::new);
        let mut cur_inp = vec![];
        cur_inp.resize_with(inp_len, ProcBuf::null);

        let mut params = vec![];
        params.resize(inp_len, 0.0);
        let mut atoms = vec![];
        atoms.resize(at_len, SAtom::setting(0));
        let mut modops = vec![];
        modops.resize_with(mod_len, ModOp::new);

        Self {
            out,
            inp,
            cur_inp,
            params,
            atoms,
            modops,
            prog:           vec![],
            out_feedback:   input_fb,
            out_fb_cons:    Some(output_fb),
            locked_buffers: false,
        }
    }

    pub fn take_feedback_consumer(&mut self) -> Option<Output<Vec<f32>>> {
        self.out_fb_cons.take()
    }

    pub fn params_mut(&mut self) -> &mut [f32] {
        &mut self.params
    }

    pub fn atoms_mut(&mut self) -> &mut [SAtom] {
        &mut self.atoms
    }

    pub fn modops_mut(&mut self) -> &mut [ModOp] {
        &mut self.modops
    }

    pub fn append_op(&mut self, node_op: NodeOp) {
        for n_op in self.prog.iter_mut() {
            if n_op.idx == node_op.idx {
                return;
            }
        }

        self.prog.push(node_op);
    }

    pub fn append_edge(
        &mut self,
        node_op: NodeOp,
        inp_index: usize,
        out_index: usize,
        mod_index: Option<usize>)
    {
        for n_op in self.prog.iter_mut() {
            if n_op.idx == node_op.idx {
                n_op.inputs.push((out_index, inp_index, mod_index));
                return;
            }
        }
    }

    /// This is called right after the [crate::nodes::NodeExecutor]
    /// received this NodeProg from the [crate::nodes::NodeConfigurator].
    /// It initializes internal buffers with parameter data.
    pub fn initialize_input_buffers(&mut self) {
        for param_idx in 0..self.params.len() {
            let param_val = self.params[param_idx];
            self.inp[param_idx].fill(param_val);
        }
    }

    pub fn swap_previous_outputs(&mut self, prev_prog: &mut NodeProg) {
        if self.locked_buffers {
            self.unlock_buffers();
        }

        if prev_prog.locked_buffers {
            prev_prog.unlock_buffers();
        }

        // XXX: Swapping is now safe, because the `cur_inp` field
        //      no longer references to the buffers in `inp` or `out`.
        for (old_inp_pb, new_inp_pb) in
            prev_prog.inp.iter_mut().zip(
                self.inp.iter_mut())
        {
            std::mem::swap(old_inp_pb, new_inp_pb);
        }
    }

    pub fn unlock_buffers(&mut self) {
        for buf in self.cur_inp.iter_mut() {
            *buf = ProcBuf::null();
        }
        for modop in self.modops.iter_mut() {
            modop.unlock();
        }
        self.locked_buffers = false;
    }

    pub fn assign_outputs(&mut self) {
        for op in self.prog.iter() {

            // First step is copying the ProcBufs to the `cur_inp` current
            // input buffer vector. It holds the data for smoothed paramter
            // inputs or just constant values since the last smoothing.
            //
            // Next we overwrite the input ProcBufs which have an
            // assigned output buffer.
            //
            // ProcBuf has a raw pointer inside, and this copying
            // is therefor very fast.
            //
            // XXX: This requires, that the graph is not cyclic,
            // because otherwise we would write output buffers which
            // are already accessed in the current iteration.
            // This might lead to unexpected effects inside the process()
            // call of the nodes.
            let input_bufs = &mut self.cur_inp;
            let out_bufs   = &mut self.out;

            let inp = op.in_idxlen;

            // First step (refresh inputs):
            input_bufs[inp.0..inp.1]
                .copy_from_slice(&self.inp[inp.0..inp.1]);

            // Second step (assign outputs):
            for io in op.inputs.iter() {
                input_bufs[io.1] = out_bufs[io.0];

                if let Some(idx) = io.2 {
                    self.modops[idx].lock(self.inp[io.1], out_bufs[io.0]);
                }
            }
        }

        self.locked_buffers = true;
    }
}

