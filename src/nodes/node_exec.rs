// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use super::{
    DropMsg, EventWindowing, GraphEvent, GraphMessage, HxMidiEvent, HxTimedEvent, NodeProg,
    MAX_INJ_MIDI_EVENTS, MAX_SMOOTHERS, UNUSED_MONITOR_IDX,
};
use crate::dsp::{Node, NodeContext, MAX_BLOCK_SIZE};
use crate::monitor::{MonitorBackend, MON_SIG_CNT};
use crate::util::Smoother;
use synfx_dsp::AtomicFloat;

use crate::log;
use std::io::Write;

use ringbuf::{Consumer, Producer};
use std::sync::Arc;

//use core::arch::x86_64::{
//    _MM_FLUSH_ZERO_ON,
//    _MM_FLUSH_ZERO_OFF,
//    _MM_SET_FLUSH_ZERO_MODE,
//    _MM_GET_FLUSH_ZERO_MODE
//};

pub const MAX_MIDI_NOTES_PER_BLOCK: usize = 512;
pub const MAX_MIDI_CC_PER_BLOCK: usize = 1024;

/// Holds the complete allocation of nodes and
/// the program. New Nodes or the program is
/// not newly allocated in the audio backend, but it is
/// copied from the input ring buffer.
/// If this turns out to be too slow, we might
/// have to push buffers of the program around.
///
pub struct NodeExecutor {
    /// Contains the stand-by smoothing operators for incoming parameter changes.
    pub(crate) smoothers: Vec<(usize, Smoother)>,

    /// Contains target parameter values after a smoother finished,
    /// these will refresh the input buffers:
    pub(crate) target_refresh: Vec<(usize, f32)>,

    /// Contains the to be executed nodes and output operations.
    /// Is copied from the input ringbuffer when a corresponding
    /// message arrives.
    pub(crate) prog: NodeProg,

    /// Holds the input vector indices which are to be monitored by the frontend.
    pub(crate) monitor_signal_cur_inp_indices: [usize; MON_SIG_CNT],

    /// The sample rate
    pub(crate) sample_rate: f32,

    /// Context that can be accessed by all (executed) nodes at runtime.
    pub(crate) exec_ctx: NodeExecContext,

    /// The connection with the [crate::nodes::NodeConfigurator].
    shared: SharedNodeExec,

    /// A small buffer for injected [HxMidiEvent]
    injected_midi: Vec<HxMidiEvent>,

    /// A flag to remember if we already initialized the logger on the audio thread.
    dsp_log_init: bool,
}

/// Contains anything that connects the [NodeExecutor] with the frontend part.
pub(crate) struct SharedNodeExec {
    /// Holds two context values interleaved.
    /// The first for each node is the LED value and the second is a
    /// phase value. The LED will be displayed in the hex matrix, while the
    /// phase might be used to display an envelope's play position.
    pub(crate) node_ctx_values: Vec<Arc<AtomicFloat>>,
    /// For receiving Node and NodeProg updates
    pub(crate) graph_update_con: Consumer<GraphMessage>,
    /// For receiving deleted/overwritten nodes from the backend thread.
    pub(crate) graph_drop_prod: Producer<DropMsg>,
    /// For sending events from the DSP graph to the frontend. Such as MIDI events for
    /// the MIDI learn functionality.
    pub(crate) graph_event_prod: Producer<GraphEvent>,
    /// For sending feedback to the frontend thread.
    pub(crate) monitor_backend: MonitorBackend,
    /// The current sample rate of the backend
    pub(crate) sample_rate: Arc<AtomicFloat>,
}

/// Contains audio driver context informations. Such as the number
/// of frames of the current buffer period and allows
/// writing output samples and reading input samples.
pub trait NodeAudioContext {
    fn nframes(&self) -> usize;
    fn output(&mut self, channel: usize, frame: usize, v: f32);
    fn input(&mut self, channel: usize, frame: usize) -> f32;
}

/// This trait needs to be implemented by the caller of the [NodeExecutor]
/// if it wants to provide the parameters for the "ExtA" to "ExtL" nodes.
pub trait ExternalParams: Send + Sync {
    fn a1(&self) -> f32;
    fn a2(&self) -> f32;
    fn a3(&self) -> f32;
    fn b1(&self) -> f32 {
        self.a1()
    }
    fn b2(&self) -> f32 {
        self.a2()
    }
    fn b3(&self) -> f32 {
        self.a3()
    }
    fn c1(&self) -> f32 {
        self.a1()
    }
    fn c2(&self) -> f32 {
        self.a2()
    }
    fn c3(&self) -> f32 {
        self.a3()
    }
    fn d1(&self) -> f32 {
        self.a1()
    }
    fn d2(&self) -> f32 {
        self.a2()
    }
    fn d3(&self) -> f32 {
        self.a3()
    }
    fn e1(&self) -> f32 {
        self.a1()
    }
    fn e2(&self) -> f32 {
        self.a2()
    }
    fn e3(&self) -> f32 {
        self.a3()
    }
    fn f1(&self) -> f32 {
        self.a1()
    }
    fn f2(&self) -> f32 {
        self.a2()
    }
    fn f3(&self) -> f32 {
        self.a3()
    }
}

/// Contains global state that all nodes can access.
/// This is used for instance to implement the MIDI functionality or the external parameters
/// for the HexoSynth plugin. Can also be used by other components outside HexoDSP on the audio
/// thread to send MIDI and provide external parameters.
pub struct NodeExecContext {
    /// List of current MIDI note events that were passed into HexoDSP in this buffer period.
    pub midi_notes: Vec<HxTimedEvent>,
    /// List of current MIDI CC events that were passed into HexoDSP in this buffer period.
    pub midi_ccs: Vec<HxTimedEvent>,
    /// Handle to the external parameters, external meaning parameters that come in via eg. the
    /// plugin API or are provided elsewhere on the audio thread.
    pub ext_param: Option<Arc<dyn ExternalParams>>,
}

impl NodeExecContext {
    fn new() -> Self {
        let midi_notes = Vec::with_capacity(MAX_MIDI_NOTES_PER_BLOCK);
        let midi_ccs = Vec::with_capacity(MAX_MIDI_CC_PER_BLOCK);
        Self { midi_notes, midi_ccs, ext_param: None }
    }

    fn set_sample_rate(&mut self, _srate: f32) {}

    fn clear(&mut self) {}
}

impl NodeExecutor {
    pub(crate) fn new(shared: SharedNodeExec) -> Self {
        let mut smoothers = Vec::new();
        smoothers.resize_with(MAX_SMOOTHERS, || (0, Smoother::new()));

        let target_refresh = Vec::with_capacity(MAX_SMOOTHERS);
        let injected_midi = Vec::with_capacity(MAX_INJ_MIDI_EVENTS);

        NodeExecutor {
            smoothers,
            target_refresh,
            sample_rate: 44100.0,
            prog: NodeProg::empty(),
            monitor_signal_cur_inp_indices: [UNUSED_MONITOR_IDX; MON_SIG_CNT],
            exec_ctx: NodeExecContext::new(),
            dsp_log_init: false,
            injected_midi,
            shared,
        }
    }

    pub fn no_logging(&mut self) {
        self.dsp_log_init = true;
    }

    #[inline]
    pub fn process_graph_updates(&mut self) {
        while let Some(upd) = self.shared.graph_update_con.pop() {
            match upd {
                GraphMessage::Clear { prog } => {
                    self.exec_ctx.clear();

                    self.monitor_signal_cur_inp_indices = [UNUSED_MONITOR_IDX; MON_SIG_CNT];

                    log(|w| {
                        let _ = write!(w, "[dbg] Cleared graph ({} nodes)", self.prog.prog.len());
                    });

                    let prev_prog = std::mem::replace(&mut self.prog, prog);
                    let _ = self.shared.graph_drop_prod.push(DropMsg::Prog { prog: prev_prog });
                }
                GraphMessage::NewProg { prog, copy_old_out } => {
                    let mut prev_prog = std::mem::replace(&mut self.prog, prog);

                    //                    unsafe {
                    //                        _MM_SET_FLUSH_ZERO_MODE(_MM_FLUSH_ZERO_ON);
                    //                    }

                    self.monitor_signal_cur_inp_indices = [UNUSED_MONITOR_IDX; MON_SIG_CNT];

                    // XXX: Copying from the old vector works, because we only
                    //      append nodes to the _end_ of the node instance vector.
                    //      If we do a garbage collection, we can't do this.
                    //
                    // XXX: Also, we need to initialize the input parameter
                    //      vector, because we don't know if they are updated from
                    //      the new program outputs anymore. So we need to
                    //      copy the old paramters to the inputs.
                    //
                    //      => This does not apply to atom data, because that
                    //         is always sent with the new program and "should"
                    //         be up to date, even if we have a slight possible race
                    //         condition between GraphMessage::NewProg
                    //         and GraphMessage::AtomUpdate.

                    // First overwrite by the current input parameters,
                    // to make sure _all_ inputs have a proper value
                    // (not just those that existed before).
                    //
                    // We preserve the modulation history in the next step.
                    // This is also to make sure that new input ports
                    // have a proper value too.
                    self.prog.initialize_input_buffers();

                    if copy_old_out {
                        // XXX: The following is commented out, because presisting
                        //      the output proc buffers does not make sense anymore.
                        //      Because we don't allow cycles, so there is no
                        //      way that a node can read from the previous
                        //      iteration anyways.
                        //
                        // // Swap the old out buffers into the new NodeProg
                        // // TODO: If we toss away most of the buffers anyways,
                        // //       we could optimize this step with more
                        // //       intelligence in the matrix compiler.
                        // for (old_pb, new_pb) in
                        //     prev_prog.out.iter_mut().zip(
                        //         self.prog.out.iter_mut())
                        // {
                        //     std::mem::swap(old_pb, new_pb);
                        // }

                        // Then overwrite the inputs by the more current previous
                        // input processing buffers, so we keep any modulation
                        // (smoothed) history of the block too.
                        self.prog.swap_previous_outputs(&mut prev_prog);
                    }

                    self.prog.assign_outputs();

                    let _ = self.shared.graph_drop_prod.push(DropMsg::Prog { prog: prev_prog });

                    log(|w| {
                        let _ = write!(
                            w,
                            "[dbg] Created new graph (node count={})",
                            self.prog.prog.len()
                        );
                    });
                }
                GraphMessage::AtomUpdate { at_idx, value } => {
                    let prog = &mut self.prog;
                    let garbage = std::mem::replace(&mut prog.atoms[at_idx], value);

                    let _ = self.shared.graph_drop_prod.push(DropMsg::Atom { atom: garbage });
                }
                GraphMessage::ParamUpdate { input_idx, value } => {
                    self.set_param(input_idx, value);
                }
                GraphMessage::ModamtUpdate { mod_idx, modamt } => {
                    self.set_modamt(mod_idx, modamt);
                }
                GraphMessage::SetMonitor { bufs } => {
                    self.monitor_signal_cur_inp_indices = bufs;
                }
                GraphMessage::InjectMidi { midi_ev } => {
                    if self.injected_midi.len() < MAX_INJ_MIDI_EVENTS {
                        self.injected_midi.push(midi_ev);
                    }
                }
            }
        }
    }

    pub fn set_external_params(&mut self, ext_param: Arc<dyn ExternalParams>) {
        self.exec_ctx.ext_param = Some(ext_param);
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.shared.sample_rate.set(sample_rate);
        self.exec_ctx.set_sample_rate(sample_rate);

        for op in self.prog.prog.iter() {
            op.node.set_sample_rate(sample_rate);
        }

        for sm in self.smoothers.iter_mut() {
            sm.1.set_sample_rate(sample_rate);
        }
    }

    #[inline]
    pub fn feed_midi_events_from<F: FnMut() -> Option<HxTimedEvent>>(&mut self, mut f: F) {
        self.exec_ctx.midi_notes.clear();
        self.exec_ctx.midi_ccs.clear();

        if self.injected_midi.len() > 0 {
            for ev in self.injected_midi.iter().rev() {
                let ev = HxTimedEvent::new_timed(0, *ev);
                if ev.is_cc() {
                    self.exec_ctx.midi_ccs.push(ev);
                } else {
                    self.exec_ctx.midi_notes.push(ev);
                }
                let _ = self.shared.graph_event_prod.push(GraphEvent::MIDI(ev.kind()));
            }

            self.injected_midi.clear();
        }

        while let Some(ev) = f() {
            if ev.is_cc() {
                self.exec_ctx.midi_ccs.push(ev);
            } else {
                self.exec_ctx.midi_notes.push(ev);
            }

            let _ = self.shared.graph_event_prod.push(GraphEvent::MIDI(ev.kind()));

            if self.exec_ctx.midi_ccs.len() == MAX_MIDI_CC_PER_BLOCK {
                break;
            }
            if self.exec_ctx.midi_notes.len() == MAX_MIDI_NOTES_PER_BLOCK {
                break;
            }
        }
    }

    #[inline]
    pub fn get_prog(&self) -> &NodeProg {
        &self.prog
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.prog.prog.iter().map(|op| op.node.clone()).collect::<Vec<Node>>()
    }

    #[inline]
    fn set_modamt(&mut self, mod_idx: usize, modamt: f32) {
        if mod_idx < self.prog.modops.len() {
            self.prog.modops[mod_idx].set_amt(modamt);
        }
    }

    #[inline]
    fn set_param(&mut self, input_idx: usize, value: f32) {
        let prog = &mut self.prog;

        if input_idx >= prog.params.len() {
            return;
        }

        // First check if we already have a running smoother for this param:
        for (sm_inp_idx, smoother) in self.smoothers.iter_mut().filter(|s| !s.1.is_done()) {
            if *sm_inp_idx == input_idx {
                smoother.set(prog.params[input_idx], value);
                //d// println!("RE-SET SMOOTHER {} {:6.3} (old = {:6.3})",
                //d//          input_idx, value, prog.params[input_idx]);
                return;
            }
        }

        // Find unused smoother and set it:
        if let Some(sm) = self.smoothers.iter_mut().find(|s| s.1.is_done()) {
            sm.0 = input_idx;
            sm.1.set(prog.params[input_idx], value);
            //d// println!("SET SMOOTHER {} {:6.3} (old = {:6.3})",
            //d//          input_idx, value, prog.params[input_idx]);
        }
    }

    #[inline]
    fn process_smoothers(&mut self, nframes: usize) {
        let prog = &mut self.prog;

        while let Some((idx, v)) = self.target_refresh.pop() {
            prog.inp[idx].fill(v);
        }

        for (idx, smoother) in self.smoothers.iter_mut().filter(|s| !s.1.is_done()) {
            let inp = &mut prog.inp[*idx];
            let mut last_v = 0.0;

            for frame in 0..nframes {
                let v = smoother.next();

                inp.write(frame, v);
                last_v = v;
            }

            prog.params[*idx] = last_v;
            self.target_refresh.push((*idx, last_v));
        }
    }

    #[inline]
    pub fn process<T: NodeAudioContext>(&mut self, ctx: &mut T) {
        // let tb = std::time::Instant::now();

        if !self.dsp_log_init && crate::log::init_thread_logger("dsp") {
            self.dsp_log_init = true;
            crate::log(|w| {
                let _ = write!(w, "DSP thread logger initialized");
            });
        }

        self.process_smoothers(ctx.nframes());

        let ctx_vals = &mut self.shared.node_ctx_values;
        let prog = &mut self.prog;
        let exec_ctx = &mut self.exec_ctx;

        let prog_out_fb = prog.out_feedback.input_buffer();

        let nframes = ctx.nframes();

        for op in prog.prog.iter() {
            let out = op.out_idxlen;
            let inp = op.in_idxlen;
            let at = op.at_idxlen;
            let md = op.mod_idxlen;
            let ctx_idx = op.idx as usize * 2;

            for modop in prog.modops[md.0..md.1].iter_mut() {
                modop.process(nframes);
            }

            op.node.process(
                ctx,
                exec_ctx,
                &NodeContext {
                    out_connected: op.out_connected,
                    in_connected: op.in_connected,
                    params: &prog.inp[inp.0..inp.1],
                },
                &prog.atoms[at.0..at.1],
                &prog.cur_inp[inp.0..inp.1],
                &mut prog.out[out.0..out.1],
                &ctx_vals[ctx_idx..ctx_idx + 2],
            );

            let last_frame_idx = nframes - 1;
            for (pb, out_buf_idx) in prog.out[out.0..out.1].iter().zip(out.0..out.1) {
                prog_out_fb[out_buf_idx] = pb.read(last_frame_idx);
            }
        }

        prog.out_feedback.publish();

        self.shared.monitor_backend.check_recycle();

        // let ta = std::time::Instant::now();

        for (i, idx) in self.monitor_signal_cur_inp_indices.iter().enumerate() {
            if *idx == UNUSED_MONITOR_IDX {
                continue;
            }

            if let Some(mut mon) = self.shared.monitor_backend.get_unused_mon_buf() {
                if i > 2 {
                    mon.feed(i, ctx.nframes(), &prog.out[*idx]);
                } else {
                    mon.feed(i, ctx.nframes(), &prog.cur_inp[*idx]);
                }

                self.shared.monitor_backend.send_mon_buf(mon);
            }
        }

        // let ta = std::time::Instant::now().duration_since(ta);
        // let tb = std::time::Instant::now().duration_since(tb);
        // println!("ta Elapsed: {:?}", ta);
        // println!("tb Elapsed: {:?}", tb);
    }

    /// This is a convenience function used for testing
    /// the DSP graph output in automated tests for this crate.
    ///
    /// The sample rate that is used to run the DSP code is 44100 Hz.
    ///
    /// Relying on the behvaiour of this function for production code
    /// is not it's intended usecase and changes might break your code.
    ///
    /// * `realtime`: If this is set, the function will sleep.
    ///
    /// You can use it's source as reference for your own audio
    /// DSP thread processing function.
    pub fn test_run_input(
        &mut self,
        input: &[f32],
        realtime: bool,
        events: &[HxTimedEvent],
    ) -> (Vec<f32>, Vec<f32>) {
        const SAMPLE_RATE: f32 = 44100.0;
        self.set_sample_rate(SAMPLE_RATE);
        self.process_graph_updates();

        let mut ev_win = EventWindowing::new();

        let mut nframes = input.len();

        let mut output_l = vec![0.0; nframes];
        let mut output_r = vec![0.0; nframes];

        for i in 0..nframes {
            output_l[i] = 0.0;
            output_r[i] = 0.0;
        }
        let mut ev_idx = 0;
        let mut offs = 0;
        while nframes > 0 {
            let cur_nframes = if nframes >= MAX_BLOCK_SIZE { MAX_BLOCK_SIZE } else { nframes };
            nframes -= cur_nframes;

            self.feed_midi_events_from(|| {
                if ev_win.feed_me() {
                    if ev_idx >= events.len() {
                        return None;
                    }

                    ev_win.feed(events[ev_idx]);
                    ev_idx += 1;
                }

                ev_win.next_event_in_range(offs, cur_nframes)
            });

            let mut context = crate::Context {
                nframes: cur_nframes,
                output: &mut [
                    &mut output_l[offs..(offs + cur_nframes)],
                    &mut output_r[offs..(offs + cur_nframes)],
                ],
                input: &[&input[offs..(offs + cur_nframes)], &input[offs..(offs + cur_nframes)]],
            };

            self.process(&mut context);

            if realtime {
                let micros = ((MAX_BLOCK_SIZE as u64) * 1000000) / (SAMPLE_RATE as u64);
                std::thread::sleep(std::time::Duration::from_micros(micros));
            }

            offs += cur_nframes;
        }

        (output_l, output_r)
    }

    /// This is a convenience function used for testing
    /// the DSP graph input and output in automated tests for this crate.
    ///
    /// The sample rate that is used to run the DSP code is 44100 Hz.
    ///
    /// Relying on the behvaiour of this function for production code
    /// is not it's intended usecase and changes might break your code.
    ///
    /// * `seconds`: The number of seconds to run the DSP thread for.
    /// * `realtime`: If this is set, the function will sleep.
    ///
    /// You can use it's source as reference for your own audio
    /// DSP thread processing function.
    pub fn test_run(
        &mut self,
        seconds: f32,
        realtime: bool,
        events: &[HxTimedEvent],
    ) -> (Vec<f32>, Vec<f32>) {
        const SAMPLE_RATE: f32 = 44100.0;
        self.set_sample_rate(SAMPLE_RATE);
        self.process_graph_updates();

        let mut ev_win = EventWindowing::new();

        let mut nframes = (seconds * SAMPLE_RATE) as usize;

        let input = vec![0.0; nframes];
        let mut output_l = vec![0.0; nframes];
        let mut output_r = vec![0.0; nframes];

        for i in 0..nframes {
            output_l[i] = 0.0;
            output_r[i] = 0.0;
        }
        let mut ev_idx = 0;
        let mut offs = 0;
        while nframes > 0 {
            let cur_nframes = if nframes >= MAX_BLOCK_SIZE { MAX_BLOCK_SIZE } else { nframes };
            nframes -= cur_nframes;

            self.feed_midi_events_from(|| {
                if ev_win.feed_me() {
                    if ev_idx >= events.len() {
                        return None;
                    }

                    ev_win.feed(events[ev_idx]);
                    ev_idx += 1;
                }

                ev_win.next_event_in_range(offs, cur_nframes)
            });

            let mut context = crate::Context {
                nframes: cur_nframes,
                output: &mut [
                    &mut output_l[offs..(offs + cur_nframes)],
                    &mut output_r[offs..(offs + cur_nframes)],
                ],
                input: &[&input[offs..(offs + cur_nframes)], &input[offs..(offs + cur_nframes)]],
            };

            self.process(&mut context);

            if realtime {
                let micros = ((MAX_BLOCK_SIZE as u64) * 1000000) / (SAMPLE_RATE as u64);
                std::thread::sleep(std::time::Duration::from_micros(micros));
            }

            offs += cur_nframes;
        }

        (output_l, output_r)
    }

    pub fn dummy_run(&mut self, seconds: f32) -> (Vec<f32>, Vec<f32>) {
        const SAMPLE_RATE: f32 = 44100.0;
        let mut nframes = (seconds * SAMPLE_RATE) as i64;

        let input = vec![0.0; MAX_BLOCK_SIZE];
        let mut output_l = vec![0.0; MAX_BLOCK_SIZE];
        let mut output_r = vec![0.0; MAX_BLOCK_SIZE];

        for i in 0..MAX_BLOCK_SIZE {
            output_l[i] = 0.0;
            output_r[i] = 0.0;
        }
        let offs = 0;
        let cur_nframes = MAX_BLOCK_SIZE;
        while nframes > 0 {
            nframes -= 128;

            let mut context = crate::Context {
                nframes: cur_nframes,
                output: &mut [
                    &mut output_l[offs..(offs + cur_nframes)],
                    &mut output_r[offs..(offs + cur_nframes)],
                ],
                input: &[&input[offs..(offs + cur_nframes)], &input[offs..(offs + cur_nframes)]],
            };

            self.process(&mut context);
        }

        (output_l, output_r)
    }
}
