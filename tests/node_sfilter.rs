mod common;
use common::*;

#[test]
fn check_node_sfilter_compare() {
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
    pset_d(&mut matrix, sf, "freq", 440.0);
    matrix.sync().unwrap();

    let ta = std::time::Instant::now();
    let (out_l, out_r) = run_for_ms(&mut node_exec, 10000.0);
    let ta = std::time::Instant::now().duration_since(ta);
    println!("ta1 Elapsed: {:?}", ta);

    save_wav("check_noise_sfilt_in.wav", &out_r);
    save_wav("check_noise_sfilt1.wav", &out_l);

//    let fft = run_and_get_fft4096(&mut node_exec, 5, 1000.0);
//    for (fq, lvl) in fft {
//        println!("{:5}: {}", fq, lvl);
//    }

    pset_s(&mut matrix, sf, "ftype", 1);

    let ta = std::time::Instant::now();
    let (out_l, _) = run_for_ms(&mut node_exec, 10000.0);
    save_wav("check_noise_sfilt2.wav", &out_l);
    let ta = std::time::Instant::now().duration_since(ta);
    println!("ta2 Elapsed: {:?}", ta);

    pset_s(&mut matrix, sf, "ftype", 2);

    let ta = std::time::Instant::now();
    let (out_l, _) = run_for_ms(&mut node_exec, 10000.0);
    save_wav("check_noise_sfilt3.wav", &out_l);
    let ta = std::time::Instant::now().duration_since(ta);
    println!("ta3 Elapsed: {:?}", ta);

    pset_s(&mut matrix, sf, "ftype", 3);

    let ta = std::time::Instant::now();
    let (out_l, _) = run_for_ms(&mut node_exec, 10000.0);
    save_wav("check_noise_sfilt4.wav", &out_l);
    let ta = std::time::Instant::now().duration_since(ta);
    println!("ta4 Elapsed: {:?}", ta);

//    let fft = run_and_get_fft4096(&mut node_exec, 5, 1000.0);
//    for (fq, lvl) in fft {
//        println!("{:5}: {}", fq, lvl);
//    }

//    let fft = run_and_get_fft4096(&mut node_exec, 50, 1000.0);
//    assert!(fft.len() == 0);
    assert!(false);
}
