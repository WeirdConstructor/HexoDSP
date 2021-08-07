// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

use crate::nodes::{NodeAudioContext, NodeExecContext};
use crate::dsp::{
    NodeId, SAtom, ProcBuf, DspNode, LedPhaseVals,
    GraphAtomData, GraphFun, NodeContext,
    denorm
};
use super::helpers::crossfade;
use super::dattorro::{
    DattorroReverb,
    DattorroReverbParams
};

pub struct DatParams {
    frame:  usize,
    predly: ProcBuf,
    size:   ProcBuf,
    dcy:    ProcBuf,
    ilpf:   ProcBuf,
    ihpf:   ProcBuf,
    idif:   ProcBuf,
    dmix:   ProcBuf,
    mspeed: ProcBuf,
    mshp:   ProcBuf,
    mdepth: ProcBuf,
    rlpf:   ProcBuf,
    rhpf:   ProcBuf,
}

impl DatParams {
    #[inline]
    pub fn set_frame(&mut self, frame: usize) { self.frame = frame; }
}

impl DattorroReverbParams for DatParams {
    fn pre_delay_time_ms(&self) -> f32 {
        denorm::PVerb::predly(&self.predly, self.frame)
    }
    fn time_scale(&self) -> f32 {
        denorm::PVerb::size(&self.size, self.frame)
    }
    fn decay(&self) -> f32 {
        denorm::PVerb::dcy(&self.dcy, self.frame)
    }
    fn input_low_cutoff_hz(&self) -> f32 {
        denorm::PVerb::ilpf(&self.ilpf, self.frame)
    }
    fn input_high_cutoff_hz(&self) -> f32 {
        denorm::PVerb::ihpf(&self.ihpf, self.frame)
    }
    fn diffusion(&self) -> f32 {
        denorm::PVerb::idif(&self.idif, self.frame)
    }
    fn input_diffusion_mix(&self) -> f32 {
        denorm::PVerb::dmix(&self.dmix, self.frame)
    }
    fn mod_speed(&self) -> f32 {
        denorm::PVerb::mspeed(&self.mspeed, self.frame)
    }
    fn mod_depth(&self) -> f32 {
        denorm::PVerb::mdepth(&self.mdepth, self.frame)
    }
    fn mod_shape(&self) -> f32 {
        denorm::PVerb::mshp(&self.mshp, self.frame)
    }
    fn reverb_low_cutoff_hz(&self) -> f32 {
        denorm::PVerb::rlpf(&self.rlpf, self.frame)
    }
    fn reverb_high_cutoff_hz(&self) -> f32 {
        denorm::PVerb::rhpf(&self.rhpf, self.frame)
    }
}

#[derive(Debug, Clone)]
pub struct PVerb {
    verb:    Box<DattorroReverb>,
}

impl PVerb {
    pub fn new(_nid: &NodeId) -> Self {
        Self {
            verb: Box::new(DattorroReverb::new()),
        }
    }

    pub const in_l : &'static str =
        "PVerb in_l\n\nRange: (-1..1)\n";
    pub const in_r : &'static str =
        "PVerb in_r\n\nRange: (-1..1)\n";
    pub const sig_l : &'static str =
        "PVerb sig_l\n\nRange: (0..1)";
    pub const sig_r : &'static str =
        "PVerb sig_r\n\nRange: (0..1)";
    pub const predly : &'static str =
        "PVerb predly\n\nRange: (0..1)";
    pub const size : &'static str =
        "PVerb size\n\nRange: (0..1)";
    pub const dcy : &'static str =
        "PVerb dcy\n\nRange: (0..1)";
    pub const ilpf : &'static str =
        "PVerb ilpf\n\nRange: (0..1)";
    pub const ihpf : &'static str =
        "PVerb ihpf\n\nRange: (0..1)";
    pub const idif : &'static str =
        "PVerb idif\n\nRange: (0..1)";
    pub const dmix : &'static str =
        "PVerb dmix\n\nRange: (0..1)";
    pub const mspeed : &'static str =
        "PVerb mspeed\n\nRange: (0..1)";
    pub const mshp : &'static str =
        "PVerb mshp\n\nRange: (0..1)";
    pub const mdepth : &'static str =
        "PVerb mdepth\n\nRange: (0..1)";
    pub const rlpf : &'static str =
        "PVerb rlpf\n\nRange: (0..1)";
    pub const rhpf : &'static str =
        "PVerb rhpf\n\nRange: (0..1)";
    pub const mix : &'static str =
        "PVerb mix\n\nRange: (0..1)";
    pub const DESC : &'static str =
r#"Plate Reverb

This is a simple but yet powerful small plate reverb based on the design by Jon Dattorro. It should suit your needs from small rooms up to large athmospheric sound scapes.
"#;
    pub const HELP : &'static str =
r#"PVerb - Plate Reverb (by Jon Dattorro)

This is a simple but yet powerful small plate reverb based on the design
by Jon Dattorro. It should suit your needs from small rooms up to large
athmospheric sound scapes. It provides two inputs, and two outputs for
stereo signals. You can also feed a monophonic input, and you will get
a stereo output.

It provides simple low-pass and high-pass filters for the inputs
and another set of them for the internal reverberation tank to control
the bandwidth of the reverbs.

Internal modulation keeps the sound alive and spreads it even more.
"#;

}

impl DspNode for PVerb {
    fn outputs() -> usize { 1 }

    fn set_sample_rate(&mut self, srate: f32) {
        self.verb.set_sample_rate(srate);
    }

    fn reset(&mut self) {
        self.verb.reset();
    }

    #[inline]
    fn process<T: NodeAudioContext>(
        &mut self, ctx: &mut T, _ectx: &mut NodeExecContext,
        _nctx: &NodeContext,
        _atoms: &[SAtom], inputs: &[ProcBuf],
        outputs: &mut [ProcBuf], ctx_vals: LedPhaseVals)
    {
        use crate::dsp::{out, inp, out_idx};

        let in_l   = inp::PVerb::in_l(inputs);
        let in_r   = inp::PVerb::in_r(inputs);

        let mut params = DatParams {
            frame: 0,
            predly: *inp::PVerb::predly(inputs),
            size:   *inp::PVerb::size(inputs),
            dcy:    *inp::PVerb::dcy(inputs),
            ilpf:   *inp::PVerb::ilpf(inputs),
            ihpf:   *inp::PVerb::ihpf(inputs),
            idif:   *inp::PVerb::idif(inputs),
            dmix:   *inp::PVerb::dmix(inputs),
            mspeed: *inp::PVerb::mspeed(inputs),
            mshp:   *inp::PVerb::mshp(inputs),
            mdepth: *inp::PVerb::mdepth(inputs),
            rlpf:   *inp::PVerb::rlpf(inputs),
            rhpf:   *inp::PVerb::rhpf(inputs),
        };

        let mix    = inp::PVerb::mix(inputs);
//        let out_l  = out::PVerb::sig_l(outputs);
//        let out_r  = out::PVerb::sig_r(outputs);
        let out_i  = out_idx::PVerb::sig_r();
        let (out_l, out_r) = outputs.split_at_mut(out_i);
        let out_l = &mut out_l[0];
        let out_r = &mut out_r[0];

        let mut verb = &mut *self.verb;

        for frame in 0..ctx.nframes() {
            let (i_l, i_r) = (in_l.read(frame), in_r.read(frame));

            params.set_frame(frame);
            let (l, r) = verb.process(&mut params, i_l, i_r);

            out_l.write(frame, crossfade(i_l, l, denorm::PVerb::mix(mix, frame)));
            out_r.write(frame, crossfade(i_r, r, denorm::PVerb::mix(mix, frame)));
        }

        ctx_vals[0].set(
              out_l.read(ctx.nframes() - 1)
            + out_r.read(ctx.nframes() - 1));
    }
}
