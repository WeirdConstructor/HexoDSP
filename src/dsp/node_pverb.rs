// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::dsp::{denorm, DspNode, GraphFun, LedPhaseVals, NodeContext, NodeId, ProcBuf, SAtom};
use crate::nodes::{NodeAudioContext, NodeExecContext};
use synfx_dsp::{crossfade, DattorroReverb, DattorroReverbParams};

pub struct DatParams {
    frame: usize,
    predly: ProcBuf,
    size: ProcBuf,
    dcy: ProcBuf,
    ilpf: ProcBuf,
    ihpf: ProcBuf,
    dif: ProcBuf,
    dmix: ProcBuf,
    mspeed: ProcBuf,
    mshp: ProcBuf,
    mdepth: ProcBuf,
    rlpf: ProcBuf,
    rhpf: ProcBuf,
}

impl DatParams {
    #[inline]
    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame;
    }
}

impl DattorroReverbParams for DatParams {
    fn pre_delay_time_ms(&self) -> f64 {
        denorm::PVerb::predly(&self.predly, self.frame) as f64
    }
    fn time_scale(&self) -> f64 {
        denorm::PVerb::size(&self.size, self.frame) as f64
    }
    fn decay(&self) -> f64 {
        denorm::PVerb::dcy(&self.dcy, self.frame) as f64
    }
    fn input_low_cutoff_hz(&self) -> f64 {
        denorm::PVerb::ilpf(&self.ilpf, self.frame) as f64
    }
    fn input_high_cutoff_hz(&self) -> f64 {
        denorm::PVerb::ihpf(&self.ihpf, self.frame) as f64
    }
    fn diffusion(&self) -> f64 {
        denorm::PVerb::dif(&self.dif, self.frame) as f64
    }
    fn input_diffusion_mix(&self) -> f64 {
        denorm::PVerb::dmix(&self.dmix, self.frame) as f64
    }
    fn mod_speed(&self) -> f64 {
        denorm::PVerb::mspeed(&self.mspeed, self.frame) as f64
    }
    fn mod_depth(&self) -> f64 {
        denorm::PVerb::mdepth(&self.mdepth, self.frame) as f64
    }
    fn mod_shape(&self) -> f64 {
        denorm::PVerb::mshp(&self.mshp, self.frame) as f64
    }
    fn reverb_low_cutoff_hz(&self) -> f64 {
        denorm::PVerb::rlpf(&self.rlpf, self.frame) as f64
    }
    fn reverb_high_cutoff_hz(&self) -> f64 {
        denorm::PVerb::rhpf(&self.rhpf, self.frame) as f64
    }
}

#[derive(Debug, Clone)]
pub struct PVerb {
    verb: Box<DattorroReverb>,
}

impl PVerb {
    pub fn new(_nid: &NodeId) -> Self {
        Self { verb: Box::new(DattorroReverb::new()) }
    }

    pub const in_l: &'static str = "Left input channel, will be summed with the right \
        channel. So you can just feed in a mono signal \
        without harm.";
    pub const in_r: &'static str = "Right input channel, will be summed with the \
        left channel.";
    pub const sig_l: &'static str = "The left channel of the output signal.";
    pub const sig_r: &'static str = "The right channel of the output signal.";
    pub const predly: &'static str = "The pre-delay length for the first reflection.";
    pub const size: &'static str = "The size of the simulated room. Goes from a small \
        chamber to a huge hall.";
    pub const dcy: &'static str = "The decay of the sound. If you set this to **1.0** the
        sound will infinitively be sustained. Just be careful feeding in more \
        sound with that.";
    pub const ilpf: &'static str = "Input low-pass filter cutoff frequency, for filtering \
        the input before it's fed into the pre-delay.";
    pub const ihpf: &'static str = "Input high-pass filter cutoff frequency, for filtering \
        the input before it's fed into the pre-delay.";
    pub const dif: &'static str = "The amount of diffusion inside the reverb tank. \
        Setting this to **0** will disable any kind of diffusion and the reverb \
        will become a more or less simple echo effect.";
    pub const dmix: &'static str = "The mix between input diffusion and clean output of the \
        pre-delay. Setting this to **0** will not diffuse any input.";
    pub const mspeed: &'static str = "The internal LFO speed, that modulates the internal \
        diffusion inside the reverb tank. Keeping this low (< **0.2**) will sound \
        a bit more natural than a fast LFO.";
    pub const mshp: &'static str =
        "The shape of the LFO. **0.0** is a down ramp, **1.0** is an up \
        ramp and **0.0** is a triangle. Setting this to **0.5** is a good choice. The \
        extreme values of **0.0** and **1.0** can lead to audible artifacts.";
    pub const mdepth: &'static str = "The depth of the LFO change that is applied to the \
        diffusion inside the reverb tank. More extreme values (above **0.2**) will \
        lead to more detuned sounds reverbing inside the tank.";
    pub const rlpf: &'static str = "Reverb tank low-pass filter cutoff frequency.";
    pub const rhpf: &'static str = "Reverb tank high-pass filter cutoff frequency.";
    pub const mix: &'static str = "Dry/Wet mix between the input and the diffused output.";
    pub const DESC: &'static str = r#"Plate Reverb

This is a simple but yet powerful small plate reverb based on the design by Jon Dattorro.
It should suit your needs from small rooms up to large atmospheric sound scapes.
"#;
    pub const HELP: &'static str = r#"Plate Reverb (by Jon Dattorro)

This is a simple but yet powerful small plate reverb based on the design
by Jon Dattorro. It should suit your needs from small rooms up to large
atmospheric sound scapes. It provides two inputs, and two outputs for
stereo signals. You can also feed a monophonic input, and you will get
a stereo output.

It provides simple low-pass and high-pass filters for the inputs
and another set of them for the internal reverberation tank to control
the bandwidth of the reverbs.

Internal modulation keeps the sound alive and spreads it even more.

Structure of the reverb is:

```text
      Left       Right
        |         |
        \----+----/
             v
           'ilpf'           'ihpf'         'predly'
      Input Low-Pass -> Input High-Pass -> Pre-Delay
                                                   |
           o------------------o--------------\     |
           +------\           +----------\   |     |
           v      |           v          |   |  v--o----> All-Pass Diffusor
     [Left Channel]     [Right Channel]  |   \--x 'dmix'     |
    /> Diffusor 1 |'size' Diffusor 1 <-\ |      ^------------/
    |    Delay 1  |'size'   Delay 1    | |
    |   LPF/HPF   |        LPF/HPF   'rlpf'/'rhpf'
    |  [x Decay]  |'dcy'  [x Decay]    | |               'mspeed'
    o> Diffusor 2 |'size' Diffusor 2 <-o----o-x-----LFO  'mshp'
    |    Delay 2  |'size'   Delay 2      |  | 'mdepth'
    |      |      |            |         |  |
    |      x 'dcy'|            x         |  |
    |      |      \-[feedback]-/         |  |
    |      \--------[feedback]-----------/  |
    \--------------------------------------/

      Multiple Taps into Left/Right Diffusors 1/2 and Delays 1/2
      are then fed to the left and right output channels.
```

"#;

    pub fn graph_fun() -> Option<GraphFun> {
        None
    }
}

impl DspNode for PVerb {
    fn set_sample_rate(&mut self, srate: f32) {
        self.verb.set_sample_rate(srate as f64);
    }

    fn reset(&mut self) {
        self.verb.reset();
    }

    #[inline]
    fn process(
        &mut self,
        ctx: &mut dyn NodeAudioContext,
        _ectx: &mut NodeExecContext,
        nctx: &NodeContext,
        _atoms: &[SAtom],
        inputs: &[ProcBuf],
        outputs: &mut [ProcBuf],
        ctx_vals: LedPhaseVals,
    ) {
        use crate::dsp::{inp, out_idx};

        let mut in_l = inp::PVerb::in_l(inputs);
        let mut in_r = inp::PVerb::in_r(inputs);

        if (nctx.in_connected & 0x03) != 0x03 {
            if nctx.in_connected & 0x01 == 0x01 {
                in_r = in_l;
            } else if nctx.in_connected & 0x02 == 0x02 {
                in_l = in_r;
            }
        }

        let mut params = DatParams {
            frame: 0,
            predly: *inp::PVerb::predly(inputs),
            size: *inp::PVerb::size(inputs),
            dcy: *inp::PVerb::dcy(inputs),
            ilpf: *inp::PVerb::ilpf(inputs),
            ihpf: *inp::PVerb::ihpf(inputs),
            dif: *inp::PVerb::dif(inputs),
            dmix: *inp::PVerb::dmix(inputs),
            mspeed: *inp::PVerb::mspeed(inputs),
            mshp: *inp::PVerb::mshp(inputs),
            mdepth: *inp::PVerb::mdepth(inputs),
            rlpf: *inp::PVerb::rlpf(inputs),
            rhpf: *inp::PVerb::rhpf(inputs),
        };

        let mix = inp::PVerb::mix(inputs);
        let out_i = out_idx::PVerb::sig_r();
        let (out_l, out_r) = outputs.split_at_mut(out_i);
        let out_l = &mut out_l[0];
        let out_r = &mut out_r[0];

        let verb = &mut *self.verb;

        for frame in 0..ctx.nframes() {
            let (i_l, i_r) = (in_l.read(frame), in_r.read(frame));

            params.set_frame(frame);
            let (l, r) = verb.process(&mut params, i_l as f64, i_r as f64);

            out_l.write(frame, crossfade(i_l, l as f32, denorm::PVerb::mix(mix, frame)));
            out_r.write(frame, crossfade(i_r, r as f32, denorm::PVerb::mix(mix, frame)));
        }

        ctx_vals[0].set(out_l.read(ctx.nframes() - 1) + out_r.read(ctx.nframes() - 1));
    }
}
