// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

use crate::nodes::NodeAudioContext;
use crate::dsp::{SAtom, ProcBuf, DspNode, LedPhaseVals};
use crate::dsp::{out, at, inp, denorm}; //, inp, denorm, denorm_v, inp_dir, at};
use super::helpers::Trigger;

/// A simple amplifier
#[derive(Debug, Clone)]
pub struct Sampl {
    phase:      f64,
    srate:      f64,
    trig:       Trigger,
    is_playing: bool,
}

impl Sampl {
    pub fn new() -> Self {
        Self {
            phase:      0.0,
            srate:      44100.0,
            trig:       Trigger::new(),
            is_playing: false,
        }
    }
    pub const freq : &'static str =
        "Sampl freq\nPitch input for the sampler, giving the playback speed of the \
        sample.\nRange: (-1..1)\n";

    pub const trig : &'static str =
        "Sampl trig\nThe trigger input causes a resync of the playback phase \
         and triggers the playback if the 'pmode' is 'OneShot'";
    pub const spos : &'static str =
        "Sampl spos\nStart position offset.\nRange: (-1..1)\n";
    pub const epos : &'static str =
        "Sampl epos\nEnd position offset.\nRange: (-1..1)\n";

    pub const sample : &'static str =
        "Sampl sample\nThe audio sample that is played back.\nRange: (-1..1)\n";

    pub const pmode : &'static str =
        "Sampl pmode\nThe playback mode of the sampler.\n\
        - 'Loop' constantly plays back the sample. You can reset/sync the phase \
        using the 'trig' input in this case.\n\
        - 'OneShot' plays back the sample if a trigger is received on 'trig' input.\n";
    pub const dclick : &'static str =
        "Sampl dclick\nIf this is enabled and the 'pmode' is 'OneShot' \
         this will enable short fade in and out ramps.\n\
         This if useful if you don't want to add an envelope just for \
         getting rid of the clicks if spos and epos are modulated.";

    pub const sig : &'static str =
        "Sampl sig\nSampler audio output\nRange: (-1..1)\n";
}

impl Sampl {
    #[inline]
    fn next_sample(&mut self, sr_factor: f64, speed: f64, sample_data: &[f32]) -> f32 {
        let sd_len = sample_data.len();

        let i = self.phase.floor() as usize + sd_len;

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

        let f = self.phase.fract();

        self.phase = (i % sd_len) as f64 + f + sr_factor * speed;

        let f = f as f32;

        (((a * f) - b_neg) * f + c) * f + x0
    }

    #[inline]
    fn play_loop(&mut self, inputs: &[ProcBuf], nframes: usize, sample_data: &[f32], out: &mut ProcBuf) {
        let freq = inp::Sampl::freq(inputs);

        let sample_srate = sample_data[0] as f64;
        let sample_data  = &sample_data[1..];
        let sr_factor    = sample_srate / self.srate;

        for frame in 0..nframes {
            let playback_speed =
                denorm::Sampl::freq(freq, frame) / 440.0;

            out.write(frame,
                self.next_sample(
                    sr_factor, playback_speed as f64, sample_data));
        }
    }

    #[inline]
    fn play_oneshot(&mut self, inputs: &[ProcBuf], nframes: usize,
                    sample_data: &[f32], out: &mut ProcBuf)
    {
        let freq = inp::Sampl::freq(inputs);
        let trig = inp::Sampl::trig(inputs);

        let sample_srate = sample_data[0] as f64;
        let sample_data  = &sample_data[1..];
        let sr_factor    = sample_srate / self.srate;

        let mut is_playing = self.is_playing;

        for frame in 0..nframes {
            let trig_val = denorm::Sampl::trig(trig, frame);
            let triggered = self.trig.check_trigger(trig_val);

            if triggered {
                self.phase = 0.0;
                is_playing = true;
            }

            if is_playing {
                let playback_speed =
                    denorm::Sampl::freq(freq, frame) / 440.0;

                let prev_phase = self.phase;

                    let s = self.next_sample(
                        sr_factor, playback_speed as f64, sample_data);
                out.write(frame, s);

                // played past end => stop playing.
                if prev_phase > self.phase {
                    is_playing = false;
                }
            } else {
                out.write(frame, 0.0);
            }
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
        &mut self, ctx: &mut T, atoms: &[SAtom], _params: &[ProcBuf],
        inputs: &[ProcBuf], outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        let sample = at::Sampl::sample(atoms);
        let pmode  = at::Sampl::pmode(atoms);
        let out    = out::Sampl::sig(outputs);

        if let SAtom::AudioSample((_, Some(sample_data))) = sample {
            if pmode.i() == 0 {
                self.play_loop(inputs, ctx.nframes(), &sample_data[..], out);
            } else {
                self.play_oneshot(inputs, ctx.nframes(), &sample_data[..], out);
            }
        } else {
            for frame in 0..ctx.nframes() {
                out.write(frame, 0.0);
            }
        }

        let last_frame = ctx.nframes() - 1;
        ctx_vals[0].set(out.read(last_frame));
    }
}
