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
