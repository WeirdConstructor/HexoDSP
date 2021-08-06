// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

// This file contains a reverb implementation that is based
// on Jon Dattorro's 1997 reverb algorithm. It's also largely
// based on the C++ implementation from ValleyAudio / ValleyRackFree
//
// ValleyRackFree Copyright (C) 2020, Valley Audio Soft, Dale Johnson
// Adapted under the GPL-3.0-or-later License.
//
// See also: https://github.com/ValleyAudio/ValleyRackFree/blob/v1.0/src/Plateau/Dattorro.cpp
//      and: https://github.com/ValleyAudio/ValleyRackFree/blob/v1.0/src/Plateau/Dattorro.hpp
//
// And: https://ccrma.stanford.edu/~dattorro/music.html
// And: https://ccrma.stanford.edu/~dattorro/EffectDesignPart1.pdf

const DAT_SAMPLE_RATE    : f32 = 29761.0;
const DAT_SAMPLES_PER_MS : f32 = DAT_SAMPLE_RATE / 1000.0;

const DAT_INPUT_APF_TIMES_MS : [f32; 4] = [
    141.0 / DAT_SAMPLES_PER_MS,
    107.0 / DAT_SAMPLES_PER_MS,
    379.0 / DAT_SAMPLES_PER_MS,
    277.0 / DAT_SAMPLES_PER_MS,
];

const DAT_LEFT_APF1_TIME_MS  : f32 = 672.0  / DAT_SAMPLES_PER_MS;
const DAT_LEFT_APF2_TIME_MS  : f32 = 1800.0 / DAT_SAMPLES_PER_MS;

const DAT_RIGHT_APF1_TIME_MS : f32 = 908.0  / DAT_SAMPLES_PER_MS;
const DAT_RIGHT_APF2_TIME_MS : f32 = 2656.0 / DAT_SAMPLES_PER_MS;

//    const long _kLeftApf1Time = 672;
//    const long _kLeftDelay1Time = 4453;
//    const long _kLeftApf2Time = 1800;
//    const long _kLeftDelay2Time = 3720;
//
//    const long _kRightApf1Time = 908;
//    const long _kRightDelay1Time = 4217;
//    const long _kRightApf2Time = 2656;
//    const long _kRightDelay2Time = 3163;

const DAT_INPUT_DIFFUSION1 : f32 = 0.75;
const DAT_INPUT_DIFFUSION2 : f32 = 0.625;
const DAT_PLATE_DIFFUSION1 : f32 = 0.7;
const DAT_PLATE_DIFFUSION2 : f32 = 0.5;

use crate::dsp::helpers::{
    AllPass,
    TriSawLFO,
    OnePoleLPF,
    OnePoleHPF,
    DelayBuffer,
    DCBlockFilter
};

pub struct DattorroReverb {
    last_scale: f32,

    inp_dc_block:   [DCBlockFilter; 2],
    out_dc_block:   [DCBlockFilter; 2],

    lfos: [TriSawLFO; 4],

    input_hpf: OnePoleHPF,
    input_lpf: OnePoleLPF,

    pre_delay:  DelayBuffer,
    input_apfs: [(AllPass, f32); 4],

    apf1:   [(AllPass, f32); 2],
    hpf:    [OnePoleHPF; 2],
    lpf:    [OnePoleLPF; 2],
    apf2:   [AllPass; 2],
    delay1: [DelayBuffer; 2],
    delay2: [DelayBuffer; 2],

    time_scale_factor: f32,
}

pub trait DattorroReverbParams {
    /// Time for the pre-delay of the reverb.
    fn pre_delay_time_s(&self) -> f32;
}

impl DattorroReverb {
    pub fn new() -> Self {
        let mut this = Self {
            last_scale: 1.0,

            inp_dc_block:   [DCBlockFilter::new(); 2],
            out_dc_block:   [DCBlockFilter::new(); 2],

            lfos: [TriSawLFO::new(); 4],

            input_hpf: OnePoleHPF::new(),
            input_lpf: OnePoleLPF::new(),

            pre_delay:  DelayBuffer::new(),
            input_apfs: [(AllPass::new(), 0.0); 4],

            apf1:   [(AllPass::new(), 0.0); 2],
            hpf:    [OnePoleHPF::new(); 2],
            lpf:    [OnePoleLPF::new(); 2],
            apf2:   [AllPass::new(); 2],
            delay1: [DelayBuffer::new(); 2],
            delay2: [DelayBuffer::new(); 2],
            time_scale_factor: 1.0,
        };

        this.reset();

        this
    }

    pub fn reset(&mut self) {
        self.input_lpf.set_freq(22000.0);
        self.input_hpf.set_freq(0.0);

        self.input_apfs[0].1 = DAT_INPUT_APF_TIMES_MS[0];
        self.input_apfs[1].1 = DAT_INPUT_APF_TIMES_MS[1];
        self.input_apfs[2].1 = DAT_INPUT_APF_TIMES_MS[2];
        self.input_apfs[3].1 = DAT_INPUT_APF_TIMES_MS[3];

        self.set_time_scale(1.0);
    }

    #[inline]
    pub fn set_time_scale(&mut self, scale: f32) {
        if (self.last_scale - scale).abs() > std::f32::consts::EPSILON {
            let scale = scale.max(0.0001);
            self.last_scale = scale;

            self.apf1[0].1 = DAT_LEFT_APF1_TIME_MS  * scale;
            self.apf1[1].1 = DAT_RIGHT_APF1_TIME_MS * scale;
            self.apf2[0].1 = DAT_LEFT_APF2_TIME_MS  * scale;
            self.apf2[1].1 = DAT_RIGHT_APF2_TIME_MS * scale;
        }
    }

    pub fn set_sample_rate(&mut self, srate: f32) {
        self.time_scale_factor = srate / DAT_SAMPLE_RATE;
    }

    pub fn process(&mut self, params: &mut dyn DattorroReverbParams, input: f32) -> (f32, f32) {
    }
}
