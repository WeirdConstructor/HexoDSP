// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals};
use crate::dsp::{out, at, inp, denorm, denorm_offs}; //, inp, denorm, denorm_v, inp_dir, at};
use super::helpers::Trigger;

#[macro_export]
macro_rules! fa_sampl_dir { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "Forward",
            1  => "Reverse",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

#[macro_export]
macro_rules! fa_sampl_dclick { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "Off",
            1  => "On",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

#[macro_export]
macro_rules! fa_sampl_pmode { ($formatter: expr, $v: expr, $denorm_v: expr) => { {
    let s =
        match ($v.round() as usize) {
            0  => "Loop",
            1  => "OneShot",
            _  => "?",
        };
    write!($formatter, "{}", s)
} } }

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Sampl {
    phase:          f64,
    srate:          f64,
    trig:           Trigger,
    is_playing:     bool,
    last_sample:    f32,
    decaying:       f32,
}

impl Sampl {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            phase:          0.0,
            srate:          44100.0,
            trig:           Trigger::new(),
            is_playing:     false,
            last_sample:    0.0,
            decaying:       0.0,
        }
    }
    pub const freq : &'static str =
        "Sampl freq\nPitch input for the sampler, giving the playback speed of the \
        sample.\nRange: (-1..1)\n";

    pub const trig : &'static str =
        "Sampl trig\nThe trigger input causes a resync of the playback phase \
         and triggers the playback if the 'pmode' is 'OneShot'";
    pub const offs : &'static str =
        "Sampl offs\nStart position offset.\nRange: (0..1)\n";
    pub const len  : &'static str =
        "Sampl len\nAdjusts the playback length of the sample in relation \
        to the original length of the sample.\nRange: (0..1)\n";
    pub const dcms   : &'static str =
        "Sampl dcms\nDeclick fade time in milliseconds.\nNot audio rate!\nRange: (0..1)\n";
    pub const det : &'static str =
        "Sin det\nDetune the oscillator in semitones and cents. \
         the input of this value is rounded to semitones on coarse input. \
         Fine input lets you detune in cents (rounded). \
         A signal sent to this port is not rounded.\n\
         Note: The signal input allows detune +-10 octaves.\
         \nRange: (Knob -0.2 .. 0.2) / (Signal -1.0 .. 1.0)\n";

    pub const sample : &'static str =
        "Sampl sample\nThe audio sample that is played back.\nRange: (-1..1)\n";

    pub const pmode : &'static str =
        "Sampl pmode\nThe playback mode of the sampler.\n\
        - 'Loop' constantly plays back the sample. You can reset/sync the phase \
        using the 'trig' input in this case.\n\
        - 'OneShot' plays back the sample if a trigger is received on 'trig' input.\n";
    pub const dclick : &'static str =
        "Sampl dclick\nIf this is enabled it will enable short fade in and out ramps.\n\
         This if useful if you don't want to add an envelope just for \
         getting rid of the clicks if spos and epos are modulated.";
    pub const dir : &'static str =
        "Sampl dir\nSets the direction of the playhead, plays the sample \
        forwards or backwards.";

    pub const sig : &'static str =
        "Sampl sig\nSampler audio output\nRange: (-1..1)\n";

    pub const DESC : &'static str =
        "Sample Player\n\n\
         Provides a simple sample player that you can load a single audio \
         sample from a WAV file into.";
    pub const HELP : &'static str =
r#"Sample Player

Provides a simple sample player for playing back one loaded audio sample.
It can be used for purposes like:

* Adding ambient samples to your patches.
* Using drum samples (set 'pmode' to 'OneShot').
* Having an oscillator with a custom waveform (set 'pmode' to 'Loop').
* As custom CV source for very long or very custom envelopes.

Only a single audio sample can be loaded into this player. In HexoSynth
the sample selection can be done by the file browser in the right panel
in the 'Samples' tab.

You can adjust the playback speed of the sample either by the 'freq' parameter
or the 'det' parameter. You can offset into the sample using the 'offs'
parameter and modify the playback length relative to the original
sample length using the 'len' parameter.

Even though you are advised to use an envelope for controlling the playback
volume of the sample to prevent clicks a simple in and out ramp is provided
using by the 'dclick' setting. The length of these ramps can be controlled
using the 'dcms' parameter.

When 'pmode' is set to 'Loop' the sample will restart playing immediately
after it has finished. This is useful when you just want to load a waveform
into the sample player to use it as oscillator.

To start samples when 'pmode' is set to 'OneShot' a trigger input needs to
be provided on the 'trig' input port. The 'trig' input also works in
'Loop' mode to retrigger the sample.
"#;
}

impl Sampl {
    #[allow(clippy::many_single_char_names)]
    #[inline]
    fn next_sample_rev(&mut self, sr_factor: f64, speed: f64, sample_data: &[f32]) -> f32 {
        let sd_len = sample_data.len();
        if sd_len < 1 { return 0.0; }

        let j = self.phase.floor() as usize % sd_len;
        let i = ((sd_len - 1) - j) + sd_len;

        let f = self.phase.fract();
        self.phase = j as f64 + f + sr_factor * speed;

        // Hermite interpolation, take from 
        // https://github.com/eric-wood/delay/blob/main/src/delay.rs#L52
        //
        // Thanks go to Eric Wood!
        //
        // For the interpolation code:
        // MIT License, Copyright (c) 2021 Eric Wood
        let xm1 = sample_data[(i + 1) % sd_len];
        let x0  = sample_data[i       % sd_len];
        let x1  = sample_data[(i - 1) % sd_len];
        let x2  = sample_data[(i - 2) % sd_len];

        let c     = (x1 - xm1) * 0.5;
        let v     = x0 - x1;
        let w     = c + v;
        let a     = w + v + (x2 - x0) * 0.5;
        let b_neg = w + a;

        let f = (1.0 - f) as f32;
        (((a * f) - b_neg) * f + c) * f + x0
    }

    #[allow(clippy::many_single_char_names)]
    #[inline]
    fn next_sample(&mut self, sr_factor: f64, speed: f64, sample_data: &[f32]) -> f32 {
        let sd_len = sample_data.len();
        if sd_len < 1 { return 0.0; }

        let i = self.phase.floor() as usize + sd_len;
        let f = self.phase.fract();
        self.phase = (i % sd_len) as f64 + f + sr_factor * speed;

        // Hermite interpolation, take from 
        // https://github.com/eric-wood/delay/blob/main/src/delay.rs#L52
        //
        // Thanks go to Eric Wood!
        //
        // For the interpolation code:
        // MIT License, Copyright (c) 2021 Eric Wood
        let xm1 = sample_data[(i - 1) % sd_len];
        let x0  = sample_data[i       % sd_len];
        let x1  = sample_data[(i + 1) % sd_len];
        let x2  = sample_data[(i + 2) % sd_len];

        let c     = (x1 - xm1) * 0.5;
        let v     = x0 - x1;
        let w     = c + v;
        let a     = w + v + (x2 - x0) * 0.5;
        let b_neg = w + a;

        let f = f as f32;
        (((a * f) - b_neg) * f + c) * f + x0
    }

    #[allow(clippy::float_cmp)]
    #[inline]
    fn play(&mut self, inputs: &[ProcBuf], nframes: usize,
            sample_data: &[f32], out: &mut ProcBuf, do_loop: bool,
            declick: bool, reverse: bool)
    {
        let freq  = inp::Sampl::freq(inputs);
        let trig  = inp::Sampl::trig(inputs);
        let offs  = inp::Sampl::offs(inputs);
        let len   = inp::Sampl::len(inputs);
        let dcms  = inp::Sampl::dcms(inputs);
        let det   = inp::Sampl::det(inputs);

        let sample_srate = sample_data[0] as f64;
        let sample_data  = &sample_data[1..];
        let sr_factor    = sample_srate / self.srate;

        let ramp_time         = denorm::Sampl::dcms(dcms, 0) as f64 * self.srate;
        let ramp_sample_count = (ramp_time / 1000.0).ceil() as usize;
        let ramp_inc          = 1000.0 / ramp_time;

        let mut is_playing = self.is_playing;

        if do_loop {
            is_playing = true;
        }

        let mut prev_offs  = -10.0;
        let mut prev_len   = -10.0;

        let mut start_idx     = 0;
        let mut end_idx_plus1 = sample_data.len();

        for frame in 0..nframes {
            let trig_val = denorm::Sampl::trig(trig, frame);
            let triggered = self.trig.check_trigger(trig_val);

            if triggered {
                self.phase = 0.0;
                self.decaying = self.last_sample;
                is_playing = true;
            }

            let s =
                if is_playing {
                    let freq =
                        denorm_offs::Sampl::freq(
                            freq, det.read(frame), frame);
                    let playback_speed = freq / 440.0;

                    let prev_phase = self.phase;

                    let sd_len = sample_data.len();

                    let cur_offs =
                        denorm::Sampl::offs(offs, frame).abs().min(0.999999)
                         as f64;
                    let recalc_end =
                        if prev_offs != cur_offs {
                            start_idx =
                                ((sd_len as f64 * cur_offs)
                                .floor() as usize).min(sd_len);
                            prev_offs = cur_offs;
                            true
                        } else {
                            false
                        };

                    let cur_len =
                         denorm::Sampl::len(len, frame).abs().min(1.0) as f64;
                    if recalc_end || prev_len != cur_len {
                        let max_sd_len =
                            (sd_len as f64 * cur_len as f64).round() as usize;

                        let remain_s_len =
                            if start_idx <= sd_len {
                                (sd_len - start_idx).min(max_sd_len)
                            } else { 0 };

                        end_idx_plus1 = remain_s_len;

                        prev_len = cur_len;
                    }

                    let sample_slice =
                        &sample_data[start_idx..(start_idx + end_idx_plus1)];

                    // next_sample mutates self.phase, so we need the current phase
                    // that is used for looking up the sample from the audio data.
                    let sample_idx = self.phase.floor() as usize;

                    let mut s =
                        if reverse {
                            self.next_sample_rev(
                                sr_factor,
                                playback_speed as f64,
                                sample_slice)
                        } else {
                            self.next_sample(
                                sr_factor,
                                playback_speed as f64,
                                sample_slice)
                        };

                    if declick {
                        let samples_to_end = sample_slice.len() - sample_idx;

                        let ramp_atten_factor =
                            if sample_idx < ramp_sample_count {
                                sample_idx as f64 * ramp_inc
                            } else if samples_to_end < ramp_sample_count {
                                samples_to_end as f64 * ramp_inc
                            } else {
                                1.0
                            };

                        s *= ramp_atten_factor as f32;
                    }

                    self.last_sample = s;
                    out.write(frame, s);

                    if !do_loop && prev_phase > self.phase {
                        // played past end => stop playing.
                        is_playing = false;
                    }

                    s
                } else {
                    0.0
                };

            let s =
                if !declick || self.decaying.abs() < 0.00001 {
                    self.decaying = 0.0;
                    s
                } else {
                    self.decaying *= 0.98;
                    (s + self.decaying).clamp(-1.0, 1.0)
                };

            self.last_sample = s;
            out.write(frame, s);
        }

        self.is_playing = is_playing;
    }

}

impl DspNode for Sampl {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) { self.srate = srate.into(); }
    fn reset(&mut self) {
        self.trig.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        atoms: &[SAtom], _params: &[ProcBuf], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        let sample = at::Sampl::sample(atoms);
        let pmode  = at::Sampl::pmode(atoms);
        let dclick = at::Sampl::dclick(atoms);
        let dir    = at::Sampl::dir(atoms);
        let out    = out::Sampl::sig(outputs);

        if let SAtom::AudioSample((_, Some(sample_data))) = sample {
            // 3 is for sample-sample-rate and at least 2 audio samples.
            if sample_data.len() < 3 {
                for frame in 0..ctx.nframes() {
                    out.write(frame, 0.0);
                }
                self.last_sample = 0.0;
                return;
            }

            self.play(
                inputs,
                ctx.nframes(),
                &sample_data[..],
                out,
                pmode.i() == 0,
                dclick.i() == 1,
                dir.i() == 1);
        } else {
            for frame in 0..ctx.nframes() {
                out.write(frame, 0.0);
            }
            self.last_sample = 0.0;
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(out.read(last_frame));
    }
}
