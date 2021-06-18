pub use hexodsp::matrix::*;
pub use hexodsp::nodes::new_node_engine;
pub use hexodsp::dsp::*;

use hound;
//use num_complex::Complex;
use microfft;

pub const SAMPLE_RATE : f32 = 44100.0;
#[allow(dead_code)]
pub const SAMPLE_RATE_US : usize = 44100;

#[macro_export]
macro_rules! assert_float_eq {
    ($a:expr, $b:expr) => {
        if ($a - $b).abs() > 0.0001 {
            panic!(r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#, $a, $b)
        }
    }
}

#[macro_export]
macro_rules! assert_fpair_eq {
    ($a:expr, $b:expr) => {
        if ($a.0 - $b.0).abs() > 0.0001 {
            panic!(r#"assertion failed: `(left.0 == right.0)`
  left: `{:?}`,
 right: `{:?}`"#, $a.0, $b.0)
        }
        if ($a.1 - $b.1).abs() > 0.0001 {
            panic!(r#"assertion failed: `(left.1 == right.1)`
  left: `{:?}`,
 right: `{:?}`"#, $a.1, $b.1)
        }
    }
}

#[macro_export]
macro_rules! assert_f3tupl_eq {
    ($a:expr, $b:expr) => {
        if ($a.0 - $b.0).abs() > 0.0001 {
            panic!(r#"assertion failed: `(left.0 == right.0)`
  left.0: `{:?}`,
 right.0: `{:?}`"#, $a.0, $b.0)
        }
        if ($a.1 - $b.1).abs() > 0.0001 {
            panic!(r#"assertion failed: `(left.1 == right.1)`
  left.1: `{:?}`,
 right.1: `{:?}`"#, $a.1, $b.1)
        }
        if ($a.2 - $b.2).abs() > 0.0001 {
            panic!(r#"assertion failed: `(left.2 == right.2)`
  left.2: `{:?}`,
 right.2: `{:?}`"#, $a.2, $b.2)
        }
    }
}

#[macro_export]
macro_rules! assert_vec_feq {
    ($vec:expr, $cmp_vec:expr) => {
        let cmp_vec = $cmp_vec;
        let res : Vec<f32> = $vec.iter().copied().collect();

        for (i, (s, scmp)) in res.iter().zip(cmp_vec.iter()).enumerate() {
            if (s - scmp).abs() > 0.0001 {
                panic!(r#"
table_left: {:?}

table_right: {:?}

assertion failed: `(left[{}] == right[{}])`
      left: `{:?}`,
     right: `{:?}`"#, &res[i..], &(cmp_vec[i..]), i, i, s, scmp)
            }
        }
    }
}

#[macro_export]
macro_rules! assert_decimated_feq {
    ($vec:expr, $decimate:expr, $cmp_vec:expr) => {
        let cmp_vec = $cmp_vec;
        let res : Vec<f32> = $vec.iter().step_by($decimate).copied().collect();

        for (i, (s, scmp)) in res.iter().zip(cmp_vec.iter()).enumerate() {
            if (s - scmp).abs() > 0.0001 {
                panic!(r#"
table_left: {:?}

table_right: {:?}

assertion failed: `(left[{}] == right[{}])`
      left: `{:?}`,
     right: `{:?}`"#, &res[i..], &(cmp_vec[i..]), i, i, s, scmp)
            }
        }
    }
}

#[macro_export]
macro_rules! assert_slope_feq {
    ($vec:expr, $cmp_vec:expr) => {
        let cmp_vec = $cmp_vec;
        let mut res : Vec<f32> = vec![];
        let mut prev = 0.0;
        for (i, s) in $vec.iter().enumerate() {
            let delta = *s - prev;
            if i > 0 {
                res.push(delta);
            }
            prev = *s;
        }

        let res : Vec<f32> = res.iter().copied().collect();

        for (i, (s, scmp)) in res.iter().zip(cmp_vec.iter()).enumerate() {
            if (s - scmp).abs() > 0.0001 {
                panic!(r#"
table_left: {:?}

table_right: {:?}

assertion failed: `(left[{}] == right[{}])`
      left: `{:?}`,
     right: `{:?}`"#, &res[i..], &(cmp_vec[i..]), i, i, s, scmp)
            }
        }
    }
}

#[macro_export]
macro_rules! assert_decimated_slope_feq {
    ($vec:expr, $decimate:expr, $cmp_vec:expr) => {
        let cmp_vec = $cmp_vec;
        let mut res : Vec<f32> = vec![];
        let mut prev = 0.0;
        for (i, s) in $vec.iter().enumerate() {
            let delta = *s - prev;
            if i > 0 {
                res.push(delta);
            }
            prev = *s;
        }

        let res : Vec<f32> = res.iter().step_by($decimate).copied().collect();

        for (i, (s, scmp)) in res.iter().zip(cmp_vec.iter()).enumerate() {
            if (s - scmp).abs() > 0.0001 {
                panic!(r#"
table_left: {:?}

table_right: {:?}

assertion failed: `(left[{}] == right[{}])`
      left: `{:?}`,
     right: `{:?}`"#, &res[i..], &(cmp_vec[i..]), i, i, s, scmp)
            }
        }
    }
}


#[macro_export]
macro_rules! assert_decimated_slope_feq_fine {
    ($vec:expr, $decimate:expr, $cmp_vec:expr) => {
        let cmp_vec = $cmp_vec;
        let mut res : Vec<f32> = vec![];
        let mut prev = 0.0;
        for (i, s) in $vec.iter().enumerate() {
            let delta = *s - prev;
            if i > 0 {
                res.push(delta);
            }
            prev = *s;
        }

        let res : Vec<f32> = res.iter().step_by($decimate).copied().collect();

        for (i, (s, scmp)) in res.iter().zip(cmp_vec.iter()).enumerate() {
            if (s - scmp).abs() > 0.0000001 {
                panic!(r#"
table_left: {:?}

table_right: {:?}

assertion failed: `(left[{}] == right[{}])`
      left: `{:?}`,
     right: `{:?}`"#, &res[i..], &(cmp_vec[i..]), i, i, s, scmp)
            }
        }
    }
}

#[macro_export]
macro_rules! assert_rmsmima {
    ($rms:expr, $b:expr) => {
        assert_f3tupl_eq!($rms, $b);
    }
}

#[macro_export]
macro_rules! assert_minmax_of_rms {
    ($rms:expr, $b:expr) => {
        let (_, min, max) = $rms;
        assert_fpair_eq!((min, max), $b);
    }
}

#[allow(unused)]
pub fn pset_n(matrix: &mut Matrix, nid: NodeId, parm: &str, v_norm: f32) {
    let p = nid.inp_param(parm).unwrap();
    matrix.set_param(p, SAtom::param(v_norm));
}

#[allow(unused)]
pub fn pset_d(matrix: &mut Matrix, nid: NodeId, parm: &str, v_denorm: f32) {
    let p = nid.inp_param(parm).unwrap();
    matrix.set_param(p, SAtom::param(p.norm(v_denorm)));
}

#[allow(dead_code)]
pub fn save_wav(name: &str, buf: &[f32]) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(name, spec).unwrap();
    for s in buf.iter() {
        let amp = i16::MAX as f32;
        writer.write_sample((amp * s) as i16).unwrap();
    }
}

pub fn run_no_input(node_exec: &mut hexodsp::nodes::NodeExecutor, seconds: f32) -> (Vec<f32>, Vec<f32>) {
    run_realtime_no_input(node_exec, seconds, false)
}

#[allow(dead_code)]
pub fn run_for_ms(node_exec: &mut hexodsp::nodes::NodeExecutor, ms: f32) -> (Vec<f32>, Vec<f32>) {
    run_realtime_no_input(node_exec, ms / 1000.0, false)
}

pub fn run_realtime_no_input(node_exec: &mut hexodsp::nodes::NodeExecutor, seconds: f32, sleep_a_bit: bool) -> (Vec<f32>, Vec<f32>) {
    node_exec.test_run(seconds, sleep_a_bit)
}

pub fn calc_rms_mimax_each_ms(buf: &[f32], ms: f32) -> Vec<(f32, f32, f32)> {
    let ms_samples = ms * SAMPLE_RATE / 1000.0;
    let len_ms     = ms_samples as usize;

    let mut idx    = 0;
    let mut res    = vec![];
    loop {
        if (idx + len_ms) > buf.len() {
            break;
        }

        let mut max = -1000.0;
        let mut min = 1000.0;
        for s in buf[idx..(idx + len_ms)].iter() {
            max = s.max(max);
            min = s.min(min);
        }

        let rms : f32 =
            buf[idx..(idx + len_ms)]
                .iter()
                .map(|s: &f32| s * s).sum::<f32>()
            / ms_samples;

        res.push((rms, min, max));

        idx += len_ms;
    }

    res
}

#[allow(dead_code)]
pub fn run_and_undersample(
    node_exec: &mut hexodsp::nodes::NodeExecutor,
    run_len_ms: f32, samples: usize) -> Vec<f32>
{
    let (out_l, _out_r) = run_no_input(node_exec, run_len_ms / 1000.0);

    let sample_interval = out_l.len() / samples;
    let mut out_samples = vec![];

    for i in 0..samples {
        let idx = i * sample_interval;
        out_samples.push(out_l[idx]);
    }

    out_samples
}

#[allow(dead_code)]
pub fn run_and_get_each_rms_mimax(
    node_exec: &mut hexodsp::nodes::NodeExecutor,
    len_ms: f32) -> Vec<(f32, f32, f32)>
{
    let (out_l, _out_r) = run_no_input(node_exec, (len_ms * 3.0) / 1000.0);
    calc_rms_mimax_each_ms(&out_l[..], len_ms)
}

#[allow(dead_code)]
pub fn run_and_get_first_rms_mimax(
    node_exec: &mut hexodsp::nodes::NodeExecutor,
    len_ms: f32) -> (f32, f32, f32)
{
    let (out_l, _out_r) = run_no_input(node_exec, (len_ms * 3.0) / 1000.0);
    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], len_ms);
    rms_mimax[0]
}

#[allow(unused)]
pub fn run_and_get_l_rms_mimax(
    node_exec: &mut hexodsp::nodes::NodeExecutor,
    len_ms: f32) -> (f32, f32, f32)
{
    let (out_l, _out_r) = run_no_input(node_exec, (len_ms * 3.0) / 1000.0);
    let rms_mimax = calc_rms_mimax_each_ms(&out_l[..], len_ms);
    rms_mimax[1]
}

#[allow(unused)]
pub fn run_and_get_counted_freq(
    node_exec: &mut hexodsp::nodes::NodeExecutor, ms: f32)
    -> f64
{
    let (out_l, _out_r) =
        // +0.1 here for some extra samples
        // this is just for tuning the frequency counter, so that it detects
        // the last swing correctly. It's probably wrong, but the results
        // match up better this way.
        run_no_input(node_exec, (ms + 0.1) / 1000.0);

    let mut zero_trans = 0;
    let mut last_val   = 0.0;

    for s in out_l.iter() {
        if last_val >= 0.0 && *s < 0.0 {
            zero_trans += 1;
        } else if last_val <= 0.0 && *s > 0.0 {
            zero_trans += 1;
        }

        last_val = *s;
    }

    println!("SAMPLES: {}", out_l.len());
    println!("ZERO TRANS: {}", zero_trans);

    let trans_per_sample =
        //                   substract the extra samples applied earlier.
        (zero_trans as f64) / ((out_l.len() - 4) as f64);
    trans_per_sample * 44100.0 * 0.5
}

#[allow(unused)]
pub fn run_and_get_fft4096(
    node_exec: &mut hexodsp::nodes::NodeExecutor,
    thres: u32,
    offs_ms: f32) -> Vec<(u16, u32)>
{
    let min_samples_for_fft = 4096.0;
    let offs_samples        = (offs_ms * (SAMPLE_RATE / 1000.0)).ceil();
    let min_len_samples =
        offs_samples
        // 2.0 * for safety margin
        + 2.0 * min_samples_for_fft;
    let run_len_s = min_len_samples / SAMPLE_RATE;
    let (mut out_l, _out_r) = run_no_input(node_exec, run_len_s);
    fft_thres_at_ms(&mut out_l[..], FFT::F4096, thres, offs_ms)
}

#[allow(unused)]
pub fn run_and_get_fft4096_now(
    node_exec: &mut hexodsp::nodes::NodeExecutor,
    thres: u32) -> Vec<(u16, u32)>
{
    let min_samples_for_fft = 4096.0 * 1.5; // 1.5 for some extra margin
    let run_len_s = min_samples_for_fft / SAMPLE_RATE;
    let (mut out_l, _out_r) = run_no_input(node_exec, run_len_s);
    fft_thres_at_ms(&mut out_l[..], FFT::F4096, thres, 0.0)
}

#[allow(unused)]
pub enum FFT {
    F16,
    F32,
    F64,
    F128,
    F512,
    F1024,
    F2048,
    F4096,
}

pub fn fft_thres_at_ms(buf: &mut [f32], size: FFT, amp_thres: u32, ms_idx: f32) -> Vec<(u16, u32)> {
    let ms_sample_offs = ms_idx * (SAMPLE_RATE / 1000.0);
    let fft_nbins = match size {
        FFT::F16      => 16,
        FFT::F32      => 32,
        FFT::F64      => 64,
        FFT::F128     => 128,
        FFT::F512     => 512,
        FFT::F1024    => 1024,
        FFT::F2048    => 2048,
        FFT::F4096    => 4096,
    };
    let len = fft_nbins;

    let idx     = ms_sample_offs as usize;
    let mut res = vec![];

    if (idx + len) > buf.len() {
        return res;
    }

    // Hann window:
    for (i, s) in buf[idx..(idx + len)].iter_mut().enumerate() {
        let w =
            0.5
            * (1.0 
               - ((2.0 * std::f32::consts::PI * i as f32)
                  / (fft_nbins as f32 - 1.0))
                 .cos());
        *s *= w;
    }

    let spec =
        match size {
            FFT::F16 =>
                microfft::real::rfft_16(&mut buf[idx..(idx + len)]),
            FFT::F32 =>
                microfft::real::rfft_32(&mut buf[idx..(idx + len)]),
            FFT::F64 =>
                microfft::real::rfft_64(&mut buf[idx..(idx + len)]),
            FFT::F128 =>
                microfft::real::rfft_128(&mut buf[idx..(idx + len)]),
            FFT::F512 =>
                microfft::real::rfft_512(&mut buf[idx..(idx + len)]),
            FFT::F1024 =>
                microfft::real::rfft_1024(&mut buf[idx..(idx + len)]),
            FFT::F2048 =>
                microfft::real::rfft_2048(&mut buf[idx..(idx + len)]),
            FFT::F4096 =>
                microfft::real::rfft_4096(&mut buf[idx..(idx + len)]),
        };
    let amplitudes: Vec<_> = spec.iter().map(|c| c.norm() as u32).collect();

    for (i, amp) in amplitudes.iter().enumerate() {
        if *amp >= amp_thres {
            let freq = (i as f32 * SAMPLE_RATE) / fft_nbins as f32;
            res.push((freq.round() as u16, *amp));
        }
    }

    res
}

