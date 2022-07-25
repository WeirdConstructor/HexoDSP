// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::SCOPE_SAMPLES;
use crate::util::AtomicFloat;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct ScopeHandle {
    bufs: [Vec<AtomicFloat>; 3],
    active: [AtomicBool; 3],
}

impl ScopeHandle {
    pub fn new_shared() -> Arc<Self> {
        let mut v1 = vec![];
        v1.resize_with(SCOPE_SAMPLES, || AtomicFloat::new(0.0));
        let mut v2 = vec![];
        v2.resize_with(SCOPE_SAMPLES, || AtomicFloat::new(0.0));
        let mut v3 = vec![];
        v3.resize_with(SCOPE_SAMPLES, || AtomicFloat::new(0.0));
        Arc::new(Self {
            bufs: [v1, v2, v3],
            active: [AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false)],
        })
    }

    pub fn write(&self, buf_idx: usize, idx: usize, v: f32) {
        self.bufs[buf_idx % 3][idx % SCOPE_SAMPLES].set(v);
    }

    pub fn read(&self, buf_idx: usize, idx: usize) -> f32 {
        self.bufs[buf_idx % 3][idx % SCOPE_SAMPLES].get()
    }

    pub fn set_active_from_mask(&self, mask: u64) {
        self.active[0].store(mask & 0x1 > 0x0, Ordering::Relaxed);
        self.active[1].store(mask & 0x2 > 0x0, Ordering::Relaxed);
        self.active[2].store(mask & 0x4 > 0x0, Ordering::Relaxed);
    }

    pub fn is_active(&self, idx: usize) -> bool {
        self.active[idx % 3].load(Ordering::Relaxed)
    }

    pub fn len(&self) -> usize {
        SCOPE_SAMPLES
    }
}
