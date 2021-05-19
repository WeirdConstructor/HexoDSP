// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This is a part of HexoDSP. Released under (A)GPLv3 or any later.
// See README.md and COPYING for details.

#[derive(Debug, Clone)]
pub enum SAtom {
    Str(String),
    MicroSample(Vec<f32>),
    AudioSample((String, Option<std::sync::Arc<Vec<f32>>>)),
    Setting(i64),
    Param(f32),
}

impl SAtom {
    pub fn str(s: &str)         -> Self { SAtom::Str(s.to_string()) }
    pub fn setting(s: i64)      -> Self { SAtom::Setting(s) }
    pub fn param(p: f32)        -> Self { SAtom::Param(p) }
    pub fn micro(m: &[f32])     -> Self { SAtom::MicroSample(m.to_vec()) }
    pub fn audio(s: &str, m: std::sync::Arc<Vec<f32>>) -> Self {
        SAtom::AudioSample((s.to_string(), Some(m)))
    }

    pub fn audio_unloaded(s: &str) -> Self {
        SAtom::AudioSample((s.to_string(), None))
    }

    pub fn default_of(&self) -> Self {
        match self {
            SAtom::Str(_)         => SAtom::Str("".to_string()),
            SAtom::MicroSample(_) => SAtom::MicroSample(vec![]),
            SAtom::AudioSample(_) => SAtom::AudioSample(("".to_string(), None)),
            SAtom::Setting(_)     => SAtom::Setting(0),
            SAtom::Param(_)       => SAtom::Param(0.0),
        }
    }

    pub fn is_continous(&self) -> bool {
        if let SAtom::Param(_) = self { true }
        else { false }
    }

    pub fn i(&self) -> i64 {
        match self {
            SAtom::Setting(i) => *i,
            SAtom::Param(i)   => *i as i64,
            _                => 0,
        }
    }

    pub fn f(&self) -> f32 {
        match self {
            SAtom::Setting(i) => *i as f32,
            SAtom::Param(i)   => *i,
            _                => 0.0,
        }
    }

    pub fn v_ref(&self) -> Option<&[f32]> {
        match self {
            SAtom::MicroSample(v)            => Some(&v[..]),
            SAtom::AudioSample((_, Some(v))) => Some(&v[..]),
            _                                => None,
        }
    }
}

impl From<f32> for SAtom {
    fn from(n: f32) -> Self { SAtom::Param(n) }
}


#[cfg(feature="hexotk")]
use hexotk::Atom;

#[cfg(feature="hexotk")]
impl From<Atom> for SAtom {
    fn from(n: Atom) -> Self {
        match n {
            Atom::Str(s)         => SAtom::Str(s),
            Atom::MicroSample(s) => SAtom::MicroSample(s),
            Atom::AudioSample(s) => SAtom::AudioSample(s),
            Atom::Setting(s)     => SAtom::Setting(s),
            Atom::Param(s)       => SAtom::Param(s),
        }
    }
}

#[cfg(feature="hexotk")]
impl From<SAtom> for Atom {
    fn from(n: SAtom) -> Atom {
        match n {
            SAtom::Str(s)         => Atom::Str(s),
            SAtom::MicroSample(s) => Atom::MicroSample(s),
            SAtom::AudioSample(s) => Atom::AudioSample(s),
            SAtom::Setting(s)     => Atom::Setting(s),
            SAtom::Param(s)       => Atom::Param(s),
        }
    }
}