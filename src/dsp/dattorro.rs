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

use crate::dsp::helpers::crossfade;

const DAT_SAMPLE_RATE: f64 = 29761.0;
const DAT_SAMPLES_PER_MS: f64 = DAT_SAMPLE_RATE / 1000.0;

const DAT_INPUT_APF_TIMES_MS: [f64; 4] = [
    141.0 / DAT_SAMPLES_PER_MS,
    107.0 / DAT_SAMPLES_PER_MS,
    379.0 / DAT_SAMPLES_PER_MS,
    277.0 / DAT_SAMPLES_PER_MS,
];

const DAT_LEFT_APF1_TIME_MS: f64 = 672.0 / DAT_SAMPLES_PER_MS;
const DAT_LEFT_APF2_TIME_MS: f64 = 1800.0 / DAT_SAMPLES_PER_MS;

const DAT_RIGHT_APF1_TIME_MS: f64 = 908.0 / DAT_SAMPLES_PER_MS;
const DAT_RIGHT_APF2_TIME_MS: f64 = 2656.0 / DAT_SAMPLES_PER_MS;

const DAT_LEFT_DELAY1_TIME_MS: f64 = 4453.0 / DAT_SAMPLES_PER_MS;
const DAT_LEFT_DELAY2_TIME_MS: f64 = 3720.0 / DAT_SAMPLES_PER_MS;

const DAT_RIGHT_DELAY1_TIME_MS: f64 = 4217.0 / DAT_SAMPLES_PER_MS;
const DAT_RIGHT_DELAY2_TIME_MS: f64 = 3163.0 / DAT_SAMPLES_PER_MS;

const DAT_LEFT_TAPS_TIME_MS: [f64; 7] = [
    266.0 / DAT_SAMPLES_PER_MS,
    2974.0 / DAT_SAMPLES_PER_MS,
    1913.0 / DAT_SAMPLES_PER_MS,
    1996.0 / DAT_SAMPLES_PER_MS,
    1990.0 / DAT_SAMPLES_PER_MS,
    187.0 / DAT_SAMPLES_PER_MS,
    1066.0 / DAT_SAMPLES_PER_MS,
];

const DAT_RIGHT_TAPS_TIME_MS: [f64; 7] = [
    353.0 / DAT_SAMPLES_PER_MS,
    3627.0 / DAT_SAMPLES_PER_MS,
    1228.0 / DAT_SAMPLES_PER_MS,
    2673.0 / DAT_SAMPLES_PER_MS,
    2111.0 / DAT_SAMPLES_PER_MS,
    335.0 / DAT_SAMPLES_PER_MS,
    121.0 / DAT_SAMPLES_PER_MS,
];

const DAT_LFO_FREQS_HZ: [f64; 4] = [0.1, 0.15, 0.12, 0.18];

const DAT_INPUT_DIFFUSION1: f64 = 0.75;
const DAT_INPUT_DIFFUSION2: f64 = 0.625;
const DAT_PLATE_DIFFUSION1: f64 = 0.7;
const DAT_PLATE_DIFFUSION2: f64 = 0.5;

const DAT_LFO_EXCURSION_MS: f64 = 16.0 / DAT_SAMPLES_PER_MS;
const DAT_LFO_EXCURSION_MOD_MAX: f64 = 16.0;

use crate::dsp::helpers::{AllPass, DCBlockFilter, DelayBuffer, OnePoleHPF, OnePoleLPF, TriSawLFO};

#[derive(Debug, Clone)]
pub struct DattorroReverb {
    last_scale: f64,

    inp_dc_block: [DCBlockFilter<f64>; 2],
    out_dc_block: [DCBlockFilter<f64>; 2],

    lfos: [TriSawLFO<f64>; 4],

    input_hpf: OnePoleHPF<f64>,
    input_lpf: OnePoleLPF<f64>,

    pre_delay: DelayBuffer<f64>,
    input_apfs: [(AllPass<f64>, f64, f64); 4],

    apf1: [(AllPass<f64>, f64, f64); 2],
    hpf: [OnePoleHPF<f64>; 2],
    lpf: [OnePoleLPF<f64>; 2],
    apf2: [(AllPass<f64>, f64, f64); 2],
    delay1: [(DelayBuffer<f64>, f64); 2],
    delay2: [(DelayBuffer<f64>, f64); 2],

    left_sum: f64,
    right_sum: f64,

    dbg_count: usize,
}

pub trait DattorroReverbParams {
    /// Time for the pre-delay of the reverb. Any sensible `ms` that fits
    /// into a delay buffer of 5 seconds.
    fn pre_delay_time_ms(&self) -> f64;
    /// The size of the reverb, values go from 0.0 to 1.0.
    fn time_scale(&self) -> f64;
    /// High-pass input filter cutoff freq in Hz, range: 0.0 to 22000.0
    fn input_high_cutoff_hz(&self) -> f64;
    /// Low-pass input filter cutoff freq in Hz, range: 0.0 to 22000.0
    fn input_low_cutoff_hz(&self) -> f64;
    /// High-pass reverb filter cutoff freq in Hz, range: 0.0 to 22000.0
    fn reverb_high_cutoff_hz(&self) -> f64;
    /// Low-pass reverb filter cutoff freq in Hz, range: 0.0 to 22000.0
    fn reverb_low_cutoff_hz(&self) -> f64;
    /// Modulation speed factor, range: 0.0 to 1.0
    fn mod_speed(&self) -> f64;
    /// Modulation depth from the LFOs, range: 0.0 to 1.0
    fn mod_depth(&self) -> f64;
    /// Modulation shape (from saw to tri to saw), range: 0.0 to 1.0
    fn mod_shape(&self) -> f64;
    /// The mix between output from the pre-delay and the input diffusion.
    /// range: 0.0 to 1.0. Default should be 1.0
    fn input_diffusion_mix(&self) -> f64;
    /// The amount of plate diffusion going on, range: 0.0 to 1.0
    fn diffusion(&self) -> f64;
    /// Internal tank decay time, range: 0.0 to 1.0
    fn decay(&self) -> f64;
}

impl DattorroReverb {
    pub fn new() -> Self {
        let mut this = Self {
            last_scale: 1.0,

            inp_dc_block: [DCBlockFilter::new(); 2],
            out_dc_block: [DCBlockFilter::new(); 2],

            lfos: [TriSawLFO::new(); 4],

            input_hpf: OnePoleHPF::new(),
            input_lpf: OnePoleLPF::new(),

            pre_delay: DelayBuffer::new(),
            input_apfs: Default::default(),

            apf1: Default::default(),
            hpf: [OnePoleHPF::new(); 2],
            lpf: [OnePoleLPF::new(); 2],
            apf2: Default::default(),
            delay1: Default::default(),
            delay2: Default::default(),

            left_sum: 0.0,
            right_sum: 0.0,

            dbg_count: 0,
        };

        this.reset();

        this
    }

    pub fn reset(&mut self) {
        self.input_lpf.reset();
        self.input_hpf.reset();

        self.input_lpf.set_freq(22000.0);
        self.input_hpf.set_freq(0.0);

        self.input_apfs[0] = (AllPass::new(), DAT_INPUT_APF_TIMES_MS[0], DAT_INPUT_DIFFUSION1);
        self.input_apfs[1] = (AllPass::new(), DAT_INPUT_APF_TIMES_MS[1], DAT_INPUT_DIFFUSION1);
        self.input_apfs[2] = (AllPass::new(), DAT_INPUT_APF_TIMES_MS[2], DAT_INPUT_DIFFUSION2);
        self.input_apfs[3] = (AllPass::new(), DAT_INPUT_APF_TIMES_MS[3], DAT_INPUT_DIFFUSION2);

        self.apf1[0] = (AllPass::new(), DAT_LEFT_APF1_TIME_MS, -DAT_PLATE_DIFFUSION1);
        self.apf1[1] = (AllPass::new(), DAT_RIGHT_APF1_TIME_MS, -DAT_PLATE_DIFFUSION1);
        self.apf2[0] = (AllPass::new(), DAT_LEFT_APF2_TIME_MS, -DAT_PLATE_DIFFUSION2);
        self.apf2[1] = (AllPass::new(), DAT_RIGHT_APF2_TIME_MS, -DAT_PLATE_DIFFUSION2);

        self.delay1[0] = (DelayBuffer::new(), DAT_LEFT_DELAY1_TIME_MS);
        self.delay1[1] = (DelayBuffer::new(), DAT_RIGHT_DELAY1_TIME_MS);
        self.delay2[0] = (DelayBuffer::new(), DAT_LEFT_DELAY2_TIME_MS);
        self.delay2[1] = (DelayBuffer::new(), DAT_RIGHT_DELAY2_TIME_MS);

        self.lpf[0].reset();
        self.lpf[1].reset();
        self.lpf[0].set_freq(10000.0);
        self.lpf[1].set_freq(10000.0);

        self.hpf[0].reset();
        self.hpf[1].reset();
        self.hpf[0].set_freq(0.0);
        self.hpf[1].set_freq(0.0);

        self.lfos[0].set(DAT_LFO_FREQS_HZ[0], 0.5);
        self.lfos[0].set_phase_offs(0.0);
        self.lfos[0].reset();
        self.lfos[1].set(DAT_LFO_FREQS_HZ[1], 0.5);
        self.lfos[1].set_phase_offs(0.25);
        self.lfos[1].reset();
        self.lfos[2].set(DAT_LFO_FREQS_HZ[2], 0.5);
        self.lfos[2].set_phase_offs(0.5);
        self.lfos[2].reset();
        self.lfos[3].set(DAT_LFO_FREQS_HZ[3], 0.5);
        self.lfos[3].set_phase_offs(0.75);
        self.lfos[3].reset();

        self.inp_dc_block[0].reset();
        self.inp_dc_block[1].reset();
        self.out_dc_block[0].reset();
        self.out_dc_block[1].reset();

        self.pre_delay.reset();

        self.left_sum = 0.0;
        self.right_sum = 0.0;

        self.set_time_scale(1.0);
    }

    #[inline]
    pub fn set_time_scale(&mut self, scale: f64) {
        if (self.last_scale - scale).abs() > std::f64::EPSILON {
            let scale = scale.max(0.1);
            self.last_scale = scale;

            self.apf1[0].1 = DAT_LEFT_APF1_TIME_MS * scale;
            self.apf1[1].1 = DAT_RIGHT_APF1_TIME_MS * scale;
            self.apf2[0].1 = DAT_LEFT_APF2_TIME_MS * scale;
            self.apf2[1].1 = DAT_RIGHT_APF2_TIME_MS * scale;

            self.delay1[0].1 = DAT_LEFT_DELAY1_TIME_MS * scale;
            self.delay1[1].1 = DAT_RIGHT_DELAY1_TIME_MS * scale;
            self.delay2[0].1 = DAT_LEFT_DELAY2_TIME_MS * scale;
            self.delay2[1].1 = DAT_RIGHT_DELAY2_TIME_MS * scale;
        }
    }

    pub fn set_sample_rate(&mut self, srate: f64) {
        self.inp_dc_block[0].set_sample_rate(srate);
        self.inp_dc_block[1].set_sample_rate(srate);
        self.out_dc_block[0].set_sample_rate(srate);
        self.out_dc_block[1].set_sample_rate(srate);

        self.lfos[0].set_sample_rate(srate);
        self.lfos[1].set_sample_rate(srate);
        self.lfos[2].set_sample_rate(srate);
        self.lfos[3].set_sample_rate(srate);

        self.input_hpf.set_sample_rate(srate);
        self.input_lpf.set_sample_rate(srate);

        self.pre_delay.set_sample_rate(srate);

        self.input_apfs[0].0.set_sample_rate(srate);
        self.input_apfs[1].0.set_sample_rate(srate);
        self.input_apfs[2].0.set_sample_rate(srate);
        self.input_apfs[3].0.set_sample_rate(srate);

        self.apf1[0].0.set_sample_rate(srate);
        self.apf1[1].0.set_sample_rate(srate);
        self.apf2[0].0.set_sample_rate(srate);
        self.apf2[1].0.set_sample_rate(srate);

        self.hpf[0].set_sample_rate(srate);
        self.hpf[1].set_sample_rate(srate);
        self.lpf[0].set_sample_rate(srate);
        self.lpf[1].set_sample_rate(srate);

        self.delay1[0].0.set_sample_rate(srate);
        self.delay1[1].0.set_sample_rate(srate);
        self.delay2[0].0.set_sample_rate(srate);
        self.delay2[1].0.set_sample_rate(srate);
    }

    #[inline]
    fn calc_apf_delay_times(
        &mut self,
        params: &mut dyn DattorroReverbParams,
    ) -> (f64, f64, f64, f64) {
        let left_apf1_delay_ms = self.apf1[0].1
            + (self.lfos[0].next_bipolar() as f64
                * DAT_LFO_EXCURSION_MS
                * DAT_LFO_EXCURSION_MOD_MAX
                * params.mod_depth());
        let right_apf1_delay_ms = self.apf1[1].1
            + (self.lfos[1].next_bipolar() as f64
                * DAT_LFO_EXCURSION_MS
                * DAT_LFO_EXCURSION_MOD_MAX
                * params.mod_depth());
        let left_apf2_delay_ms = self.apf2[0].1
            + (self.lfos[2].next_bipolar() as f64
                * DAT_LFO_EXCURSION_MS
                * DAT_LFO_EXCURSION_MOD_MAX
                * params.mod_depth());
        let right_apf2_delay_ms = self.apf2[1].1
            + (self.lfos[3].next_bipolar() as f64
                * DAT_LFO_EXCURSION_MS
                * DAT_LFO_EXCURSION_MOD_MAX
                * params.mod_depth());

        (left_apf1_delay_ms, right_apf1_delay_ms, left_apf2_delay_ms, right_apf2_delay_ms)
    }

    pub fn process(
        &mut self,
        params: &mut dyn DattorroReverbParams,
        input_l: f64,
        input_r: f64,
    ) -> (f64, f64) {
        // Some parameter setup...
        let timescale = 0.1 + (4.0 - 0.1) * params.time_scale();
        self.set_time_scale(timescale);

        self.hpf[0].set_freq(params.reverb_high_cutoff_hz());
        self.hpf[1].set_freq(params.reverb_high_cutoff_hz());
        self.lpf[0].set_freq(params.reverb_low_cutoff_hz());
        self.lpf[1].set_freq(params.reverb_low_cutoff_hz());

        let mod_speed = params.mod_speed();
        let mod_speed = mod_speed * mod_speed;
        let mod_speed = mod_speed * 99.0 + 1.0;

        self.lfos[0].set(DAT_LFO_FREQS_HZ[0] * mod_speed, params.mod_shape());
        self.lfos[1].set(DAT_LFO_FREQS_HZ[1] * mod_speed, params.mod_shape());
        self.lfos[2].set(DAT_LFO_FREQS_HZ[2] * mod_speed, params.mod_shape());
        self.lfos[3].set(DAT_LFO_FREQS_HZ[3] * mod_speed, params.mod_shape());

        self.apf1[0].2 = -DAT_PLATE_DIFFUSION1 * params.diffusion();
        self.apf1[1].2 = -DAT_PLATE_DIFFUSION1 * params.diffusion();
        self.apf2[0].2 = DAT_PLATE_DIFFUSION2 * params.diffusion();
        self.apf2[1].2 = DAT_PLATE_DIFFUSION2 * params.diffusion();

        let (left_apf1_delay_ms, right_apf1_delay_ms, left_apf2_delay_ms, right_apf2_delay_ms) =
            self.calc_apf_delay_times(params);

        // Parameter setup done!

        // Input into their corresponding DC blockers
        let input_r = self.inp_dc_block[0].next(input_r);
        let input_l = self.inp_dc_block[1].next(input_l);

        // Sum of DC outputs => LPF => HPF
        self.input_lpf.set_freq(params.input_low_cutoff_hz());
        self.input_hpf.set_freq(params.input_high_cutoff_hz());
        let out_lpf = self.input_lpf.process(input_r + input_l);
        let out_hpf = self.input_hpf.process(out_lpf);

        // HPF => Pre-Delay
        let out_pre_delay = if params.pre_delay_time_ms() < 0.1 {
            out_hpf
        } else {
            self.pre_delay.next_cubic(params.pre_delay_time_ms(), out_hpf)
        };

        // Pre-Delay => 4 All-Pass filters
        let mut diffused = out_pre_delay;
        for (apf, time, g) in &mut self.input_apfs {
            diffused = apf.next(*time, *g, diffused);
        }

        // Mix between diffused and pre-delayed intput for further processing
        let tank_feed = crossfade(out_pre_delay, diffused, params.input_diffusion_mix());

        // First tap for the output
        self.left_sum += tank_feed;
        self.right_sum += tank_feed;

        // Calculate tank decay of the left/right signal channels.
        let decay = 1.0 - params.decay().clamp(0.1, 0.9999);
        let decay = 1.0 - (decay * decay);

        // Left Sum => APF1 => Delay1 => LPF => HPF => APF2 => Delay2
        // And then send this over to the right sum.
        let left = self.left_sum;
        let left = self.apf1[0].0.next(left_apf1_delay_ms, self.apf1[0].2, left);
        let left_apf_tap = left;
        let left = self.delay1[0].0.next_cubic(self.delay1[0].1, left);
        let left = self.lpf[0].process(left);
        let left = self.hpf[0].process(left);
        let left = left * decay;
        let left = self.apf2[0].0.next(left_apf2_delay_ms, self.apf2[0].2, left);
        let left = self.delay2[0].0.next_cubic(self.delay2[0].1, left);

        //        if self.dbg_count % 48 == 0 {
        //            println!("APFS dcy={:8.6}; {:8.6} {:8.6} {:8.6} {:8.6} | {:8.6} {:8.6} {:8.6} {:8.6}",
        //                decay,
        //                self.apf1[0].2,
        //                self.apf1[1].2,
        //                self.apf2[0].2,
        //                self.apf2[1].2,
        //                left_apf1_delay_ms, right_apf1_delay_ms,
        //                left_apf2_delay_ms, right_apf2_delay_ms);
        //            println!("DELY1/2 {:8.6} / {:8.6} | {:8.6} / {:8.6}",
        //                self.delay1[0].1,
        //                self.delay2[0].1,
        //                self.delay1[1].1,
        //                self.delay2[1].1);
        //        }

        // Right Sum => APF1 => Delay1 => LPF => HPF => APF2 => Delay2
        // And then send this over to the left sum.
        let right = self.right_sum;
        let right = self.apf1[1].0.next(right_apf1_delay_ms, self.apf1[1].2, right);
        let right_apf_tap = right;
        let right = self.delay1[1].0.next_cubic(self.delay1[1].1, right);
        let right = self.lpf[1].process(right);
        let right = self.hpf[1].process(right);
        let right = right * decay;
        let right = self.apf2[1].0.next(right_apf2_delay_ms, self.apf2[1].2, right);
        let right = self.delay2[1].0.next_cubic(self.delay2[1].1, right);

        self.right_sum = left * decay;
        self.left_sum = right * decay;

        let mut left_accum = left_apf_tap;
        left_accum += self.delay1[0].0.tap_n(DAT_LEFT_TAPS_TIME_MS[0]);
        left_accum += self.delay1[0].0.tap_n(DAT_LEFT_TAPS_TIME_MS[1]);
        left_accum -= self.apf2[0].0.delay_tap_n(DAT_LEFT_TAPS_TIME_MS[2]);
        left_accum += self.delay2[0].0.tap_n(DAT_LEFT_TAPS_TIME_MS[3]);
        left_accum -= self.delay1[1].0.tap_n(DAT_LEFT_TAPS_TIME_MS[4]);
        left_accum -= self.apf2[1].0.delay_tap_n(DAT_LEFT_TAPS_TIME_MS[5]);
        left_accum -= self.delay2[1].0.tap_n(DAT_LEFT_TAPS_TIME_MS[6]);

        let mut right_accum = right_apf_tap;
        right_accum += self.delay1[1].0.tap_n(DAT_RIGHT_TAPS_TIME_MS[0]);
        right_accum += self.delay1[1].0.tap_n(DAT_RIGHT_TAPS_TIME_MS[1]);
        right_accum -= self.apf2[1].0.delay_tap_n(DAT_RIGHT_TAPS_TIME_MS[2]);
        right_accum += self.delay2[1].0.tap_n(DAT_RIGHT_TAPS_TIME_MS[3]);
        right_accum -= self.delay1[0].0.tap_n(DAT_RIGHT_TAPS_TIME_MS[4]);
        right_accum -= self.apf2[0].0.delay_tap_n(DAT_RIGHT_TAPS_TIME_MS[5]);
        right_accum -= self.delay2[0].0.tap_n(DAT_RIGHT_TAPS_TIME_MS[6]);

        let left_out = self.out_dc_block[0].next(left_accum);
        let right_out = self.out_dc_block[1].next(right_accum);

        self.dbg_count += 1;

        (left_out * 0.5, right_out * 0.5)
    }
}
