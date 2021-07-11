mod common;
use common::*;

fn setup_sfilter_matrix() -> (Matrix, NodeExecutor) {
    let (node_conf, mut node_exec) = new_node_engine();
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
    ftype: i64, freq: f32, _res: f32) -> Vec<(u16, u32)>
{
    let sf = NodeId::SFilter(0);
    pset_d_wait(matrix, node_exec, sf, "freq", freq);
//    pset_d_wait(&mut matrix, &mut node_exec, sf, "freq", freq);
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
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 8000, 12000, 16000,
            22050,
        ], &fft[..]), vec![
            (0, 16), (100, 20), (250, 12), (500, 16), (750, 16), (1000, 16),
            (1500, 16), (2000, 16), (3000, 20), (4000, 16), (8000, 16), (12000, 16),
            (16000, 16),
        ]);

    // Low Pass @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 0, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            500, 1000, 1500, 2000, 8000, 12000, 16000, 20000
        ], &fft[..]), vec![
            (0, 0), (500, 0), (1000, 0),
            (1500, 0), (2000, 0), (8000, 0), (12000, 0), (16000, 0),
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
        avg_fft_freqs(4.0, &[
            100, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 8000, 12000, 16000,
            22050,
        ], &fft[..]), vec![
            (0, 16), (100, 20), (250, 12), (500, 16), (750, 16), (1000, 16),
            (1500, 16), (2000, 16), (3000, 20), (4000, 16), (8000, 16), (12000, 16),
            (16000, 16),
        ]);

    // Low Pass TPT @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 1, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            500, 1000, 1500, 2000, 8000, 12000, 16000, 20000
        ], &fft[..]), vec![
            (0, 0), (500, 0), (1000, 0),
            (1500, 0), (2000, 0), (8000, 0), (12000, 0), (16000, 0),
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
        avg_fft_freqs(4.0, &[
            500, 1000, 1500, 2000, 8000, 12000, 16000, 20000
        ], &fft[..]), vec![
            (0, 0), (500, 0), (1000, 0),
            (1500, 0), (2000, 4), (8000, 12), (12000, 12), (16000, 16),
        ]);

    // High Pass @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 2, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            10, 50, 100, 250, 500, 750, 1000, 1500, 2000, 3000, 8000, 12000
        ], &fft[..]), vec![
            (0, 12), (10, 24), (50, 24), (100, 20), (250, 12),
            (500, 16), (750, 20), (1000, 16), (1500, 16), (2000, 16),
            (3000, 20), (8000, 16),
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
        avg_fft_freqs(4.0, &[
            500, 1000, 1500, 2000, 8000, 12000, 16000, 20000
        ], &fft[..]), vec![
            (0, 0), (500, 0), (1000, 0),
            (1500, 0), (2000, 0), (8000, 0), (12000, 0), (16000, 0),
        ]);

    // High Pass TPT @ 0Hz
    let fft = fft_with_freq_res_type(&mut matrix, &mut node_exec, 3, 0.0, 0.0);
    assert_eq!(
        avg_fft_freqs(4.0, &[
            10, 50, 100, 250, 500, 750, 1000, 1500, 2000, 3000, 8000, 12000
        ], &fft[..]), vec![
            (0, 12), (10, 24), (50, 24), (100, 20), (250, 12),
            (500, 16), (750, 20), (1000, 16), (1500, 16), (2000, 16),
            (3000, 20), (8000, 16),
        ]);
}
