// Copyright (c) 2022 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::SCOPE_SAMPLES;
use crate::util::{AtomicFloatPair, AtomicFloat};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct ScopeHandle {
    bufs: [Vec<AtomicFloatPair>; 3],
    active: [AtomicBool; 3],
    offs_gain: [AtomicFloatPair; 3],
    threshold: (AtomicBool, AtomicFloat),
}

impl ScopeHandle {
    pub fn new_shared() -> Arc<Self> {
        let mut v1 = vec![];
        v1.resize_with(SCOPE_SAMPLES, || AtomicFloatPair::default());
        let mut v2 = vec![];
        v2.resize_with(SCOPE_SAMPLES, || AtomicFloatPair::default());
        let mut v3 = vec![];
        v3.resize_with(SCOPE_SAMPLES, || AtomicFloatPair::default());
        Arc::new(Self {
            bufs: [v1, v2, v3],
            active: [AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false)],
            offs_gain: [
                AtomicFloatPair::default(),
                AtomicFloatPair::default(),
                AtomicFloatPair::default(),
            ],
            threshold: (AtomicBool::new(false), AtomicFloat::default()),
        })
    }

    pub fn write_oversampled(&self, buf_idx: usize, idx: usize, copies: usize, v: f32) {
        let end = (idx + copies).min(SCOPE_SAMPLES);
        for i in idx..end {
            self.bufs[buf_idx % 3][i % SCOPE_SAMPLES].set((v, v));
        }
    }

    pub fn set_offs_gain(&self, buf_idx: usize, offs: f32, gain: f32) {
        self.offs_gain[buf_idx % 3].set((offs, gain));
    }

    pub fn get_offs_gain(&self, buf_idx: usize) -> (f32, f32) {
        self.offs_gain[buf_idx % 3].get()
    }

    pub fn set_threshold(&self, thresh: Option<f32>) {
        if let Some(t) = thresh {
            self.threshold.1.set(t);
            self.threshold.0.store(true, Ordering::Relaxed);
        } else {
            self.threshold.0.store(false, Ordering::Relaxed);
        }
    }

    pub fn get_threshold(&self) -> Option<f32> {
        if self.threshold.0.load(Ordering::Relaxed) {
            Some(self.threshold.1.get())
        } else {
            None
        }
    }

    pub fn write(&self, buf_idx: usize, idx: usize, v: (f32, f32)) {
        self.bufs[buf_idx % 3][idx % SCOPE_SAMPLES].set(v);
    }

    pub fn read(&self, buf_idx: usize, idx: usize) -> (f32, f32) {
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
