// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::sync::{Arc, Mutex};
use crate::{NodeId, ScopeHandle, SharedFeedback, SharedFeedbackWriter, SharedFeedbackReader};
use std::collections::HashMap;

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
    /// Holding the scope buffers:
    scopes: HashMap<usize, Arc<ScopeHandle>>,
    /// Holds the shared feedback buffers:
    feedback: HashMap<usize, SharedFeedback>,
}

impl NodeGlobalData {
    pub fn new_ref() -> NodeGlobalRef {
        Arc::new(Mutex::new(Self {
            scopes: HashMap::new(),
            feedback: HashMap::new(),
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
}
