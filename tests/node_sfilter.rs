mod common;
use common::*;

fn setup_sfilter_matrix() -> (Matrix, NodeExecutor) {
    let (node_conf, node_exec) = new_node_engine();
    let mut matrix = Matrix::new(node_conf, 3, 3);

    let noise = NodeId::Noise(0);
    let sf    = NodeId::SFilter(0);
    let out   = NodeId::Out(0);
    matrix.place(0, 0, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(0, 1, Cell::empty(sf)
                       .input(sf.inp("inp"), None, None)
                       .out(None, None, sf.out("sig")));
    matrix.place(0, 2, Cell::empty(out)
                       .input(out.inp("ch1"), None, None));
    matrix.place(1, 1, Cell::empty(noise)
                       .out(None, None, noise.out("sig")));
    matrix.place(1, 2, Cell::empty(out)
                       .input(out.inp("ch2"), None, None));
    matrix.sync().unwrap();

    (matrix, node_exec)
}

fn fft_with_freq_res_type(
    matrix: &mut Matrix,
    node_exec: &mut NodeExecutor,
    ftype: i64, freq: f32, res: f32) -> Vec<(u16, u32)>
{
    let sf = NodeId::SFilter(0);
    pset_d(matrix, sf, "freq", freq);
    pset_d_wait(matrix, node_exec, sf, "res", res);
    pset_s(matrix, sf, "ftype", ftype);
    run_and_get_fft4096(node_exec, 0, 1000.0)
}

#[test]
fn check_node_sfilter_lowpass() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // Low Pass @ 1000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 0, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 8000, 12000, 16000,
        ], &fft[..]), vec![
            (0, 16), (100, 24), (250, 16), (500, 12), (750, 12), (1000, 12),
            (1500, 8), (2000, 4), (3000, 4), (4000, 0), (8000, 0), (12000, 0),
        ]);

//    let v = run_and_get_fft4096_2(&mut node_exec, 1);
//    assert_eq!(
//        avg_fft_freqs(4.0, &[
//            100, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 8000, 12000, 16000,
//        ], &v[..]), vec![
//            (0, 16), (100, 24), (250, 16), (500, 12), (750, 12), (1000, 12),
//            (1500, 8), (2000, 4), (3000, 4), (4000, 0), (8000, 0), (12000, 0),
//        ]);
//    assert!(false);

    // Low Pass @ 4000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 0, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 8000, 12000, 16000,
        ], &fft[..]), vec![
            (0, 16), (100, 20), (250, 16), (500, 12), (750, 20), (1000, 16),
            (1500, 16), (2000, 16), (3000, 12), (4000, 8), (8000, 4), (12000, 4),
        ]);

    // Low Pass @ 22050Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 0, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 16), (100, 16), (1000, 16), (4000, 16), (12000, 16),
        ]);

    // Low Pass @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 0, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 0), (100, 0), (1000, 0), (4000, 0), (12000, 0),
        ]);
}

#[test]
fn check_node_sfilter_lowpass_tpt() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // Low Pass TPT @ 1000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 1, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 8000, 12000, 16000,
        ], &fft[..]), vec![
            (0, 16), (100, 24), (250, 16), (500, 12), (750, 12), (1000, 12),
            (1500, 8), (2000, 4), (3000, 4), (4000, 0), (8000, 0), (12000, 0),
        ]);

    // Low Pass TPT @ 4000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 1, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 8000, 12000, 16000,
        ], &fft[..]), vec![
            (0, 16), (100, 20), (250, 16), (500, 12), (750, 20), (1000, 16),
            (1500, 16), (2000, 16), (3000, 12), (4000, 8), (8000, 4), (12000, 0),
        ]);

    // Low Pass TPT @ 22050Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 1, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 16), (100, 16), (1000, 16), (4000, 16), (12000, 16),
        ]);

    // Low Pass TPT @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 1, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 0), (100, 0), (1000, 0), (4000, 0), (12000, 0),
        ]);
}

#[test]
fn check_node_sfilter_highpass() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // High Pass @ 1000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 2, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 4), (250, 4), (500, 8), (750, 8), (1000, 16),
            (1500, 16), (2000, 16), (3000, 16), (8000, 16),
        ]);

    // High Pass @ 4000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 2, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 0), (250, 0), (500, 0), (750, 4), (1000, 4),
            (1500, 4), (2000, 8), (3000, 12), (8000, 16),
        ]);

    // High Pass @ 22050Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 2, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 0), (100, 0), (1000, 0), (4000, 8), (12000, 16),
        ]);

    // High Pass @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 2, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 24), (100, 16), (1000, 16), (4000, 16), (12000, 16),
        ]);
}

#[test]
fn check_node_sfilter_highpass_tpt() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // High Pass TPT @ 1000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 3, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 0), (250, 4), (500, 8), (750, 8), (1000, 16),
            (1500, 16), (2000, 16), (3000, 16), (8000, 16),
        ]);

    // High Pass TPT @ 4000Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 3, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 0), (250, 0), (500, 0), (750, 4), (1000, 4),
            (1500, 4), (2000, 8), (3000, 12), (8000, 16),
        ]);

    // High Pass TPT @ 22050Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 3, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 0), (100, 0), (1000, 0), (4000, 0), (12000, 0),
        ]);

    // High Pass TPT @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 3, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 24), (100, 16), (1000, 16), (4000, 16), (12000, 16),
        ]);
}


#[test]
fn check_node_sfilter_halsvf_lowpass() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // Low Pass Hal Chamberlin SVF @ 1000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 1000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 20), (500, 20), (700, 50), (900, 240), (1000, 60),
            (1500, 10), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Low Pass Hal Chamberlin SVF @ 1000Hz RES=0.5
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 1000.0, 0.5);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 20), (500, 20), (700, 30), (900, 40), (1000, 20),
            (1500, 0), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Low Pass Hal Chamberlin SVF @ 1000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 10), (500, 20), (700, 20), (900, 10), (1000, 10),
            (1500, 0), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Low Pass Hal Chamberlin SVF @ 4000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 4000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 24), (100, 16), (500, 20), (1000, 20), (2000, 40), (3500, 340),
            (4000, 180), (5000, 20), (6000, 8), (8000, 0)
        ]);

    // Low Pass Hal Chamberlin SVF @ 4000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 20), (100, 12), (500, 16), (1000, 16), (2000, 20), (3500, 20),
            (4000, 16), (5000, 8), (6000, 4), (8000, 0)
        ]);

    // Low Pass Hal Chamberlin SVF @ 22050Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]), vec![
            (0, 16), (100, 16), (1000, 16), (4000, 16), (12000, 16),
            (16000, 24), (20000, 16)
        ]);

    // Low Pass Hal Chamberlin SVF @ 22050Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 22050.0, 1.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]), vec![
            (0, 8), (100, 16), (1000, 16), (4000, 24), (12000, 160),
            (16000, 176), (20000, 24)
        ]);

    // Low Pass Hal Chamberlin SVF @ 0Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 12), (10, 0), (100, 0), (1000, 0), (4000, 0), (12000, 0),
        ]);

    // Low Pass Hal Chamberlin SVF @ 0Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 4, 0.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[1, 5, 10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 56), (1, 0), (5, 0), (10, 0), (100, 0), (1000, 0),
            (4000, 0), (12000, 0),
        ]);
}

#[test]
fn check_node_sfilter_halsvf_highpass() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // High Pass Hal Chamberlin SVF @ 1000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 1000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 0), (500, 0), (700, 30), (900, 220), (1000, 80),
            (1500, 30), (2000, 20), (3000, 20), (4000, 10)
        ]);

    // High Pass Hal Chamberlin SVF @ 1000Hz RES=0.5
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 1000.0, 0.5);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 0), (500, 0), (700, 20), (900, 30), (1000, 30),
            (1500, 20), (2000, 20), (3000, 20), (4000, 20)
        ]);

    // High Pass Hal Chamberlin SVF @ 1000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 0), (500, 0), (700, 10), (900, 10), (1000, 20), (1500, 20),
            (2000, 20), (3000, 20), (4000, 10)
        ]);

    // High Pass Hal Chamberlin SVF @ 4000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 4000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 0), (500, 0), (1000, 0), (2000, 20),
            (3500, 320), (4000, 200), (5000, 40), (6000, 28), (8000, 20)
        ]);

    // High Pass Hal Chamberlin SVF @ 4000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 0), (500, 0), (1000, 0), (2000, 8),
            (3500, 12), (4000, 16), (5000, 16), (6000, 20), (8000, 20)
        ]);

    // High Pass Hal Chamberlin SVF @ 22050Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]),
        vec![
            (0, 0), (100, 0), (1000, 8), (4000, 24),
            (12000, 32), (16000, 40), (20000, 40)
        ]);

    // High Pass Hal Chamberlin SVF @ 22050Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 22050.0, 1.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]),
        vec![
            (0, 0), (100, 0), (1000, 0), (4000, 8), (12000, 144),
            (16000, 192), (20000, 48)
        ]);

    // High Pass Hal Chamberlin SVF @ 0Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 52), (10, 12), (100, 20), (1000, 16), (4000, 16), (12000, 16)
        ]);

    // High Pass Hal Chamberlin SVF @ 0Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 5, 0.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 112), (10, 36), (100, 16), (1000, 20), (4000, 16), (12000, 20)
        ]);
}

#[test]
fn check_node_sfilter_halsvf_bandpass() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // Band Pass Hal Chamberlin SVF @ 1000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 1000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            250, 500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 0), (250, 0), (500, 10), (700, 40), (900, 230),
            (1000, 70), (1500, 20), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Band Pass Hal Chamberlin SVF @ 1000Hz RES=0.5
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 1000.0, 0.5);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            250, 500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 0), (250, 0), (500, 10), (700, 30), (900, 30),
            (1000, 30), (1500, 10), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Band Pass Hal Chamberlin SVF @ 1000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            250, 500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 0), (250, 0), (500, 10), (700, 10), (900, 10),
            (1000, 10), (1500, 10), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Band Pass Hal Chamberlin SVF @ 4000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 4000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 0), (500, 0), (1000, 0), (2000, 20),
            (3500, 330), (4000, 190), (5000, 30), (6000, 10), (8000, 0)
        ]);

    // Band Pass Hal Chamberlin SVF @ 4000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 0), (100, 0), (500, 0), (1000, 0), (2000, 10), (3500, 20),
            (4000, 10), (5000, 10), (6000, 10), (8000, 10)
        ]);

    // Band Pass Hal Chamberlin SVF @ 22050Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]),
        vec![
            (0, 0), (100, 0), (1000, 0), (4000, 0),
            (12000, 0), (16000, 0), (20000, 0)
        ]);

    // Band Pass Hal Chamberlin SVF @ 22050Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 22050.0, 1.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]),
        vec![
            (0, 0), (100, 0), (1000, 0), (4000, 8), (12000, 136),
            (16000, 200), (20000, 48)
        ]);

    // Band Pass Hal Chamberlin SVF @ 0Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 48), (10, 0), (100, 0), (1000, 0), (4000, 0), (12000, 0)
        ]);

    // Band Pass Hal Chamberlin SVF @ 0Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 6, 0.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 4), (10, 0), (100, 0), (1000, 0), (4000, 0), (12000, 0)
        ]);
}

#[test]
fn check_node_sfilter_halsvf_notch() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // Notch Hal Chamberlin SVF @ 1000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 1000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 850, 900, 950, 1000, 1100, 1200, 1400, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 10), (500, 10), (700, 10), (850, 10), (900, 20), (950, 10),
            (1000, 10), (1100, 20), (1200, 20), (1400, 20), (2000, 10),
            (3000, 10), (4000, 10)
        ]);

    // Notch Hal Chamberlin SVF @ 1000Hz RES=0.5
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 1000.0, 0.5);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 850, 900, 950, 1000, 1100, 1200, 1400, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 10), (500, 10), (700, 10), (850, 10), (900, 0), (950, 0),
            (1000, 0), (1100, 0), (1200, 10), (1400, 10), (2000, 10),
            (3000, 10), (4000, 10)
        ]);

    // Notch Hal Chamberlin SVF @ 1000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 850, 900, 950, 1000, 1100, 1200, 1400, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 10), (500, 10), (700, 0), (850, 0), (900, 0), (950, 0),
            (1000, 0), (1100, 0), (1200, 0), (1400, 10), (2000, 10),
            (3000, 10), (4000, 10)
        ]);

    // Notch Hal Chamberlin SVF @ 4000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 4000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            100, 500, 1000, 2000, 3500, 4000, 4500, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 20), (100, 10), (500, 10), (1000, 10), (2000, 10), (3500, 10),
            (4000, 20), (4500, 10), (5000, 10), (6000, 20), (8000, 10)
        ]);

    // Notch Hal Chamberlin SVF @ 4000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 20), (100, 10), (500, 10), (1000, 10), (2000, 10), (3500, 0),
            (4000, 0), (5000, 0), (6000, 10), (8000, 10)
        ]);

    // Notch Hal Chamberlin SVF @ 22050Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]),
        vec![
            (0, 16), (100, 16), (1000, 16), (4000, 16), (12000, 16),
            (16000, 16), (20000, 16)
        ]);

    // Notch Hal Chamberlin SVF @ 22050Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 22050.0, 1.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050], &fft[..]),
        vec![
            (0, 8), (100, 16), (1000, 16), (4000, 16), (12000, 16),
            (16000, 16), (20000, 16)
        ]);

    // Notch Hal Chamberlin SVF @ 0Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 68), (10, 12), (100, 20), (1000, 16), (4000, 16), (12000, 16)
        ]);

    // Notch Hal Chamberlin SVF @ 0Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 7, 0.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050], &fft[..]), vec![
            (0, 20), (10, 32), (100, 16), (1000, 20), (4000, 16), (12000, 20)
        ]);
}

#[test]
fn check_node_sfilter_simpersvf_lowpass() {
    let (mut matrix, mut node_exec) = setup_sfilter_matrix();

    // Low Pass Simper SVF @ 1000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 1000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 20), (500, 20), (700, 50), (900, 110), (1000, 40),
            (1500, 10), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Low Pass Simper SVF @ 1000Hz RES=0.5
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 1000.0, 0.5);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 10), (500, 10), (700, 20), (900, 10), (1000, 10),
            (1500, 0), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Low Pass Simper SVF @ 1000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 1000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(10.0, &[
            500, 700, 900, 1000, 1500, 2000, 3000, 4000, 12000
        ], &fft[..]), vec![
            (0, 10), (500, 10), (700, 10), (900, 0), (1000, 0),
            (1500, 0), (2000, 0), (3000, 0), (4000, 0)
        ]);

    // Low Pass Simper SVF @ 4000Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 4000.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 24), (100, 16), (500, 20), (1000, 20), (2000, 36), (3500, 132),
            (4000, 80), (5000, 20), (6000, 8), (8000, 0)
        ]);

    // Low Pass Simper SVF @ 4000Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 4000.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            100, 500, 1000, 2000, 3500, 4000, 5000, 6000, 8000, 12000
        ], &fft[..]), vec![
            (0, 20), (100, 12), (500, 16), (1000, 16), (2000, 12), (3500, 8),
            (4000, 8), (5000, 4), (6000, 4), (8000, 0)
        ]);

    // Low Pass Simper SVF @ 22050Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 22050.0, 0.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050, 22051], &fft[..]), vec![
            (0, 16), (100, 16), (1000, 16), (4000, 16), (12000, 16),
            (16000, 16), (20000, 16), (22050, 0)
        ]);

    // Low Pass Simper SVF @ 22050Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 22050.0, 1.0);
    assert_eq!(
        avg_fft_freqs(8.0, &[100, 1000, 4000, 12000, 16000, 20000, 22050, 22051], &fft[..]), vec![
            (0, 8), (100, 16), (1000, 16), (4000, 16), (12000, 16),
            (16000, 16), (20000, 16), (22050, 0)
        ]);

    // Low Pass Simper SVF @ 0Hz RES=0.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[10, 100, 1000, 4000, 12000, 22050, 22051], &fft[..]), vec![
            (0, 0), (10, 0), (100, 0), (1000, 0), (4000, 0), (12000, 0),
            (22050, 0)
        ]);

    // Low Pass Simper SVF @ 0Hz RES=1.0
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 8, 0.0, 1.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[1, 5, 10, 100, 1000, 4000, 12000, 22050, 22051], &fft[..]), vec![
            (0, 56), (1, 0), (5, 0), (10, 0), (100, 0), (1000, 0),
            (4000, 0), (12000, 0), (22050, 0)
        ]);
}

