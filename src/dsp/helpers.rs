// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use num_traits::{cast::FromPrimitive, cast::ToPrimitive, Float, FloatConst};
use std::cell::RefCell;

macro_rules! trait_alias {
    ($name:ident = $base1:ident + $($base2:ident +)+) => {
        pub trait $name: $base1 $(+ $base2)+ { }
        impl<T: $base1 $(+ $base2)+> $name for T { }
    };
}

trait_alias!(Flt = Float + FloatConst + ToPrimitive + FromPrimitive +);

/// Logarithmic table size of the table in [fast_cos] / [fast_sin].
static FAST_COS_TAB_LOG2_SIZE: usize = 9;
/// Table size of the table in [fast_cos] / [fast_sin].
static FAST_COS_TAB_SIZE: usize = 1 << FAST_COS_TAB_LOG2_SIZE; // =512
/// The wave table of [fast_cos] / [fast_sin].
static mut FAST_COS_TAB: [f32; 513] = [0.0; 513];

/// Initializes the cosine wave table for [fast_cos] and [fast_sin].
pub fn init_cos_tab() {
    for i in 0..(FAST_COS_TAB_SIZE + 1) {
        let phase: f32 = (i as f32) * ((std::f32::consts::TAU) / (FAST_COS_TAB_SIZE as f32));
        unsafe {
            // XXX: note: mutable statics can be mutated by multiple
            //      threads: aliasing violations or data races
            //      will cause undefined behavior
            FAST_COS_TAB[i] = phase.cos();
        }
    }
}

/// Internal phase increment/scaling for [fast_cos].
const PHASE_SCALE: f32 = 1.0_f32 / (std::f32::consts::TAU);

/// A faster implementation of cosine. It's not that much faster than
/// Rust's built in cosine function. But YMMV.
///
/// Don't forget to call [init_cos_tab] before using this!
///
///```
/// use hexodsp::dsp::helpers::*;
/// init_cos_tab(); // Once on process initialization.
///
/// // ...
/// assert!((fast_cos(std::f32::consts::PI) - -1.0).abs() < 0.001);
///```
pub fn fast_cos(mut x: f32) -> f32 {
    x = x.abs(); // cosine is symmetrical around 0, let's get rid of negative values

    // normalize range from 0..2PI to 1..2
    let phase = x * PHASE_SCALE;

    let index = FAST_COS_TAB_SIZE as f32 * phase;

    let fract = index.fract();
    let index = index.floor() as usize;

    unsafe {
        // XXX: note: mutable statics can be mutated by multiple
        //      threads: aliasing violations or data races
        //      will cause undefined behavior
        let left = FAST_COS_TAB[index as usize];
        let right = FAST_COS_TAB[index as usize + 1];

        return left + (right - left) * fract;
    }
}

/// A faster implementation of sine. It's not that much faster than
/// Rust's built in sine function. But YMMV.
///
/// Don't forget to call [init_cos_tab] before using this!
///
///```
/// use hexodsp::dsp::helpers::*;
/// init_cos_tab(); // Once on process initialization.
///
/// // ...
/// assert!((fast_sin(0.5 * std::f32::consts::PI) - 1.0).abs() < 0.001);
///```
pub fn fast_sin(x: f32) -> f32 {
    fast_cos(x - (std::f32::consts::PI / 2.0))
}

/// A wavetable filled entirely with white noise.
/// Don't forget to call [init_white_noise_tab] before using it.
static mut WHITE_NOISE_TAB: [f64; 1024] = [0.0; 1024];

#[allow(rustdoc::private_intra_doc_links)]
/// Initializes [WHITE_NOISE_TAB].
pub fn init_white_noise_tab() {
    let mut rng = RandGen::new();
    unsafe {
        for i in 0..WHITE_NOISE_TAB.len() {
            WHITE_NOISE_TAB[i as usize] = rng.next_open01();
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// Random number generator based on xoroshiro128.
/// Requires two internal state variables. You may prefer [SplitMix64] or [Rng].
pub struct RandGen {
    r: [u64; 2],
}

// Taken from xoroshiro128 crate under MIT License
// Implemented by Matthew Scharley (Copyright 2016)
// https://github.com/mscharley/rust-xoroshiro128
/// Given the mutable `state` generates the next pseudo random number.
pub fn next_xoroshiro128(state: &mut [u64; 2]) -> u64 {
    let s0: u64 = state[0];
    let mut s1: u64 = state[1];
    let result: u64 = s0.wrapping_add(s1);

    s1 ^= s0;
    state[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
    state[1] = s1.rotate_left(36); // c

    result
}

// Taken from rand::distributions
// Licensed under the Apache License, Version 2.0
// Copyright 2018 Developers of the Rand project.
/// Maps any `u64` to a `f64` in the open interval `[0.0, 1.0)`.
pub fn u64_to_open01(u: u64) -> f64 {
    use core::f64::EPSILON;
    let float_size = std::mem::size_of::<f64>() as u32 * 8;
    let fraction = u >> (float_size - 52);
    let exponent_bits: u64 = (1023 as u64) << 52;
    f64::from_bits(fraction | exponent_bits) - (1.0 - EPSILON / 2.0)
}

impl RandGen {
    pub fn new() -> Self {
        RandGen { r: [0x193a6754a8a7d469, 0x97830e05113ba7bb] }
    }

    /// Next random unsigned 64bit integer.
    pub fn next(&mut self) -> u64 {
        next_xoroshiro128(&mut self.r)
    }

    /// Next random float between `[0.0, 1.0)`.
    pub fn next_open01(&mut self) -> f64 {
        u64_to_open01(self.next())
    }
}

#[derive(Debug, Copy, Clone)]
/// Random number generator based on [SplitMix64].
/// Requires two internal state variables. You may prefer [SplitMix64] or [Rng].
pub struct Rng {
    sm: SplitMix64,
}

impl Rng {
    pub fn new() -> Self {
        Self { sm: SplitMix64::new(0x193a67f4a8a6d769) }
    }

    pub fn seed(&mut self, seed: u64) {
        self.sm = SplitMix64::new(seed);
    }

    #[inline]
    pub fn next(&mut self) -> f32 {
        self.sm.next_open01() as f32
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.sm.next_u64()
    }
}

thread_local! {
    static GLOBAL_RNG: RefCell<Rng> = RefCell::new(Rng::new());
}

#[inline]
pub fn rand_01() -> f32 {
    GLOBAL_RNG.with(|r| r.borrow_mut().next())
}

#[inline]
pub fn rand_u64() -> u64 {
    GLOBAL_RNG.with(|r| r.borrow_mut().next_u64())
}

// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//- splitmix64 (http://xoroshiro.di.unimi.it/splitmix64.c)
//
/// A splitmix64 random number generator.
///
/// The splitmix algorithm is not suitable for cryptographic purposes, but is
/// very fast and has a 64 bit state.
///
/// The algorithm used here is translated from [the `splitmix64.c`
/// reference source code](http://xoshiro.di.unimi.it/splitmix64.c) by
/// Sebastiano Vigna. For `next_u32`, a more efficient mixing function taken
/// from [`dsiutils`](http://dsiutils.di.unimi.it/) is used.
#[derive(Debug, Copy, Clone)]
pub struct SplitMix64(pub u64);

/// Internal random constant for [SplitMix64].
const PHI: u64 = 0x9e3779b97f4a7c15;

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
    pub fn new_from_i64(seed: i64) -> Self {
        Self::new(u64::from_be_bytes(seed.to_be_bytes()))
    }

    pub fn new_time_seed() -> Self {
        use std::time::SystemTime;

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Self::new(n.as_secs() as u64),
            Err(_) => Self::new(123456789),
        }
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(PHI);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    #[inline]
    pub fn next_i64(&mut self) -> i64 {
        i64::from_be_bytes(self.next_u64().to_be_bytes())
    }

    #[inline]
    pub fn next_open01(&mut self) -> f64 {
        u64_to_open01(self.next_u64())
    }
}

/// Linear crossfade.
///
/// * `v1` - signal 1, range -1.0 to 1.0
/// * `v2` - signal 2, range -1.0 to 1.0
/// * `mix` - mix position, range 0.0 to 1.0, mid is at 0.5
#[inline]
pub fn crossfade<F: Flt>(v1: F, v2: F, mix: F) -> F {
    v1 * (f::<F>(1.0) - mix) + v2 * mix
}

/// Linear crossfade with clipping the `v2` result.
///
/// This crossfade actually does clip the `v2` signal to the -1.0 to 1.0
/// range. This is useful for Dry/Wet of plugins that might go beyond the
/// normal signal range.
///
/// * `v1` - signal 1, range -1.0 to 1.0
/// * `v2` - signal 2, range -1.0 to 1.0
/// * `mix` - mix position, range 0.0 to 1.0, mid is at 0.5
#[inline]
pub fn crossfade_clip<F: Flt>(v1: F, v2: F, mix: F) -> F {
    v1 * (f::<F>(1.0) - mix) + (v2 * mix).min(f::<F>(1.0)).max(f::<F>(-1.0))
}

/// Linear (f32) crossfade with driving the `v2` result through a tanh().
///
/// * `v1` - signal 1, range -1.0 to 1.0
/// * `v2` - signal 2, range -1.0 to 1.0
/// * `mix` - mix position, range 0.0 to 1.0, mid is at 0.5
#[inline]
pub fn crossfade_drive_tanh(v1: f32, v2: f32, mix: f32) -> f32 {
    v1 * (1.0 - mix) + tanh_approx_drive(v2 * mix * 0.111, 0.95) * 0.9999
}

/// Constant power crossfade.
///
/// * `v1` - signal 1, range -1.0 to 1.0
/// * `v2` - signal 2, range -1.0 to 1.0
/// * `mix` - mix position, range 0.0 to 1.0, mid is at 0.5
#[inline]
pub fn crossfade_cpow(v1: f32, v2: f32, mix: f32) -> f32 {
    let s1 = (mix * std::f32::consts::FRAC_PI_2).sin();
    let s2 = ((1.0 - mix) * std::f32::consts::FRAC_PI_2).sin();
    v1 * s2 + v2 * s1
}

const CROSS_LOG_MIN: f32 = -13.815510557964274; // (0.000001_f32).ln();
const CROSS_LOG_MAX: f32 = 0.0; // (1.0_f32).ln();

/// Logarithmic crossfade.
///
/// * `v1` - signal 1, range -1.0 to 1.0
/// * `v2` - signal 2, range -1.0 to 1.0
/// * `mix` - mix position, range 0.0 to 1.0, mid is at 0.5
#[inline]
pub fn crossfade_log(v1: f32, v2: f32, mix: f32) -> f32 {
    let x = (mix * (CROSS_LOG_MAX - CROSS_LOG_MIN) + CROSS_LOG_MIN).exp();
    crossfade(v1, v2, x)
}

/// Exponential crossfade.
///
/// * `v1` - signal 1, range -1.0 to 1.0
/// * `v2` - signal 2, range -1.0 to 1.0
/// * `mix` - mix position, range 0.0 to 1.0, mid is at 0.5
#[inline]
pub fn crossfade_exp(v1: f32, v2: f32, mix: f32) -> f32 {
    crossfade(v1, v2, mix * mix)
}

#[inline]
pub fn clamp(f: f32, min: f32, max: f32) -> f32 {
    if f < min {
        min
    } else if f > max {
        max
    } else {
        f
    }
}

pub fn square_135(phase: f32) -> f32 {
    fast_sin(phase) + fast_sin(phase * 3.0) / 3.0 + fast_sin(phase * 5.0) / 5.0
}

pub fn square_35(phase: f32) -> f32 {
    fast_sin(phase * 3.0) / 3.0 + fast_sin(phase * 5.0) / 5.0
}

// note: MIDI note value?
pub fn note_to_freq(note: f32) -> f32 {
    440.0 * (2.0_f32).powf((note - 69.0) / 12.0)
}

// Ported from LMMS under GPLv2
// * DspEffectLibrary.h - library with template-based inline-effects
// * Copyright (c) 2006-2014 Tobias Doerffel <tobydox/at/users.sourceforge.net>
//
// Original source seems to be musicdsp.org, Author: Bram de Jong
// see also: https://www.musicdsp.org/en/latest/Effects/41-waveshaper.html
// Notes:
//     where x (in [-1..1] will be distorted and a is a distortion parameter
//     that goes from 1 to infinity. The equation is valid for positive and
//     negativ values. If a is 1, it results in a slight distortion and with
//     bigger a's the signal get's more funky.
//     A good thing about the shaper is that feeding it with bigger-than-one
//     values, doesn't create strange fx. The maximum this function will reach
//     is 1.2 for a=1.
//
//     f(x,a) = x*(abs(x) + a)/(x^2 + (a-1)*abs(x) + 1)
/// Signal distortion by Bram de Jong.
/// ```text
/// gain:        0.1 - 5.0       default = 1.0
/// threshold:   0.0 - 100.0     default = 0.8
/// i:           signal
/// ```
#[inline]
pub fn f_distort(gain: f32, threshold: f32, i: f32) -> f32 {
    gain * (i * (i.abs() + threshold) / (i * i + (threshold - 1.0) * i.abs() + 1.0))
}

// Ported from LMMS under GPLv2
// * DspEffectLibrary.h - library with template-based inline-effects
// * Copyright (c) 2006-2014 Tobias Doerffel <tobydox/at/users.sourceforge.net>
//
/// Foldback Signal distortion
/// ```text
/// gain:        0.1 - 5.0       default = 1.0
/// threshold:   0.0 - 100.0     default = 0.8
/// i:           signal
/// ```
#[inline]
pub fn f_fold_distort(gain: f32, threshold: f32, i: f32) -> f32 {
    if i >= threshold || i < -threshold {
        gain * ((((i - threshold) % threshold * 4.0).abs() - threshold * 2.0).abs() - threshold)
    } else {
        gain * i
    }
}

/// Apply linear interpolation between the value a and b.
///
/// * `a` - value at x=0.0
/// * `b` - value at x=1.0
/// * `x` - value between 0.0 and 1.0 to blend between `a` and `b`.
#[inline]
pub fn lerp(x: f32, a: f32, b: f32) -> f32 {
    (a * (1.0 - x)) + (b * x)
}

/// Apply 64bit linear interpolation between the value a and b.
///
/// * `a` - value at x=0.0
/// * `b` - value at x=1.0
/// * `x` - value between 0.0 and 1.0 to blend between `a` and `b`.
pub fn lerp64(x: f64, a: f64, b: f64) -> f64 {
    (a * (1.0 - x)) + (b * x)
}

pub fn p2range(x: f32, a: f32, b: f32) -> f32 {
    lerp(x, a, b)
}

pub fn p2range_exp(x: f32, a: f32, b: f32) -> f32 {
    let x = x * x;
    (a * (1.0 - x)) + (b * x)
}

pub fn p2range_exp4(x: f32, a: f32, b: f32) -> f32 {
    let x = x * x * x * x;
    (a * (1.0 - x)) + (b * x)
}

pub fn range2p(v: f32, a: f32, b: f32) -> f32 {
    ((v - a) / (b - a)).abs()
}

pub fn range2p_exp(v: f32, a: f32, b: f32) -> f32 {
    (((v - a) / (b - a)).abs()).sqrt()
}

pub fn range2p_exp4(v: f32, a: f32, b: f32) -> f32 {
    (((v - a) / (b - a)).abs()).sqrt().sqrt()
}

/// ```text
/// gain: 24.0 - -90.0   default = 0.0
/// ```
pub fn gain2coef(gain: f32) -> f32 {
    if gain > -90.0 {
        10.0_f32.powf(gain * 0.05)
    } else {
        0.0
    }
}

// quickerTanh / quickerTanh64 credits to mopo synthesis library:
// Under GPLv3 or any later.
// Little IO <littleioaudio@gmail.com>
// Matt Tytel <matthewtytel@gmail.com>
pub fn quicker_tanh64(v: f64) -> f64 {
    let square = v * v;
    v / (1.0 + square / (3.0 + square / 5.0))
}

#[inline]
pub fn quicker_tanh(v: f32) -> f32 {
    let square = v * v;
    v / (1.0 + square / (3.0 + square / 5.0))
}

// quickTanh / quickTanh64 credits to mopo synthesis library:
// Under GPLv3 or any later.
// Little IO <littleioaudio@gmail.com>
// Matt Tytel <matthewtytel@gmail.com>
pub fn quick_tanh64(v: f64) -> f64 {
    let abs_v = v.abs();
    let square = v * v;
    let num = v
        * (2.45550750702956
            + 2.45550750702956 * abs_v
            + square * (0.893229853513558 + 0.821226666969744 * abs_v));
    let den =
        2.44506634652299 + (2.44506634652299 + square) * (v + 0.814642734961073 * v * abs_v).abs();

    num / den
}

pub fn quick_tanh(v: f32) -> f32 {
    let abs_v = v.abs();
    let square = v * v;
    let num = v
        * (2.45550750702956
            + 2.45550750702956 * abs_v
            + square * (0.893229853513558 + 0.821226666969744 * abs_v));
    let den =
        2.44506634652299 + (2.44506634652299 + square) * (v + 0.814642734961073 * v * abs_v).abs();

    num / den
}

// Taken from ValleyAudio
// Copyright Dale Johnson
// https://github.dev/ValleyAudio/ValleyRackFree/tree/v2.0
// Under GPLv3 license
pub fn tanh_approx_drive(v: f32, drive: f32) -> f32 {
    let x = v * drive;

    if x < -1.25 {
        -1.0
    } else if x < -0.75 {
        1.0 - (x * (-2.5 - x) - 0.5625) - 1.0
    } else if x > 1.25 {
        1.0
    } else if x > 0.75 {
        x * (2.5 - x) - 0.5625
    } else {
        x
    }
}

/// A helper function for exponential envelopes.
/// It's a bit faster than calling the `pow` function of Rust.
///
/// * `x` the input value
/// * `v' the shape value.
/// Which is linear at `0.5`, the forth root of `x` at `1.0` and x to the power
/// of 4 at `0.0`. You can vary `v` as you like.
///
///```
/// use hexodsp::dsp::helpers::*;
///
/// assert!(((sqrt4_to_pow4(0.25, 0.0) - 0.25_f32 * 0.25 * 0.25 * 0.25)
///          .abs() - 1.0)
///         < 0.0001);
///
/// assert!(((sqrt4_to_pow4(0.25, 1.0) - (0.25_f32).sqrt().sqrt())
///          .abs() - 1.0)
///         < 0.0001);
///
/// assert!(((sqrt4_to_pow4(0.25, 0.5) - 0.25_f32).abs() - 1.0) < 0.0001);
///```
#[inline]
pub fn sqrt4_to_pow4(x: f32, v: f32) -> f32 {
    if v > 0.75 {
        let xsq1 = x.sqrt();
        let xsq = xsq1.sqrt();
        let v = (v - 0.75) * 4.0;
        xsq1 * (1.0 - v) + xsq * v
    } else if v > 0.5 {
        let xsq = x.sqrt();
        let v = (v - 0.5) * 4.0;
        x * (1.0 - v) + xsq * v
    } else if v > 0.25 {
        let xx = x * x;
        let v = (v - 0.25) * 4.0;
        x * v + xx * (1.0 - v)
    } else {
        let xx = x * x;
        let xxxx = xx * xx;
        let v = v * 4.0;
        xx * v + xxxx * (1.0 - v)
    }
}

/// A-100 Eurorack states, that a trigger is usually 2-10 milliseconds.
pub const TRIG_SIGNAL_LENGTH_MS: f32 = 2.0;

/// The lower threshold for the schmidt trigger to reset.
pub const TRIG_LOW_THRES: f32 = 0.25;
/// The threshold, once reached, will cause a trigger event and signals
/// a logical '1'. Anything below this is a logical '0'.
pub const TRIG_HIGH_THRES: f32 = 0.5;

/// Trigger signal generator for HexoDSP nodes.
///
/// A trigger in HexoSynth and HexoDSP is commonly 2.0 milliseconds.
/// This generator generates a trigger signal when [TrigSignal::trigger] is called.
#[derive(Debug, Clone, Copy)]
pub struct TrigSignal {
    length: u32,
    scount: u32,
}

impl TrigSignal {
    /// Create a new trigger generator
    pub fn new() -> Self {
        Self { length: ((44100.0 * TRIG_SIGNAL_LENGTH_MS) / 1000.0).ceil() as u32, scount: 0 }
    }

    /// Reset the trigger generator.
    pub fn reset(&mut self) {
        self.scount = 0;
    }

    /// Set the sample rate to calculate the amount of samples for the trigger signal.
    pub fn set_sample_rate(&mut self, srate: f32) {
        self.length = ((srate * TRIG_SIGNAL_LENGTH_MS) / 1000.0).ceil() as u32;
        self.scount = 0;
    }

    /// Enable sending a trigger impulse the next time [TrigSignal::next] is called.
    #[inline]
    pub fn trigger(&mut self) {
        self.scount = self.length;
    }

    /// Trigger signal output.
    #[inline]
    pub fn next(&mut self) -> f32 {
        if self.scount > 0 {
            self.scount -= 1;
            1.0
        } else {
            0.0
        }
    }
}

impl Default for TrigSignal {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal change detector that emits a trigger when the input signal changed.
///
/// This is commonly used for control signals. It has not much use for audio signals.
#[derive(Debug, Clone, Copy)]
pub struct ChangeTrig {
    ts: TrigSignal,
    last: f32,
}

impl ChangeTrig {
    /// Create a new change detector
    pub fn new() -> Self {
        Self {
            ts: TrigSignal::new(),
            last: -100.0, // some random value :-)
        }
    }

    /// Reset internal state.
    pub fn reset(&mut self) {
        self.ts.reset();
        self.last = -100.0;
    }

    /// Set the sample rate for the trigger signal generator
    pub fn set_sample_rate(&mut self, srate: f32) {
        self.ts.set_sample_rate(srate);
    }

    /// Feed a new input signal sample.
    ///
    /// The return value is the trigger signal.
    #[inline]
    pub fn next(&mut self, inp: f32) -> f32 {
        if (inp - self.last).abs() > std::f32::EPSILON {
            self.ts.trigger();
            self.last = inp;
        }

        self.ts.next()
    }
}

impl Default for ChangeTrig {
    fn default() -> Self {
        Self::new()
    }
}

/// Trigger signal detector for HexoDSP.
///
/// Whenever you need to detect a trigger on an input you can use this component.
/// A trigger in HexoDSP is any signal over [TRIG_HIGH_THRES]. The internal state is
/// resetted when the signal drops below [TRIG_LOW_THRES].
#[derive(Debug, Clone, Copy)]
pub struct Trigger {
    triggered: bool,
}

impl Trigger {
    /// Create a new trigger detector.
    pub fn new() -> Self {
        Self { triggered: false }
    }

    /// Reset the internal state of the trigger detector.
    #[inline]
    pub fn reset(&mut self) {
        self.triggered = false;
    }

    /// Checks the input signal for a trigger and returns true when the signal
    /// surpassed [TRIG_HIGH_THRES] and has not fallen below [TRIG_LOW_THRES] yet.
    #[inline]
    pub fn check_trigger(&mut self, input: f32) -> bool {
        if self.triggered {
            if input <= TRIG_LOW_THRES {
                self.triggered = false;
            }

            false
        } else if input > TRIG_HIGH_THRES {
            self.triggered = true;
            true
        } else {
            false
        }
    }
}

/// Trigger signal detector with custom range.
///
/// Whenever you need to detect a trigger with a custom threshold.
#[derive(Debug, Clone, Copy)]
pub struct CustomTrigger {
    triggered: bool,
    low_thres: f32,
    high_thres: f32,
}

impl CustomTrigger {
    /// Create a new trigger detector.
    pub fn new(low_thres: f32, high_thres: f32) -> Self {
        Self { triggered: false, low_thres, high_thres }
    }

    pub fn set_threshold(&mut self, low_thres: f32, high_thres: f32) {
        self.low_thres = low_thres;
        self.high_thres = high_thres;
    }

    /// Reset the internal state of the trigger detector.
    #[inline]
    pub fn reset(&mut self) {
        self.triggered = false;
    }

    /// Checks the input signal for a trigger and returns true when the signal
    /// surpassed the high threshold and has not fallen below low threshold yet.
    #[inline]
    pub fn check_trigger(&mut self, input: f32) -> bool {
        //        println!("TRIG CHECK: {} <> {}", input, self.high_thres);
        if self.triggered {
            if input <= self.low_thres {
                self.triggered = false;
            }

            false
        } else if input > self.high_thres {
            self.triggered = true;
            true
        } else {
            false
        }
    }
}

/// Generates a phase signal from a trigger/gate input signal.
///
/// This helper allows you to measure the distance between trigger or gate pulses
/// and generates a phase signal for you that increases from 0.0 to 1.0.
#[derive(Debug, Clone, Copy)]
pub struct TriggerPhaseClock {
    clock_phase: f64,
    clock_inc: f64,
    prev_trigger: bool,
    clock_samples: u32,
}

impl TriggerPhaseClock {
    /// Create a new phase clock.
    pub fn new() -> Self {
        Self { clock_phase: 0.0, clock_inc: 0.0, prev_trigger: true, clock_samples: 0 }
    }

    /// Reset the phase clock.
    #[inline]
    pub fn reset(&mut self) {
        self.clock_samples = 0;
        self.clock_inc = 0.0;
        self.prev_trigger = true;
        self.clock_samples = 0;
    }

    /// Restart the phase clock. It will count up from 0.0 again on [TriggerPhaseClock::next_phase].
    #[inline]
    pub fn sync(&mut self) {
        self.clock_phase = 0.0;
    }

    /// Generate the phase signal of this clock.
    ///
    /// * `clock_limit` - The maximum number of samples to detect two trigger signals in.
    /// * `trigger_in` - Trigger signal input.
    #[inline]
    pub fn next_phase(&mut self, clock_limit: f64, trigger_in: f32) -> f64 {
        if self.prev_trigger {
            if trigger_in <= TRIG_LOW_THRES {
                self.prev_trigger = false;
            }
        } else if trigger_in > TRIG_HIGH_THRES {
            self.prev_trigger = true;

            if self.clock_samples > 0 {
                self.clock_inc = 1.0 / (self.clock_samples as f64);
            }

            self.clock_samples = 0;
        }

        self.clock_samples += 1;

        self.clock_phase += self.clock_inc;
        self.clock_phase = self.clock_phase % clock_limit;

        self.clock_phase
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TriggerSampleClock {
    prev_trigger: bool,
    clock_samples: u32,
    counter: u32,
}

impl TriggerSampleClock {
    pub fn new() -> Self {
        Self { prev_trigger: true, clock_samples: 0, counter: 0 }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.clock_samples = 0;
        self.counter = 0;
    }

    #[inline]
    pub fn next(&mut self, trigger_in: f32) -> u32 {
        if self.prev_trigger {
            if trigger_in <= TRIG_LOW_THRES {
                self.prev_trigger = false;
            }
        } else if trigger_in > TRIG_HIGH_THRES {
            self.prev_trigger = true;
            self.clock_samples = self.counter;
            self.counter = 0;
        }

        self.counter += 1;

        self.clock_samples
    }
}

/// A slew rate limiter, with a configurable time per 1.0 increase.
#[derive(Debug, Clone, Copy)]
pub struct SlewValue<F: Flt> {
    current: F,
    slew_per_ms: F,
}

impl<F: Flt> SlewValue<F> {
    pub fn new() -> Self {
        Self { current: f(0.0), slew_per_ms: f(1000.0 / 44100.0) }
    }

    pub fn reset(&mut self) {
        self.current = f(0.0);
    }

    pub fn set_sample_rate(&mut self, srate: F) {
        self.slew_per_ms = f::<F>(1000.0) / srate;
    }

    #[inline]
    pub fn value(&self) -> F {
        self.current
    }

    /// * `slew_ms_per_1` - The time (in milliseconds) it should take
    /// to get to 1.0 from 0.0.
    #[inline]
    pub fn next(&mut self, target: F, slew_ms_per_1: F) -> F {
        // at 0.11ms, there are barely enough samples for proper slew.
        if slew_ms_per_1 < f(0.11) {
            self.current = target;
        } else {
            let max_delta = self.slew_per_ms / slew_ms_per_1;
            self.current = target.min(self.current + max_delta).max(self.current - max_delta);
        }

        self.current
    }
}

/// A ramped value changer, with a configurable time to reach the target value.
#[derive(Debug, Clone, Copy)]
pub struct RampValue<F: Flt> {
    slew_count: u64,
    current: F,
    target: F,
    inc: F,
    sr_ms: F,
}

impl<F: Flt> RampValue<F> {
    pub fn new() -> Self {
        Self {
            slew_count: 0,
            current: f(0.0),
            target: f(0.0),
            inc: f(0.0),
            sr_ms: f(44100.0 / 1000.0),
        }
    }

    pub fn reset(&mut self) {
        self.slew_count = 0;
        self.current = f(0.0);
        self.target = f(0.0);
        self.inc = f(0.0);
    }

    pub fn set_sample_rate(&mut self, srate: F) {
        self.sr_ms = srate / f(1000.0);
    }

    #[inline]
    pub fn set_target(&mut self, target: F, slew_time_ms: F) {
        self.target = target;

        // 0.02ms, thats a fraction of a sample at 44.1kHz
        if slew_time_ms < f(0.02) {
            self.current = self.target;
            self.slew_count = 0;
        } else {
            let slew_samples = slew_time_ms * self.sr_ms;
            self.slew_count = slew_samples.to_u64().unwrap_or(0);
            self.inc = (self.target - self.current) / slew_samples;
        }
    }

    #[inline]
    pub fn value(&self) -> F {
        self.current
    }

    #[inline]
    pub fn next(&mut self) -> F {
        if self.slew_count > 0 {
            self.current = self.current + self.inc;
            self.slew_count -= 1;
        } else {
            self.current = self.target;
        }

        self.current
    }
}

/// Default size of the delay buffer: 5 seconds at 8 times 48kHz
const DEFAULT_DELAY_BUFFER_SAMPLES: usize = 8 * 48000 * 5;

macro_rules! fc {
    ($F: ident, $e: expr) => {
        F::from_f64($e).unwrap()
    };
}

#[allow(dead_code)]
#[inline]
fn f<F: Flt>(x: f64) -> F {
    F::from_f64(x).unwrap()
}

#[allow(dead_code)]
#[inline]
fn fclamp<F: Flt>(x: F, mi: F, mx: F) -> F {
    x.max(mi).min(mx)
}

#[allow(dead_code)]
#[inline]
fn fclampc<F: Flt>(x: F, mi: f64, mx: f64) -> F {
    x.max(f(mi)).min(f(mx))
}

/// Hermite / Cubic interpolation of a buffer full of samples at the given _index_.
/// _len_ is the buffer length to consider and wrap the index into. And _fract_ is the
/// fractional part of the index.
///
/// This function is generic over f32 and f64. That means you can use your preferred float size.
///
/// Commonly used like this:
///
///```
/// use hexodsp::dsp::helpers::cubic_interpolate;
///
/// let buf : [f32; 9] = [1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];
/// let pos = 3.3_f32;
///
/// let i = pos.floor() as usize;
/// let f = pos.fract();
///
/// let res = cubic_interpolate(&buf[..], buf.len(), i, f);
/// assert!((res - 0.67).abs() < 0.2_f32);
///```
#[inline]
pub fn cubic_interpolate<F: Flt>(data: &[F], len: usize, index: usize, fract: F) -> F {
    let index = index + len;
    // Hermite interpolation, take from
    // https://github.com/eric-wood/delay/blob/main/src/delay.rs#L52
    //
    // Thanks go to Eric Wood!
    //
    // For the interpolation code:
    // MIT License, Copyright (c) 2021 Eric Wood
    let xm1 = data[(index - 1) % len];
    let x0 = data[index % len];
    let x1 = data[(index + 1) % len];
    let x2 = data[(index + 2) % len];

    let c = (x1 - xm1) * f(0.5);
    let v = x0 - x1;
    let w = c + v;
    let a = w + v + (x2 - x0) * f(0.5);
    let b_neg = w + a;

    let res = (((a * fract) - b_neg) * fract + c) * fract + x0;

    // let rr2 =
    //     x0 + f::<F>(0.5) * fract * (
    //         x1 - xm1 + fract * (
    //             f::<F>(4.0) * x1
    //             + f::<F>(2.0) * xm1
    //             - f::<F>(5.0) * x0
    //             - x2
    //             + fract * (f::<F>(3.0) * (x0 - x1) - xm1 + x2)));

    // eprintln!(
    //     "index={} fract={:6.4} xm1={:6.4} x0={:6.4} x1={:6.4} x2={:6.4} = {:6.4} <> {:6.4}",
    //     index, fract.to_f64().unwrap(), xm1.to_f64().unwrap(), x0.to_f64().unwrap(), x1.to_f64().unwrap(), x2.to_f64().unwrap(),
    //     res.to_f64().unwrap(),
    //     rr2.to_f64().unwrap()
    // );

    // eprintln!(
    //     "index={} fract={:6.4} xm1={:6.4} x0={:6.4} x1={:6.4} x2={:6.4} = {:6.4}",
    //     index, fract.to_f64().unwrap(), xm1.to_f64().unwrap(), x0.to_f64().unwrap(), x1.to_f64().unwrap(), x2.to_f64().unwrap(),
    //     res.to_f64().unwrap(),
    // );

    res
}

/// This is a delay buffer/line with linear and cubic interpolation.
///
/// It's the basic building block underneath the all-pass filter, comb filters and delay effects.
/// You can use linear and cubic and no interpolation to access samples in the past. Either
/// by sample offset or time (millisecond) based.
#[derive(Debug, Clone, Default)]
pub struct DelayBuffer<F: Flt> {
    data: Vec<F>,
    wr: usize,
    srate: F,
}

impl<F: Flt> DelayBuffer<F> {
    /// Creates a delay buffer with about 5 seconds of capacity at 8*48000Hz sample rate.
    pub fn new() -> Self {
        Self { data: vec![f(0.0); DEFAULT_DELAY_BUFFER_SAMPLES], wr: 0, srate: f(44100.0) }
    }

    /// Creates a delay buffer with the given amount of samples capacity.
    pub fn new_with_size(size: usize) -> Self {
        Self { data: vec![f(0.0); size], wr: 0, srate: f(44100.0) }
    }

    /// Sets the sample rate that is used for milliseconds => sample conversion.
    pub fn set_sample_rate(&mut self, srate: F) {
        self.srate = srate;
    }

    /// Reset the delay buffer contents and write position.
    pub fn reset(&mut self) {
        self.data.fill(f(0.0));
        self.wr = 0;
    }

    /// Feed one sample into the delay line and increment the write pointer.
    /// Please note: For sample accurate feedback you need to retrieve the
    /// output of the delay line before feeding in a new signal.
    #[inline]
    pub fn feed(&mut self, input: F) {
        self.data[self.wr] = input;
        self.wr = (self.wr + 1) % self.data.len();
    }

    /// Combines [DelayBuffer::cubic_interpolate_at] and [DelayBuffer::feed]
    /// into one convenient function.
    #[inline]
    pub fn next_cubic(&mut self, delay_time_ms: F, input: F) -> F {
        let res = self.cubic_interpolate_at(delay_time_ms);
        self.feed(input);
        res
    }

    /// Combines [DelayBuffer::linear_interpolate_at] and [DelayBuffer::feed]
    /// into one convenient function.
    #[inline]
    pub fn next_linear(&mut self, delay_time_ms: F, input: F) -> F {
        let res = self.linear_interpolate_at(delay_time_ms);
        self.feed(input);
        res
    }

    /// Combines [DelayBuffer::nearest_at] and [DelayBuffer::feed]
    /// into one convenient function.
    #[inline]
    pub fn next_nearest(&mut self, delay_time_ms: F, input: F) -> F {
        let res = self.nearest_at(delay_time_ms);
        self.feed(input);
        res
    }

    /// Shorthand for [DelayBuffer::cubic_interpolate_at].
    #[inline]
    pub fn tap_c(&self, delay_time_ms: F) -> F {
        self.cubic_interpolate_at(delay_time_ms)
    }

    /// Shorthand for [DelayBuffer::cubic_interpolate_at].
    #[inline]
    pub fn tap_n(&self, delay_time_ms: F) -> F {
        self.nearest_at(delay_time_ms)
    }

    /// Shorthand for [DelayBuffer::cubic_interpolate_at].
    #[inline]
    pub fn tap_l(&self, delay_time_ms: F) -> F {
        self.linear_interpolate_at(delay_time_ms)
    }

    /// Fetch a sample from the delay buffer at the given tim with linear interpolation.
    ///
    /// * `delay_time_ms` - Delay time in milliseconds.
    #[inline]
    pub fn linear_interpolate_at(&self, delay_time_ms: F) -> F {
        self.linear_interpolate_at_s((delay_time_ms * self.srate) / f(1000.0))
    }

    /// Fetch a sample from the delay buffer at the given offset with linear interpolation.
    ///
    /// * `s_offs` - Sample offset in samples.
    #[inline]
    pub fn linear_interpolate_at_s(&self, s_offs: F) -> F {
        let data = &self.data[..];
        let len = data.len();
        let offs = s_offs.floor().to_usize().unwrap_or(0) % len;
        let fract = s_offs.fract();

        // one extra offset, because feed() advances self.wr to the next writing position!
        let i = (self.wr + len) - (offs + 1);
        let x0 = data[i % len];
        let x1 = data[(i - 1) % len];

        let res = x0 + fract * (x1 - x0);
        //d// eprintln!(
        //d//     "INTERP: {:6.4} x0={:6.4} x1={:6.4} fract={:6.4} => {:6.4}",
        //d//     s_offs.to_f64().unwrap_or(0.0),
        //d//     x0.to_f64().unwrap(),
        //d//     x1.to_f64().unwrap(),
        //d//     fract.to_f64().unwrap(),
        //d//     res.to_f64().unwrap(),
        //d// );
        res
    }

    /// Fetch a sample from the delay buffer at the given time with cubic interpolation.
    ///
    /// * `delay_time_ms` - Delay time in milliseconds.
    #[inline]
    pub fn cubic_interpolate_at(&self, delay_time_ms: F) -> F {
        self.cubic_interpolate_at_s((delay_time_ms * self.srate) / f(1000.0))
    }

    /// Fetch a sample from the delay buffer at the given offset with cubic interpolation.
    ///
    /// * `s_offs` - Sample offset in samples into the past of the [DelayBuffer]
    /// from the current write (or the "now") position.
    #[inline]
    pub fn cubic_interpolate_at_s(&self, s_offs: F) -> F {
        let data = &self.data[..];
        let len = data.len();
        let offs = s_offs.floor().to_usize().unwrap_or(0) % len;
        let fract = s_offs.fract();

        // (offs + 1) offset for compensating that self.wr points to the next
        // unwritten position.
        // Additional (offs + 1 + 1) offset for cubic_interpolate, which
        // interpolates into the past through the delay buffer.
        let i = (self.wr + len) - (offs + 2);
        let res = cubic_interpolate(data, len, i, f::<F>(1.0) - fract);
        //        eprintln!(
        //            "cubic at={} ({:6.4}) res={:6.4}",
        //            i % len,
        //            s_offs.to_f64().unwrap(),
        //            res.to_f64().unwrap()
        //        );
        res
    }

    /// Fetch a sample from the delay buffer at the given time without any interpolation.
    ///
    /// * `delay_time_ms` - Delay time in milliseconds.
    #[inline]
    pub fn nearest_at(&self, delay_time_ms: F) -> F {
        let len = self.data.len();
        let offs = ((delay_time_ms * self.srate) / f(1000.0)).floor().to_usize().unwrap_or(0) % len;
        // (offs + 1) one extra offset, because feed() advances
        // self.wr to the next writing position!
        let idx = ((self.wr + len) - (offs + 1)) % len;
        self.data[idx]
    }

    /// Fetch a sample from the delay buffer at the given number of samples in the past.
    #[inline]
    pub fn at(&self, delay_sample_count: usize) -> F {
        let len = self.data.len();
        // (delay_sample_count + 1) one extra offset, because feed() advances self.wr to
        // the next writing position!
        let idx = ((self.wr + len) - (delay_sample_count + 1)) % len;
        self.data[idx]
    }
}

/// Default size of the delay buffer: 1 seconds at 8 times 48kHz
const DEFAULT_ALLPASS_COMB_SAMPLES: usize = 8 * 48000;

/// An all-pass filter based on a delay line.
#[derive(Debug, Clone, Default)]
pub struct AllPass<F: Flt> {
    delay: DelayBuffer<F>,
}

impl<F: Flt> AllPass<F> {
    /// Creates a new all-pass filter with about 1 seconds space for samples.
    pub fn new() -> Self {
        Self { delay: DelayBuffer::new_with_size(DEFAULT_ALLPASS_COMB_SAMPLES) }
    }

    /// Set the sample rate for millisecond based access.
    pub fn set_sample_rate(&mut self, srate: F) {
        self.delay.set_sample_rate(srate);
    }

    /// Reset the internal delay buffer.
    pub fn reset(&mut self) {
        self.delay.reset();
    }

    /// Access the internal delay at the given amount of milliseconds in the past.
    #[inline]
    pub fn delay_tap_n(&self, time_ms: F) -> F {
        self.delay.tap_n(time_ms)
    }

    /// Retrieve the next (cubic interpolated) sample from the all-pass
    /// filter while feeding in the next.
    ///
    /// * `time_ms` - Delay time in milliseconds.
    /// * `g` - Feedback factor (usually something around 0.7 is common)
    /// * `v` - The new input sample to feed the filter.
    #[inline]
    pub fn next(&mut self, time_ms: F, g: F, v: F) -> F {
        let s = self.delay.cubic_interpolate_at(time_ms);
        let input = v + -g * s;
        self.delay.feed(input);
        input * g + s
    }

    /// Retrieve the next (linear interpolated) sample from the all-pass
    /// filter while feeding in the next.
    ///
    /// * `time_ms` - Delay time in milliseconds.
    /// * `g` - Feedback factor (usually something around 0.7 is common)
    /// * `v` - The new input sample to feed the filter.
    #[inline]
    pub fn next_linear(&mut self, time_ms: F, g: F, v: F) -> F {
        let s = self.delay.linear_interpolate_at(time_ms);
        let input = v + -g * s;
        self.delay.feed(input);
        input * g + s
    }
}

#[derive(Debug, Clone)]
pub struct Comb {
    delay: DelayBuffer<f32>,
}

impl Comb {
    pub fn new() -> Self {
        Self { delay: DelayBuffer::new_with_size(DEFAULT_ALLPASS_COMB_SAMPLES) }
    }

    pub fn set_sample_rate(&mut self, srate: f32) {
        self.delay.set_sample_rate(srate);
    }

    pub fn reset(&mut self) {
        self.delay.reset();
    }

    #[inline]
    pub fn delay_tap_c(&self, time_ms: f32) -> f32 {
        self.delay.tap_c(time_ms)
    }

    #[inline]
    pub fn delay_tap_n(&self, time_ms: f32) -> f32 {
        self.delay.tap_n(time_ms)
    }

    #[inline]
    pub fn next_feedback(&mut self, time: f32, g: f32, v: f32) -> f32 {
        let s = self.delay.cubic_interpolate_at(time);
        let v = v + s * g;
        self.delay.feed(v);
        v
    }

    #[inline]
    pub fn next_feedforward(&mut self, time: f32, g: f32, v: f32) -> f32 {
        let s = self.delay.next_cubic(time, v);
        v + s * g
    }
}

// one pole lp from valley rack free:
// https://github.com/ValleyAudio/ValleyRackFree/blob/v1.0/src/Common/DSP/OnePoleFilters.cpp
#[inline]
/// Process a very simple one pole 6dB low pass filter.
/// Useful in various applications, from usage in a synthesizer to
/// damping stuff in a reverb/delay.
///
/// * `input`  - Input sample
/// * `freq`   - Frequency between 1.0 and 22000.0Hz
/// * `israte` - 1.0 / samplerate
/// * `z`      - The internal one sample buffer of the filter.
///
///```
///    use hexodsp::dsp::helpers::*;
///
///    let samples  = vec![0.0; 44100];
///    let mut z    = 0.0;
///    let mut freq = 1000.0;
///
///    for s in samples.iter() {
///        let s_out =
///            process_1pole_lowpass(*s, freq, 1.0 / 44100.0, &mut z);
///        // ... do something with the result here.
///    }
///```
pub fn process_1pole_lowpass(input: f32, freq: f32, israte: f32, z: &mut f32) -> f32 {
    let b = (-std::f32::consts::TAU * freq * israte).exp();
    let a = 1.0 - b;
    *z = a * input + *z * b;
    *z
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OnePoleLPF<F: Flt> {
    israte: F,
    a: F,
    b: F,
    freq: F,
    z: F,
}

impl<F: Flt> OnePoleLPF<F> {
    pub fn new() -> Self {
        Self {
            israte: f::<F>(1.0) / f(44100.0),
            a: f::<F>(0.0),
            b: f::<F>(0.0),
            freq: f::<F>(1000.0),
            z: f::<F>(0.0),
        }
    }

    pub fn reset(&mut self) {
        self.z = f(0.0);
    }

    #[inline]
    fn recalc(&mut self) {
        self.b = (f::<F>(-1.0) * F::TAU() * self.freq * self.israte).exp();
        self.a = f::<F>(1.0) - self.b;
    }

    #[inline]
    pub fn set_sample_rate(&mut self, srate: F) {
        self.israte = f::<F>(1.0) / srate;
        self.recalc();
    }

    #[inline]
    pub fn set_freq(&mut self, freq: F) {
        if freq != self.freq {
            self.freq = freq;
            self.recalc();
        }
    }

    #[inline]
    pub fn process(&mut self, input: F) -> F {
        self.z = self.a * input + self.z * self.b;
        self.z
    }
}

// Fixed one pole with setable pole and gain.
// Implementation taken from tubonitaub / alec-deason
// from https://github.com/alec-deason/virtual_modular/blob/4025f1ef343c2eb9cd74eac07b5350c1e7ec9c09/src/simd_graph.rs#L4292
// under MIT License
#[derive(Debug, Copy, Clone, Default)]
pub struct FixedOnePole {
    b0: f32,
    a1: f32,
    y1: f32,
    gain: f32,
}

impl FixedOnePole {
    pub fn new(pole: f32, gain: f32) -> Self {
        let b0 = if pole > 0.0 { 1.0 - pole } else { 1.0 + pole };

        Self { b0, a1: -pole, y1: 0.0, gain }
    }

    pub fn reset(&mut self) {
        self.y1 = 0.0;
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * self.gain * input - self.a1 * self.y1;
        self.y1 = output;
        output
    }
}

// one pole hp from valley rack free:
// https://github.com/ValleyAudio/ValleyRackFree/blob/v1.0/src/Common/DSP/OnePoleFilters.cpp
#[inline]
/// Process a very simple one pole 6dB high pass filter.
/// Useful in various applications.
///
/// * `input`  - Input sample
/// * `freq`   - Frequency between 1.0 and 22000.0Hz
/// * `israte` - 1.0 / samplerate
/// * `z`      - The first internal buffer of the filter.
/// * `y`      - The second internal buffer of the filter.
///
///```
///    use hexodsp::dsp::helpers::*;
///
///    let samples  = vec![0.0; 44100];
///    let mut z    = 0.0;
///    let mut y    = 0.0;
///    let mut freq = 1000.0;
///
///    for s in samples.iter() {
///        let s_out =
///            process_1pole_highpass(*s, freq, 1.0 / 44100.0, &mut z, &mut y);
///        // ... do something with the result here.
///    }
///```
pub fn process_1pole_highpass(input: f32, freq: f32, israte: f32, z: &mut f32, y: &mut f32) -> f32 {
    let b = (-std::f32::consts::TAU * freq * israte).exp();
    let a = (1.0 + b) / 2.0;

    let v = a * input - a * *z + b * *y;
    *y = v;
    *z = input;
    v
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OnePoleHPF<F: Flt> {
    israte: F,
    a: F,
    b: F,
    freq: F,
    z: F,
    y: F,
}

impl<F: Flt> OnePoleHPF<F> {
    pub fn new() -> Self {
        Self {
            israte: f(1.0 / 44100.0),
            a: f(0.0),
            b: f(0.0),
            freq: f(1000.0),
            z: f(0.0),
            y: f(0.0),
        }
    }

    pub fn reset(&mut self) {
        self.z = f(0.0);
        self.y = f(0.0);
    }

    #[inline]
    fn recalc(&mut self) {
        self.b = (f::<F>(-1.0) * F::TAU() * self.freq * self.israte).exp();
        self.a = (f::<F>(1.0) + self.b) / f(2.0);
    }

    pub fn set_sample_rate(&mut self, srate: F) {
        self.israte = f::<F>(1.0) / srate;
        self.recalc();
    }

    #[inline]
    pub fn set_freq(&mut self, freq: F) {
        if freq != self.freq {
            self.freq = freq;
            self.recalc();
        }
    }

    #[inline]
    pub fn process(&mut self, input: F) -> F {
        let v = self.a * input - self.a * self.z + self.b * self.y;

        self.y = v;
        self.z = input;

        v
    }
}

// one pole from:
// http://www.willpirkle.com/Downloads/AN-4VirtualAnalogFilters.pdf
// (page 5)
#[inline]
/// Process a very simple one pole 6dB low pass filter in TPT form.
/// Useful in various applications, from usage in a synthesizer to
/// damping stuff in a reverb/delay.
///
/// * `input`  - Input sample
/// * `freq`   - Frequency between 1.0 and 22000.0Hz
/// * `israte` - 1.0 / samplerate
/// * `z`      - The internal one sample buffer of the filter.
///
///```
///    use hexodsp::dsp::helpers::*;
///
///    let samples  = vec![0.0; 44100];
///    let mut z    = 0.0;
///    let mut freq = 1000.0;
///
///    for s in samples.iter() {
///        let s_out =
///            process_1pole_tpt_highpass(*s, freq, 1.0 / 44100.0, &mut z);
///        // ... do something with the result here.
///    }
///```
pub fn process_1pole_tpt_lowpass(input: f32, freq: f32, israte: f32, z: &mut f32) -> f32 {
    let g = (std::f32::consts::PI * freq * israte).tan();
    let a = g / (1.0 + g);

    let v1 = a * (input - *z);
    let v2 = v1 + *z;
    *z = v2 + v1;

    // let (m0, m1) = (0.0, 1.0);
    // (m0 * input + m1 * v2) as f32);
    v2
}

// one pole from:
// http://www.willpirkle.com/Downloads/AN-4VirtualAnalogFilters.pdf
// (page 5)
#[inline]
/// Process a very simple one pole 6dB high pass filter in TPT form.
/// Useful in various applications.
///
/// * `input`  - Input sample
/// * `freq`   - Frequency between 1.0 and 22000.0Hz
/// * `israte` - 1.0 / samplerate
/// * `z`      - The internal one sample buffer of the filter.
///
///```
///    use hexodsp::dsp::helpers::*;
///
///    let samples  = vec![0.0; 44100];
///    let mut z    = 0.0;
///    let mut freq = 1000.0;
///
///    for s in samples.iter() {
///        let s_out =
///            process_1pole_tpt_lowpass(*s, freq, 1.0 / 44100.0, &mut z);
///        // ... do something with the result here.
///    }
///```
pub fn process_1pole_tpt_highpass(input: f32, freq: f32, israte: f32, z: &mut f32) -> f32 {
    let g = (std::f32::consts::PI * freq * israte).tan();
    let a1 = g / (1.0 + g);

    let v1 = a1 * (input - *z);
    let v2 = v1 + *z;
    *z = v2 + v1;

    input - v2
}

/// The internal oversampling factor of [process_hal_chamberlin_svf].
const FILTER_OVERSAMPLE_HAL_CHAMBERLIN: usize = 2;
// Hal Chamberlin's State Variable (12dB/oct) filter
// https://www.earlevel.com/main/2003/03/02/the-digital-state-variable-filter/
// Inspired by SynthV1 by Rui Nuno Capela, under the terms of
// GPLv2 or any later:
/// Process a HAL Chamberlin filter with two delays/state variables that is 12dB.
/// The filter does internal oversampling with very simple decimation to
/// rise the stability for cutoff frequency up to 16kHz.
///
/// * `input` - Input sample.
/// * `freq` - Frequency in Hz. Please keep it inside 0.0 to 16000.0 Hz!
/// otherwise the filter becomes unstable.
/// * `res`  - Resonance from 0.0 to 0.99. Resonance of 1.0 is not recommended,
/// as the filter will then oscillate itself out of control.
/// * `israte` - 1.0 divided by the sampling rate (eg. 1.0 / 44100.0).
/// * `band` - First state variable, containing the band pass result
/// after processing.
/// * `low` - Second state variable, containing the low pass result
/// after processing.
///
/// Returned are the results of the high and notch filter.
///
///```
///    use hexodsp::dsp::helpers::*;
///
///    let samples  = vec![0.0; 44100];
///    let mut band = 0.0;
///    let mut low  = 0.0;
///    let mut freq = 1000.0;
///
///    for s in samples.iter() {
///        let (high, notch) =
///            process_hal_chamberlin_svf(
///                *s, freq, 0.5, 1.0 / 44100.0, &mut band, &mut low);
///        // ... do something with the result here.
///    }
///```
#[inline]
pub fn process_hal_chamberlin_svf(
    input: f32,
    freq: f32,
    res: f32,
    israte: f32,
    band: &mut f32,
    low: &mut f32,
) -> (f32, f32) {
    let q = 1.0 - res;
    let cutoff = 2.0 * (std::f32::consts::PI * freq * 0.5 * israte).sin();

    let mut high = 0.0;
    let mut notch = 0.0;

    for _ in 0..FILTER_OVERSAMPLE_HAL_CHAMBERLIN {
        *low += cutoff * *band;
        high = input - *low - q * *band;
        *band += cutoff * high;
        notch = high + *low;
    }

    //d// println!("q={:4.2} cut={:8.3} freq={:8.1} LP={:8.3} HP={:8.3} BP={:8.3} N={:8.3}",
    //d//     q, cutoff, freq, *low, high, *band, notch);

    (high, notch)
}

/// This function processes a Simper SVF with 12dB. It's a much newer algorithm
/// for filtering and provides easy to calculate multiple outputs.
///
/// * `input` - Input sample.
/// * `freq` - Frequency in Hz.
/// otherwise the filter becomes unstable.
/// * `res`  - Resonance from 0.0 to 0.99. Resonance of 1.0 is not recommended,
/// as the filter will then oscillate itself out of control.
/// * `israte` - 1.0 divided by the sampling rate (eg. 1.0 / 44100.0).
/// * `band` - First state variable, containing the band pass result
/// after processing.
/// * `low` - Second state variable, containing the low pass result
/// after processing.
///
/// This function returns the low pass, band pass and high pass signal.
/// For a notch or peak filter signal, please consult the following example:
///
///```
///    use hexodsp::dsp::helpers::*;
///
///    let samples   = vec![0.0; 44100];
///    let mut ic1eq = 0.0;
///    let mut ic2eq = 0.0;
///    let mut freq  = 1000.0;
///
///    for s in samples.iter() {
///        let (low, band, high) =
///            process_simper_svf(
///                *s, freq, 0.5, 1.0 / 44100.0, &mut ic1eq, &mut ic2eq);
///
///        // You can easily calculate the notch and peak results too:
///        let notch = low + high;
///        let peak  = low - high;
///        // ... do something with the result here.
///    }
///```
// Simper SVF implemented from
// https://cytomic.com/files/dsp/SvfLinearTrapezoidalSin.pdf
// Big thanks go to Andrew Simper @ Cytomic for developing and publishing
// the paper.
#[inline]
pub fn process_simper_svf(
    input: f32,
    freq: f32,
    res: f32,
    israte: f32,
    ic1eq: &mut f32,
    ic2eq: &mut f32,
) -> (f32, f32, f32) {
    // XXX: the 1.989 were tuned by hand, so the resonance is more audible.
    let k = 2f32 - (1.989f32 * res);
    let w = std::f32::consts::PI * freq * israte;

    let s1 = w.sin();
    let s2 = (2.0 * w).sin();
    let nrm = 1.0 / (2.0 + k * s2);

    let g0 = s2 * nrm;
    let g1 = (-2.0 * s1 * s1 - k * s2) * nrm;
    let g2 = (2.0 * s1 * s1) * nrm;

    let t0 = input - *ic2eq;
    let t1 = g0 * t0 + g1 * *ic1eq;
    let t2 = g2 * t0 + g0 * *ic1eq;

    let v1 = t1 + *ic1eq;
    let v2 = t2 + *ic2eq;

    *ic1eq += 2.0 * t1;
    *ic2eq += 2.0 * t2;

    // low   = v2
    // band  = v1
    // high  = input - k * v1 - v2
    // notch = low + high            = input - k * v1
    // peak  = low - high            = 2 * v2 - input + k * v1
    // all   = low + high - k * band = input - 2 * k * v1

    (v2, v1, input - k * v1 - v2)
}

/// This function implements a simple Stilson/Moog low pass filter with 24dB.
/// It provides only a low pass output.
///
/// * `input` - Input sample.
/// * `freq` - Frequency in Hz.
/// otherwise the filter becomes unstable.
/// * `res`  - Resonance from 0.0 to 0.99. Resonance of 1.0 is not recommended,
/// as the filter will then oscillate itself out of control.
/// * `israte` - 1.0 divided by the sampling rate (`1.0 / 44100.0`).
/// * `b0` to `b3` - Internal values used for filtering.
/// * `delay` - A buffer holding other delayed samples.
///
///```
///    use hexodsp::dsp::helpers::*;
///
///    let samples  = vec![0.0; 44100];
///    let mut b0   = 0.0;
///    let mut b1   = 0.0;
///    let mut b2   = 0.0;
///    let mut b3   = 0.0;
///    let mut delay = [0.0; 4];
///    let mut freq = 1000.0;
///
///    for s in samples.iter() {
///        let low =
///            process_stilson_moog(
///                *s, freq, 0.5, 1.0 / 44100.0,
///                &mut b0, &mut b1, &mut b2, &mut b3,
///                &mut delay);
///
///        // ... do something with the result here.
///    }
///```
// Stilson/Moog implementation partly translated from here:
// https://github.com/ddiakopoulos/MoogLadders/blob/master/src/MusicDSPModel.h
// without any copyright as found on musicdsp.org
// (http://www.musicdsp.org/showone.php?id=24).
//
// It's also found on MusicDSP and has probably no proper license anyways.
// See also: https://github.com/ddiakopoulos/MoogLadders
// and https://github.com/rncbc/synthv1/blob/master/src/synthv1_filter.h#L103
// and https://github.com/ddiakopoulos/MoogLadders/blob/master/src/MusicDSPModel.h
#[inline]
pub fn process_stilson_moog(
    input: f32,
    freq: f32,
    res: f32,
    israte: f32,
    b0: &mut f32,
    b1: &mut f32,
    b2: &mut f32,
    b3: &mut f32,
    delay: &mut [f32; 4],
) -> f32 {
    let cutoff = 2.0 * freq * israte;

    let p = cutoff * (1.8 - 0.8 * cutoff);
    let k = 2.0 * (cutoff * std::f32::consts::PI * 0.5).sin() - 1.0;

    let t1 = (1.0 - p) * 1.386249;
    let t2 = 12.0 + t1 * t1;

    let res = res * (t2 + 6.0 * t1) / (t2 - 6.0 * t1);

    let x = input - res * *b3;

    // Four cascaded one-pole filters (bilinear transform)
    *b0 = x * p + delay[0] * p - k * *b0;
    *b1 = *b0 * p + delay[1] * p - k * *b1;
    *b2 = *b1 * p + delay[2] * p - k * *b2;
    *b3 = *b2 * p + delay[3] * p - k * *b3;

    // Clipping band-limited sigmoid
    *b3 -= (*b3 * *b3 * *b3) * 0.166667;

    delay[0] = x;
    delay[1] = *b0;
    delay[2] = *b1;
    delay[3] = *b2;

    *b3
}

// translated from Odin 2 Synthesizer Plugin
// Copyright (C) 2020 TheWaveWarden
// under GPLv3 or any later
#[derive(Debug, Clone, Copy)]
pub struct DCBlockFilter<F: Flt> {
    xm1: F,
    ym1: F,
    r: F,
}

impl<F: Flt> DCBlockFilter<F> {
    pub fn new() -> Self {
        Self { xm1: f(0.0), ym1: f(0.0), r: f(0.995) }
    }

    pub fn reset(&mut self) {
        self.xm1 = f(0.0);
        self.ym1 = f(0.0);
    }

    pub fn set_sample_rate(&mut self, srate: F) {
        self.r = f(0.995);
        if srate > f(90000.0) {
            self.r = f(0.9965);
        } else if srate > f(120000.0) {
            self.r = f(0.997);
        }
    }

    pub fn next(&mut self, input: F) -> F {
        let y = input - self.xm1 + self.r * self.ym1;
        self.xm1 = input;
        self.ym1 = y;
        y as F
    }
}

// PolyBLEP by Tale
// (slightly modified)
// http://www.kvraudio.com/forum/viewtopic.php?t=375517
// from http://www.martin-finke.de/blog/articles/audio-plugins-018-polyblep-oscillator/
//
// default for `pw' should be 1.0, it's the pulse width
// for the square wave.
#[allow(dead_code)]
fn poly_blep_64(t: f64, dt: f64) -> f64 {
    if t < dt {
        let t = t / dt;
        2. * t - (t * t) - 1.
    } else if t > (1.0 - dt) {
        let t = (t - 1.0) / dt;
        (t * t) + 2. * t + 1.
    } else {
        0.
    }
}

fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt {
        let t = t / dt;
        2. * t - (t * t) - 1.
    } else if t > (1.0 - dt) {
        let t = (t - 1.0) / dt;
        (t * t) + 2. * t + 1.
    } else {
        0.
    }
}

/// This is a band-limited oscillator based on the PolyBlep technique.
/// Here is a quick example on how to use it:
///
///```
/// use hexodsp::dsp::helpers::{PolyBlepOscillator, rand_01};
///
/// // Randomize the initial phase to make cancellation on summing less
/// // likely:
/// let mut osc =
///     PolyBlepOscillator::new(rand_01() * 0.25);
///
///
/// let freq   = 440.0; // Hz
/// let israte = 1.0 / 44100.0; // Seconds per Sample
/// let pw     = 0.2;   // Pulse-Width for the next_pulse()
/// let waveform = 0;   // 0 being pulse in this example, 1 being sawtooth
///
/// let mut block_of_samples = [0.0; 128];
/// // in your process function:
/// for output_sample in block_of_samples.iter_mut() {
///    *output_sample =
///        if waveform == 1 {
///             osc.next_saw(freq, israte)
///        } else {
///             osc.next_pulse(freq, israte, pw)
///        }
/// }
///```
#[derive(Debug, Clone)]
pub struct PolyBlepOscillator {
    phase: f32,
    init_phase: f32,
    last_output: f32,
}

impl PolyBlepOscillator {
    /// Create a new instance of [PolyBlepOscillator].
    ///
    /// * `init_phase` - Initial phase of the oscillator.
    /// Range of this parameter is from 0.0 to 1.0. Passing a random
    /// value is advised for preventing phase cancellation when summing multiple
    /// oscillators.
    ///
    ///```
    /// use hexodsp::dsp::helpers::{PolyBlepOscillator, rand_01};
    ///
    /// let mut osc = PolyBlepOscillator::new(rand_01() * 0.25);
    ///```
    pub fn new(init_phase: f32) -> Self {
        Self { phase: 0.0, last_output: 0.0, init_phase }
    }

    /// Reset the internal state of the oscillator as if you just called
    /// [PolyBlepOscillator::new].
    #[inline]
    pub fn reset(&mut self) {
        self.phase = self.init_phase;
        self.last_output = 0.0;
    }

    /// Creates the next sample of a sine wave.
    ///
    /// * `freq` - The frequency in Hz.
    /// * `israte` - The inverse sampling rate, or seconds per sample as in eg. `1.0 / 44100.0`.
    ///```
    /// use hexodsp::dsp::helpers::{PolyBlepOscillator, rand_01};
    ///
    /// let mut osc = PolyBlepOscillator::new(rand_01() * 0.25);
    ///
    /// let freq   = 440.0; // Hz
    /// let israte = 1.0 / 44100.0; // Seconds per Sample
    ///
    /// // ...
    /// let sample = osc.next_sin(freq, israte);
    /// // ...
    ///```
    #[inline]
    pub fn next_sin(&mut self, freq: f32, israte: f32) -> f32 {
        let phase_inc = freq * israte;

        let s = fast_sin(self.phase * 2.0 * std::f32::consts::PI);

        self.phase += phase_inc;
        self.phase = self.phase.fract();

        s as f32
    }

    /// Creates the next sample of a triangle wave. Please note that the
    /// bandlimited waveform needs a few initial samples to swing in.
    ///
    /// * `freq` - The frequency in Hz.
    /// * `israte` - The inverse sampling rate, or seconds per sample as in eg. `1.0 / 44100.0`.
    ///```
    /// use hexodsp::dsp::helpers::{PolyBlepOscillator, rand_01};
    ///
    /// let mut osc = PolyBlepOscillator::new(rand_01() * 0.25);
    ///
    /// let freq   = 440.0; // Hz
    /// let israte = 1.0 / 44100.0; // Seconds per Sample
    ///
    /// // ...
    /// let sample = osc.next_tri(freq, israte);
    /// // ...
    ///```
    #[inline]
    pub fn next_tri(&mut self, freq: f32, israte: f32) -> f32 {
        let phase_inc = freq * israte;

        let mut s = if self.phase < 0.5 { 1.0 } else { -1.0 };

        s += poly_blep(self.phase, phase_inc);
        s -= poly_blep((self.phase + 0.5).fract(), phase_inc);

        // leaky integrator: y[n] = A * x[n] + (1 - A) * y[n-1]
        s = phase_inc * s + (1.0 - phase_inc) * self.last_output;
        self.last_output = s;

        self.phase += phase_inc;
        self.phase = self.phase.fract();

        // the signal is a bit too weak, we need to amplify it
        // or else the volume diff between the different waveforms
        // is too big:
        s * 4.0
    }

    /// Creates the next sample of a sawtooth wave.
    ///
    /// * `freq` - The frequency in Hz.
    /// * `israte` - The inverse sampling rate, or seconds per sample as in eg. `1.0 / 44100.0`.
    ///```
    /// use hexodsp::dsp::helpers::{PolyBlepOscillator, rand_01};
    ///
    /// let mut osc = PolyBlepOscillator::new(rand_01() * 0.25);
    ///
    /// let freq   = 440.0; // Hz
    /// let israte = 1.0 / 44100.0; // Seconds per Sample
    ///
    /// // ...
    /// let sample = osc.next_saw(freq, israte);
    /// // ...
    ///```
    #[inline]
    pub fn next_saw(&mut self, freq: f32, israte: f32) -> f32 {
        let phase_inc = freq * israte;

        let mut s = (2.0 * self.phase) - 1.0;
        s -= poly_blep(self.phase, phase_inc);

        self.phase += phase_inc;
        self.phase = self.phase.fract();

        s
    }

    /// Creates the next sample of a pulse wave.
    /// In comparison to [PolyBlepOscillator::next_pulse_no_dc] this
    /// version is DC compensated, so that you may add multiple different
    /// pulse oscillators for a unison effect without too big DC issues.
    ///
    /// * `freq` - The frequency in Hz.
    /// * `israte` - The inverse sampling rate, or seconds per sample as in eg. `1.0 / 44100.0`.
    /// * `pw` - The pulse width. Use the value 0.0 for a square wave.
    ///```
    /// use hexodsp::dsp::helpers::{PolyBlepOscillator, rand_01};
    ///
    /// let mut osc = PolyBlepOscillator::new(rand_01() * 0.25);
    ///
    /// let freq   = 440.0; // Hz
    /// let israte = 1.0 / 44100.0; // Seconds per Sample
    /// let pw     = 0.0; // 0.0 is a square wave.
    ///
    /// // ...
    /// let sample = osc.next_pulse(freq, israte, pw);
    /// // ...
    ///```
    #[inline]
    pub fn next_pulse(&mut self, freq: f32, israte: f32, pw: f32) -> f32 {
        let phase_inc = freq * israte;

        let pw = (0.1 * pw) + ((1.0 - pw) * 0.5); // some scaling
        let dc_compensation = (0.5 - pw) * 2.0;

        let mut s = if self.phase < pw { 1.0 } else { -1.0 };

        s += poly_blep(self.phase, phase_inc);
        s -= poly_blep((self.phase + (1.0 - pw)).fract(), phase_inc);

        s += dc_compensation;

        self.phase += phase_inc;
        self.phase = self.phase.fract();

        s
    }

    /// Creates the next sample of a pulse wave.
    /// In comparison to [PolyBlepOscillator::next_pulse] this
    /// version is not DC compensated. So be careful when adding multiple
    /// of this or generally using it in an audio context.
    ///
    /// * `freq` - The frequency in Hz.
    /// * `israte` - The inverse sampling rate, or seconds per sample as in eg. `1.0 / 44100.0`.
    /// * `pw` - The pulse width. Use the value 0.0 for a square wave.
    ///```
    /// use hexodsp::dsp::helpers::{PolyBlepOscillator, rand_01};
    ///
    /// let mut osc = PolyBlepOscillator::new(rand_01() * 0.25);
    ///
    /// let freq   = 440.0; // Hz
    /// let israte = 1.0 / 44100.0; // Seconds per Sample
    /// let pw     = 0.0; // 0.0 is a square wave.
    ///
    /// // ...
    /// let sample = osc.next_pulse_no_dc(freq, israte, pw);
    /// // ...
    ///```
    #[inline]
    pub fn next_pulse_no_dc(&mut self, freq: f32, israte: f32, pw: f32) -> f32 {
        let phase_inc = freq * israte;

        let pw = (0.1 * pw) + ((1.0 - pw) * 0.5); // some scaling

        let mut s = if self.phase < pw { 1.0 } else { -1.0 };

        s += poly_blep(self.phase, phase_inc);
        s -= poly_blep((self.phase + (1.0 - pw)).fract(), phase_inc);

        self.phase += phase_inc;
        self.phase = self.phase.fract();

        s
    }
}

// This oscillator is based on the work "VECTOR PHASESHAPING SYNTHESIS"
// by: Jari Kleimola*, Victor Lazzarini†, Joseph Timoney†, Vesa Välimäki*
// *Aalto University School of Electrical Engineering Espoo, Finland;
// †National University of Ireland, Maynooth Ireland
//
// See also this PDF: http://recherche.ircam.fr/pub/dafx11/Papers/55_e.pdf
/// Vector Phase Shaping Oscillator.
/// The parameters `d` and `v` control the shape of the sinus
/// wave. This leads to interesting modulation properties of those
/// control values.
///
///```
/// use hexodsp::dsp::helpers::{VPSOscillator, rand_01};
///
/// // Randomize the initial phase to make cancellation on summing less
/// // likely:
/// let mut osc =
///     VPSOscillator::new(rand_01() * 0.25);
///
///
/// let freq   = 440.0; // Hz
/// let israte = 1.0 / 44100.0; // Seconds per Sample
/// let d      = 0.5;   // Range: 0.0 to 1.0
/// let v      = 0.75;  // Range: 0.0 to 1.0
///
/// let mut block_of_samples = [0.0; 128];
/// // in your process function:
/// for output_sample in block_of_samples.iter_mut() {
///     // It is advised to limit the `v` value, because with certain
///     // `d` values the combination creates just a DC offset.
///     let v = VPSOscillator::limit_v(d, v);
///     *output_sample = osc.next(freq, israte, d, v);
/// }
///```
///
/// It can be beneficial to apply distortion and oversampling.
/// Especially oversampling can be important for some `d` and `v`
/// combinations, even without distortion.
///
///```
/// use hexodsp::dsp::helpers::{VPSOscillator, rand_01, apply_distortion};
/// use hexodsp::dsp::biquad::Oversampling;
///
/// let mut osc = VPSOscillator::new(rand_01() * 0.25);
/// let mut ovr : Oversampling<4> = Oversampling::new();
///
/// let freq   = 440.0; // Hz
/// let israte = 1.0 / 44100.0; // Seconds per Sample
/// let d      = 0.5;   // Range: 0.0 to 1.0
/// let v      = 0.75;  // Range: 0.0 to 1.0
///
/// let mut block_of_samples = [0.0; 128];
/// // in your process function:
/// for output_sample in block_of_samples.iter_mut() {
///     // It is advised to limit the `v` value, because with certain
///     // `d` values the combination creates just a DC offset.
///     let v = VPSOscillator::limit_v(d, v);
///
///     let overbuf = ovr.resample_buffer();
///     for b in overbuf {
///         *b = apply_distortion(osc.next(freq, israte, d, v), 0.9,  1);
///     }
///     *output_sample = ovr.downsample();
/// }
///```
#[derive(Debug, Clone)]
pub struct VPSOscillator {
    phase: f32,
    init_phase: f32,
}

impl VPSOscillator {
    /// Create a new instance of [VPSOscillator].
    ///
    /// * `init_phase` - The initial phase of the oscillator.
    pub fn new(init_phase: f32) -> Self {
        Self { phase: 0.0, init_phase }
    }

    /// Reset the phase of the oscillator to the initial phase.
    #[inline]
    pub fn reset(&mut self) {
        self.phase = self.init_phase;
    }

    #[inline]
    fn s(p: f32) -> f32 {
        -(std::f32::consts::TAU * p).cos()
    }

    #[inline]
    fn phi_vps(x: f32, v: f32, d: f32) -> f32 {
        if x < d {
            (v * x) / d
        } else {
            v + ((1.0 - v) * (x - d)) / (1.0 - d)
        }
    }

    /// This rather complicated function blends out some
    /// combinations of 'd' and 'v' that just lead to a constant DC
    /// offset. Which is not very useful in an audio oscillator
    /// context.
    ///
    /// Call this before passing `v` to [VPSOscillator::next].
    #[inline]
    pub fn limit_v(d: f32, v: f32) -> f32 {
        let delta = 0.5 - (d - 0.5).abs();
        if delta < 0.05 {
            let x = (0.05 - delta) * 19.99;
            if d < 0.5 {
                let mm = x * 0.5;
                let max = 1.0 - mm;
                if v > max && v < 1.0 {
                    max
                } else if v >= 1.0 && v < (1.0 + mm) {
                    1.0 + mm
                } else {
                    v
                }
            } else {
                if v < 1.0 {
                    v.clamp(x * 0.5, 1.0)
                } else {
                    v
                }
            }
        } else {
            v
        }
    }

    /// Creates the next sample of this oscillator.
    ///
    /// * `freq` - The frequency in Hz.
    /// * `israte` - The inverse sampling rate, or seconds per sample as in eg. `1.0 / 44100.0`.
    /// * `d` - The phase distortion parameter `d` which must be in the range `0.0` to `1.0`.
    /// * `v` - The phase distortion parameter `v` which must be in the range `0.0` to `1.0`.
    ///
    /// It is advised to limit the `v` using the [VPSOscillator::limit_v] function
    /// before calling this function. To prevent DC offsets when modulating the parameters.
    pub fn next(&mut self, freq: f32, israte: f32, d: f32, v: f32) -> f32 {
        let s = Self::s(Self::phi_vps(self.phase, v, d));

        self.phase += freq * israte;
        self.phase = self.phase.fract();

        s
    }
}

// Adapted from https://github.com/ValleyAudio/ValleyRackFree/blob/v1.0/src/Common/DSP/LFO.hpp
//
// ValleyRackFree Copyright (C) 2020, Valley Audio Soft, Dale Johnson
// Adapted under the GPL-3.0-or-later License.
/// An LFO with a variable reverse point, which can go from reverse Saw, to Tri
/// and to Saw, depending on the reverse point.
#[derive(Debug, Clone, Copy)]
pub struct TriSawLFO<F: Flt> {
    /// The (inverse) sample rate. Eg. 1.0 / 44100.0.
    israte: F,
    /// The current oscillator phase.
    phase: F,
    /// The point from where the falling edge will be used.
    rev: F,
    /// The frequency.
    freq: F,
    /// Precomputed rise/fall rate of the LFO.
    rise_r: F,
    fall_r: F,
    /// Initial phase offset.
    init_phase: F,
}

impl<F: Flt> TriSawLFO<F> {
    pub fn new() -> Self {
        let mut this = Self {
            israte: f(1.0 / 44100.0),
            phase: f(0.0),
            rev: f(0.5),
            freq: f(1.0),
            fall_r: f(0.0),
            rise_r: f(0.0),
            init_phase: f(0.0),
        };
        this.recalc();
        this
    }

    pub fn set_phase_offs(&mut self, phase: F) {
        self.init_phase = phase;
        self.phase = phase;
    }

    #[inline]
    fn recalc(&mut self) {
        self.rev = fclampc(self.rev, 0.0001, 0.999);
        self.rise_r = f::<F>(1.0) / self.rev;
        self.fall_r = f::<F>(-1.0) / (f::<F>(1.0) - self.rev);
    }

    pub fn set_sample_rate(&mut self, srate: F) {
        self.israte = f::<F>(1.0) / (srate as F);
        self.recalc();
    }

    pub fn reset(&mut self) {
        self.phase = self.init_phase;
        self.rev = f(0.5);
    }

    #[inline]
    pub fn set(&mut self, freq: F, rev: F) {
        self.freq = freq as F;
        self.rev = rev as F;
        self.recalc();
    }

    #[inline]
    pub fn next_unipolar(&mut self) -> F {
        if self.phase >= f(1.0) {
            self.phase = self.phase - f(1.0);
        }

        let s = if self.phase < self.rev {
            self.phase * self.rise_r
        } else {
            self.phase * self.fall_r - self.fall_r
        };

        self.phase = self.phase + self.freq * self.israte;

        s
    }

    #[inline]
    pub fn next_bipolar(&mut self) -> F {
        (self.next_unipolar() * f(2.0)) - f(1.0)
    }
}

#[derive(Debug, Clone)]
pub struct Quantizer {
    old_mask: i64,
    lkup_tbl: [(f32, f32); 24],
    last_key: f32,
}

impl Quantizer {
    pub fn new() -> Self {
        Self { old_mask: 0xFFFF_FFFF, lkup_tbl: [(0.0, 0.0); 24], last_key: 0.0 }
    }

    #[inline]
    pub fn set_keys(&mut self, keys_mask: i64) {
        if keys_mask == self.old_mask {
            return;
        }
        self.old_mask = keys_mask;

        self.setup_lookup_table();
    }

    #[inline]
    fn setup_lookup_table(&mut self) {
        let mask = self.old_mask;
        let any_enabled = mask > 0x0;

        for i in 0..24 {
            let mut min_d_note_idx = 0;
            let mut min_dist = 1000000000;

            for note in -12..=24 {
                let dist = ((i + 1_i64) / 2 - note).abs();
                let note_idx = note.rem_euclid(12);

                // XXX: We add 9 here for the mask lookup,
                // to shift the keyboard, which starts at C!
                // And first bit in the mask is the C note. 10th is the A note.
                if any_enabled && (mask & (0x1 << ((note_idx + 9) % 12))) == 0x0 {
                    continue;
                }

                //d// println!("I={:3} NOTE={:3} (IDX={:3} => bitset {}) DIST={:3}",
                //d//     i, note, note_idx,
                //d//     if (mask & (0x1 << ((note_idx + 9) % 12))) > 0x0 { 1 } else { 0 },
                //d//     dist);

                if dist < min_dist {
                    min_d_note_idx = note;
                    min_dist = dist;
                } else {
                    break;
                }
            }

            self.lkup_tbl[i as usize] = (
                (min_d_note_idx + 9).rem_euclid(12) as f32 * (0.1 / 12.0),
                min_d_note_idx.rem_euclid(12) as f32 * (0.1 / 12.0)
                    + (if min_d_note_idx < 0 {
                        -0.1
                    } else if min_d_note_idx > 11 {
                        0.1
                    } else {
                        0.0
                    }),
            );
        }
        //d// println!("TBL: {:?}", self.lkup_tbl);
    }

    #[inline]
    pub fn last_key_pitch(&self) -> f32 {
        self.last_key
    }

    #[inline]
    pub fn process(&mut self, inp: f32) -> f32 {
        let note_num = (inp * 240.0).round() as i64;
        let octave = note_num.div_euclid(24);
        let note_idx = note_num - octave * 24;

        //        println!(
        //            "INP {:7.4} => octave={:3}, note_idx={:3} note_num={:3} inp={:9.6}",
        //            inp, octave, note_idx, note_num, inp * 240.0);
        //d// println!("TBL: {:?}", self.lkup_tbl);

        let (ui_key_pitch, note_pitch) = self.lkup_tbl[note_idx as usize % 24];
        self.last_key = ui_key_pitch;
        note_pitch + octave as f32 * 0.1
    }
}

#[derive(Debug, Clone)]
pub struct CtrlPitchQuantizer {
    /// All keys, containing the min/max octave!
    keys: Vec<f32>,
    /// Only the used keys with their pitches from the UI
    used_keys: [f32; 12],
    /// A value combination of the arguments to `update_keys`.
    input_params: u64,
    /// The number of used keys from the mask.
    mask_key_count: u16,
    /// The last key for the pitch that was returned by `process`.
    last_key: u8,
}

const QUANT_TUNE_TO_A4: f32 = (9.0 / 12.0) * 0.1;

impl CtrlPitchQuantizer {
    pub fn new() -> Self {
        Self {
            keys: vec![0.0; 12 * 10],
            used_keys: [0.0; 12],
            mask_key_count: 0,
            input_params: 0xFFFFFFFFFF,
            last_key: 0,
        }
    }

    #[inline]
    pub fn last_key_pitch(&self) -> f32 {
        self.used_keys[self.last_key as usize % (self.mask_key_count as usize)] + QUANT_TUNE_TO_A4
    }

    #[inline]
    pub fn update_keys(&mut self, mut mask: i64, min_oct: i64, max_oct: i64) {
        let inp_params = (mask as u64) | ((min_oct as u64) << 12) | ((max_oct as u64) << 20);

        if self.input_params == inp_params {
            return;
        }

        self.input_params = inp_params;

        let mut mask_count = 0;

        // set all keys, if none are set!
        if mask == 0x0 {
            mask = 0xFFFF;
        }

        for i in 0..12 {
            if mask & (0x1 << i) > 0 {
                self.used_keys[mask_count] = (i as f32 / 12.0) * 0.1 - QUANT_TUNE_TO_A4;
                mask_count += 1;
            }
        }

        self.keys.clear();

        let min_oct = min_oct as usize;
        for o in 0..min_oct {
            let o = min_oct - o;

            for i in 0..mask_count {
                self.keys.push(self.used_keys[i] - (o as f32) * 0.1);
            }
        }

        for i in 0..mask_count {
            self.keys.push(self.used_keys[i]);
        }

        let max_oct = max_oct as usize;
        for o in 1..=max_oct {
            for i in 0..mask_count {
                self.keys.push(self.used_keys[i] + (o as f32) * 0.1);
            }
        }

        self.mask_key_count = mask_count as u16;
    }

    #[inline]
    pub fn signal_to_pitch(&mut self, inp: f32) -> f32 {
        let len = self.keys.len();
        let key = (inp.clamp(0.0, 0.9999) * (len as f32)).floor();
        let key = key as usize % len;
        self.last_key = key as u8;
        self.keys[key]
    }
}

#[macro_export]
macro_rules! fa_distort {
    ($formatter: expr, $v: expr, $denorm_v: expr) => {{
        let s = match ($v.round() as usize) {
            0 => "Off",
            1 => "TanH",
            2 => "B.D.Jong",
            3 => "Fold",
            _ => "?",
        };
        write!($formatter, "{}", s)
    }};
}

#[inline]
pub fn apply_distortion(s: f32, damt: f32, dist_type: u8) -> f32 {
    match dist_type {
        1 => (damt.clamp(0.01, 1.0) * 100.0 * s).tanh(),
        2 => f_distort(1.0, damt * damt * damt * 1000.0, s),
        3 => {
            let damt = damt.clamp(0.0, 0.99);
            let damt = 1.0 - damt * damt;
            f_fold_distort(1.0, damt, s) * (1.0 / damt)
        }
        _ => s,
    }
}

//pub struct UnisonBlep {
//    oscs: Vec<PolyBlepOscillator>,
////    dc_block: crate::filter::DCBlockFilter,
//}
//
//impl UnisonBlep {
//    pub fn new(max_unison: usize) -> Self {
//        let mut oscs = vec![];
//        let mut rng = RandGen::new();
//
//        let dis_init_phase = 0.05;
//        for i in 0..(max_unison + 1) {
//            // randomize phases so we fatten the unison, get
//            // less DC and not an amplified signal until the
//            // detune desyncs the waves.
//            // But no random phase for first, so we reduce the click
//            let init_phase =
//                if i == 0 { 0.0 } else { rng.next_open01() };
//            oscs.push(PolyBlepOscillator::new(init_phase));
//        }
//
//        Self {
//            oscs,
////            dc_block: crate::filter::DCBlockFilter::new(),
//        }
//    }
//
//    pub fn set_sample_rate(&mut self, srate: f32) {
////        self.dc_block.set_sample_rate(srate);
//        for o in self.oscs.iter_mut() {
//            o.set_sample_rate(srate);
//        }
//    }
//
//    pub fn reset(&mut self) {
////        self.dc_block.reset();
//        for o in self.oscs.iter_mut() {
//            o.reset();
//        }
//    }
//
//    pub fn next<P: OscillatorInputParams>(&mut self, params: &P) -> f32 {
//        let unison =
//            (params.unison().floor() as usize)
//            .min(self.oscs.len() - 1);
//        let detune = params.detune() as f64;
//
//        let mix = (1.0 / ((unison + 1) as f32)).sqrt();
//
//        let mut s = mix * self.oscs[0].next(params, 0.0);
//
//        for u in 0..unison {
//            let detune_factor =
//                detune * (((u / 2) + 1) as f64
//                          * if (u % 2) == 0 { 1.0 } else { -1.0 });
//            s += mix * self.oscs[u + 1].next(params, detune_factor * 0.01);
//        }
//
////        self.dc_block.next(s)
//        s
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_range2p_exp() {
        let a = p2range_exp(0.5, 1.0, 100.0);
        let x = range2p_exp(a, 1.0, 100.0);

        assert!((x - 0.5).abs() < std::f32::EPSILON);
    }

    #[test]
    fn check_range2p() {
        let a = p2range(0.5, 1.0, 100.0);
        let x = range2p(a, 1.0, 100.0);

        assert!((x - 0.5).abs() < std::f32::EPSILON);
    }
}
