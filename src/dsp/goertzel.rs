// Copyright (c) 2022 theloni-monk <theo.acooper@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.
const PI: f32 = std::f32::consts::PI;

#[derive(Debug, Clone, Default)]
pub struct GoertzelParams {}
#[derive(Debug, Clone, Default)]
pub struct Goertzel {
    pub target_freq: f32,
    pub coeff: f32,

    q0: f32,
    q1: f32,
    q2: f32,
}
const DEFAULT_BUFFSIZE: usize = 100;
// Calculates an individual term of the Discrete Fourier Series
// implementation with notes taken from https://www.embedded.com/the-goertzel-algorithm/
impl Goertzel {
    pub fn new() -> Self {
        let mut s: Goertzel = Default::default();
        s.setCoeff(880.0, DEFAULT_BUFFSIZE, 44100.0);
        (s.q0, s.q1, s.q2) = (0.0, 0.0, 0.0);
        s
    }

    #[inline]
    pub fn new_with(tfreq: f32, buffsize: usize, srate: f32) -> Self {
        let mut s = Self::new();
        s.setCoeff(tfreq, buffsize, srate);
        s
    }

    pub fn reset(&mut self) {
        self.q0 = 0.0;
        self.q1 = 0.0;
        self.q2 = 0.0;
    }

    pub fn setCoeff(&mut self, tfreq: f32, buffsize: usize, srate: f32) {
        self.target_freq = tfreq;
        let k = (0.5 * ((buffsize as f32 * self.target_freq) / srate as f32)).floor() as f32;
        let w = (2.0 * PI * k / buffsize as f32) as f32;
        let c = f32::cos(w);
        self.coeff = 2.0 * c;
    }

    #[inline]
    pub fn tick(&mut self, input: f32) -> f32 {
        let x0 = input;

        self.q0 = self.coeff * self.q1 - self.q2 + x0;
        self.q2 = self.q1;
        self.q1 = self.q0;

        let mag_squared =
            (self.q1.powi(2) + (self.q2.powi(2)) - (self.q1 * self.q2 * self.coeff)) as f32;

        f32::sqrt(mag_squared) / 100.0
    }
}
