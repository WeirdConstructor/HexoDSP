// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::tracker::{PatternData, Tracker, TrackerBackend};
use crate::wblockdsp::*;
use crate::{ScopeHandle, SharedFeedback, SharedFeedbackReader, SharedFeedbackWriter};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[cfg(feature = "synfx-dsp-jit")]
use synfx_dsp_jit::engine::{CodeEngine, CodeEngineBackend};

/// Reference to a [crate::NodeGlobalData] instance.
pub type NodeGlobalRef = Arc<Mutex<NodeGlobalData>>;

/// This structure holds any global state that may be shared among
/// [crate::dsp::DspNode] instances.
///
/// This structure is provided to you by the [crate::Matrix] and
/// resides in [crate::NodeConfigurator].
///
/// These may be things like feedback buffers that are shared among `FbWr` and `FbRd`
/// nodes, or the [crate::dsp::tracker::Tracker] that drives the `TSeq` sequencers.
/// Also the [crate::ScopeHandle] instances used to connect the `Scope` nodes to the
/// frontend are exchanged through this structure.
pub struct NodeGlobalData {
    /// Holding the scope buffers
    scopes: HashMap<usize, Arc<ScopeHandle>>,
    /// Holds the shared feedback buffers
    feedback: HashMap<usize, SharedFeedback>,
    /// Holds the handles to the tracker sequencers
    trackers: HashMap<usize, Tracker>,
    /// Holding the synfx-dsp-jit code engine backends
    /// (which are used for the WBlockDSP block functions, but can also be directly used):
    #[cfg(feature = "synfx-dsp-jit")]
    pub(crate) code_engines: HashMap<usize, CodeEngine>,
    /// Holds the block functions that are JIT compiled to DSP code
    /// for the `Code` nodes. The code is then sent via the [CodeEngine]
    /// in [NodeConfigurator::check_block_function].
    #[cfg(feature = "synfx-dsp-jit")]
    pub(crate) block_functions: HashMap<usize, (u64, Arc<Mutex<BlockFun>>)>,
}

impl NodeGlobalData {
    pub fn new_ref() -> NodeGlobalRef {
        Arc::new(Mutex::new(Self {
            scopes: HashMap::new(),
            feedback: HashMap::new(),
            trackers: HashMap::new(),
            #[cfg(feature = "synfx-dsp-jit")]
            code_engines: HashMap::new(),
            #[cfg(feature = "synfx-dsp-jit")]
            block_functions: HashMap::new(),
        }))
    }

    pub fn get_scope_handle(&mut self, scope: usize) -> Arc<ScopeHandle> {
        if let Some(handle) = self.scopes.get(&scope) {
            return handle.clone();
        }

        // TODO: Make Scopes sample rate agnostic?
        let new_handle = ScopeHandle::new_shared();
        self.scopes.insert(scope, new_handle.clone());
        new_handle
    }

    pub fn get_shared_feedback(&mut self, instance: usize) -> &mut SharedFeedback {
        if !self.feedback.contains_key(&instance) {
            // FIXME: Sample rate needs to be determined properly!
            let new_shared_feedback = SharedFeedback::new(44100.0);
            self.feedback.insert(instance, new_shared_feedback);
        }

        self.feedback.get_mut(&instance).unwrap()
    }

    pub fn get_feedback_reader(&mut self, instance: usize) -> Box<SharedFeedbackReader> {
        return Box::new(SharedFeedbackReader::new(self.get_shared_feedback(instance)));
    }

    pub fn get_feedback_writer(&mut self, instance: usize) -> Box<SharedFeedbackWriter> {
        return Box::new(SharedFeedbackWriter::new(self.get_shared_feedback(instance)));
    }

    /// Returns the [PatternData] handle for the tracker `index`.
    /// Implicitly allocates the [Tracker] instance.
    pub fn get_pattern_data(&mut self, index: usize) -> Arc<Mutex<PatternData>> {
        if !self.trackers.contains_key(&index) {
            self.trackers.insert(index, Tracker::new());
        }

        self.trackers.get(&index).unwrap().data()
    }

    /// Returns true, if the tracker and patterndata for `index` has already been allocated.
    pub fn has_tracker(&self, index: usize) -> bool {
        self.trackers.contains_key(&index)
    }

    /// Returns the [TrackerBackend] handle for the tracker `index`.
    /// Implicitly allocates the [Tracker] instance.
    /// The returned [TrackerBackend] will invalidate the other backend handles
    /// for the corresponding tracker `index`.
    pub fn get_tracker_backend(&mut self, index: usize) -> TrackerBackend {
        if !self.trackers.contains_key(&index) {
            self.trackers.insert(index, Tracker::new());
        }

        self.trackers.get_mut(&index).unwrap().get_backend()
    }

    /// Checks if there are any updates to send for the pattern data that belongs to the
    /// tracker `index`. Call this repeatedly, eg. once per frame in a GUI, in case the user
    /// modified the pattern data. It will make sure that the modifications are sent to the
    /// audio thread.
    pub fn check_pattern_data(&mut self, index: usize) {
        if let Some(tracker) = self.trackers.get_mut(&index) {
            tracker.send_one_update();
        }
    }

    /// Gives you direct access to the [CodeEngine] for `index` instance of
    /// the corresponding `Code` DSP node. You can also get the corresponding
    /// [BlockFun] via [NodeGlobalData::get_block_fun]. But in case of
    /// eg. [SynthConstructor] you can use the [CodeEngine] directly too!
    ///
    /// This function only returns `None` if there is no `synfx-dsp-jit` feature enabled!
    ///
    /// Make sure to call [NodeGlobalData::check_code] regularily!
    #[cfg(feature = "synfx-dsp-jit")]
    pub fn get_code_engine(&mut self, index: usize) -> &mut CodeEngine {
        #[cfg(feature = "synfx-dsp-jit")]
        {
            if !self.code_engines.contains_key(&index) {
                self.code_engines.insert(index, CodeEngine::new_stdlib());
            }

            self.code_engines.get_mut(&index).unwrap()
        }
    }

    /// Returns true if there was the [CodeEngine] `index` already used.
    pub fn has_code_engine(&mut self, index: usize) -> bool {
        #[cfg(feature = "synfx-dsp-jit")]
        {
            self.code_engines.contains_key(&index)
        }

        #[cfg(not(feature = "synfx-dsp-jit"))]
        {
            false
        }
    }

    /// Returns the [synfx-dsp::CodeEngineBackend] for the corresponding code engine at `index`.
    ///
    /// Any call of this function clears the connection of the corresponding [CodeEngine]
    /// to the previously returned [CodeEngineBackend].
    #[cfg(feature = "synfx-dsp-jit")]
    pub fn get_code_engine_backend(&mut self, index: usize) -> CodeEngineBackend {
        self.get_code_engine(index).get_backend()
    }

    /// Retrieve a handle to the block function `id`. In case you modify the block function,
    /// make sure to call [NodeGlobalData::check_code].
    ///
    /// Only returns `None` if the feature `synfx-dsp-jit` is disabled!
    pub fn get_block_function(&mut self, id: usize) -> Option<Arc<Mutex<BlockFun>>> {
        #[cfg(feature = "synfx-dsp-jit")]
        {
            let lang = setup_hxdsp_block_language(self.get_code_engine(id).get_lib());

            if !self.block_functions.contains_key(&id) {
                self.block_functions
                    .insert(id, (0, Arc::new(Mutex::new(BlockFun::new(lang.clone())))));
            }

            self.block_functions.get(&id).map(|pair| pair.1.clone())
        }

        #[cfg(not(feature = "synfx-dsp-jit"))]
        {
            None
        }
    }

    /// Checks the [CodeEngine] and block function for the id `id`. If the block function did change,
    /// updates are then sent to the audio thread.
    /// See also [NodeGlobalData::get_block_function].
    pub fn check_code(&mut self, id: usize) -> Result<(), BlkJITCompileError> {
        #[cfg(feature = "synfx-dsp-jit")]
        if self.has_code_engine(id) {
            self.get_code_engine(id).query_returns();

            if let Some((generation, block_fun)) = self.block_functions.get_mut(&id) {
                if let Ok(block_fun) = block_fun.lock() {
                    if *generation != block_fun.generation() {
                        *generation = block_fun.generation();
                        let mut compiler = Block2JITCompiler::new(block_fun.block_language());
                        let ast = compiler.compile(&block_fun)?;

                        if let Some(cod) = self.code_engines.get_mut(&id) {
                            match cod.upload(ast) {
                                Err(e) => return Err(BlkJITCompileError::JITCompileError(e)),
                                Ok(()) => (),
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
