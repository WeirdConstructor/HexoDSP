// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

const SMOOTHING_TIME_MS: f32 = 10.0;

pub struct Smoother {
    slope_samples: usize,
    value: f32,
    inc: f32,
    target: f32,
    count: usize,
    done: bool,
}

impl Smoother {
    pub fn new() -> Self {
        Self { slope_samples: 0, value: 0.0, inc: 0.0, count: 0, target: 0.0, done: true }
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.slope_samples = ((sr * SMOOTHING_TIME_MS) / 1000.0).ceil() as usize;
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        self.done
    }

    #[inline]
    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.done = true;
    }

    #[inline]
    pub fn set(&mut self, current: f32, target: f32) {
        self.value = current;
        self.count = self.slope_samples;
        self.inc = (target - current) / (self.count as f32);
        self.target = target;
        self.done = false;
    }

    #[inline]
    pub fn next(&mut self) -> f32 {
        //d// println!("NEXT: count={}, value={:6.3} inc={:6.4}",
        //d//          self.count,
        //d//          self.value,
        //d//          self.inc);
        if self.count == 0 {
            self.done = true;

            self.target
        } else {
            self.value += self.inc;
            self.count -= 1;
            self.value
        }
    }
}

pub struct PerfTimer {
    lbl: &'static str,
    i: std::time::Instant,
    off: bool,
    // let tb = std::time::Instant::now();
    // let ta = std::time::Instant::now().duration_since(ta);
    // let tb = std::time::Instant::now().duration_since(tb);
    // println!("ta Elapsed: {:?}", ta);
}

impl PerfTimer {
    #[inline]
    pub fn off(mut self) -> Self {
        self.off = true;
        self
    }

    #[inline]
    pub fn new(lbl: &'static str) -> Self {
        Self { lbl, i: std::time::Instant::now(), off: false }
    }

    #[inline]
    pub fn print(&mut self, lbl2: &str) {
        if self.off {
            return;
        }

        let t = std::time::Instant::now().duration_since(self.i);
        println!("*** PERF[{}/{}] {:?}", self.lbl, lbl2, t);
        self.i = std::time::Instant::now();
    }
}

// Implementation from vst-rs
// https://github.com/RustAudio/vst-rs/blob/master/src/util/atomic_float.rs
// Under MIT License
// Copyright (c) 2015 Marko Mijalkovic
pub struct AtomicFloat {
    atomic: AtomicU32,
}

impl AtomicFloat {
    /// New atomic float with initial value `value`.
    pub fn new(value: f32) -> AtomicFloat {
        AtomicFloat { atomic: AtomicU32::new(value.to_bits()) }
    }

    /// Get the current value of the atomic float.
    #[inline]
    pub fn get(&self) -> f32 {
        f32::from_bits(self.atomic.load(Ordering::Relaxed))
    }

    /// Set the value of the atomic float to `value`.
    #[inline]
    pub fn set(&self, value: f32) {
        self.atomic.store(value.to_bits(), Ordering::Relaxed)
    }
}

impl Default for AtomicFloat {
    fn default() -> Self {
        AtomicFloat::new(0.0)
    }
}

impl std::fmt::Debug for AtomicFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.get(), f)
    }
}

impl std::fmt::Display for AtomicFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.get(), f)
    }
}

impl From<f32> for AtomicFloat {
    fn from(value: f32) -> Self {
        AtomicFloat::new(value)
    }
}

impl From<AtomicFloat> for f32 {
    fn from(value: AtomicFloat) -> Self {
        value.get()
    }
}

/// The AtomicFloatPair can store two `f32` numbers atomically.
///
/// This is useful for storing eg. min and max values of a sampled signal.
pub struct AtomicFloatPair {
    atomic: AtomicU64,
}

impl AtomicFloatPair {
    /// New atomic float with initial value `value`.
    pub fn new(v: (f32, f32)) -> AtomicFloatPair {
        AtomicFloatPair {
            atomic: AtomicU64::new(((v.0.to_bits() as u64) << 32) | (v.1.to_bits() as u64)),
        }
    }

    /// Get the current value of the atomic float.
    #[inline]
    pub fn get(&self) -> (f32, f32) {
        let v = self.atomic.load(Ordering::Relaxed);
        (f32::from_bits((v >> 32 & 0xFFFFFFFF) as u32), f32::from_bits((v & 0xFFFFFFFF) as u32))
    }

    /// Set the value of the atomic float to `value`.
    #[inline]
    pub fn set(&self, v: (f32, f32)) {
        let v = ((v.0.to_bits() as u64) << 32) | (v.1.to_bits()) as u64;
        self.atomic.store(v, Ordering::Relaxed)
    }
}

impl Default for AtomicFloatPair {
    fn default() -> Self {
        AtomicFloatPair::new((0.0, 0.0))
    }
}

impl std::fmt::Debug for AtomicFloatPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.get();
        write!(f, "({}, {})", v.0, v.1)
    }
}

impl std::fmt::Display for AtomicFloatPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.get();
        write!(f, "({}, {})", v.0, v.1)
    }
}

impl From<(f32, f32)> for AtomicFloatPair {
    fn from(value: (f32, f32)) -> Self {
        AtomicFloatPair::new((value.0, value.1))
    }
}

impl From<AtomicFloatPair> for (f32, f32) {
    fn from(value: AtomicFloatPair) -> Self {
        value.get()
    }
}
