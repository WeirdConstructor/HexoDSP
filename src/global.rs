// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::sync::{Arc, Mutex};
use crate::{NodeId, ScopeHandle};
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
}

impl NodeGlobalData {
    pub fn new_ref() -> NodeGlobalRef {
        Arc::new(Mutex::new(Self {
            scopes: HashMap::new(),
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
}
