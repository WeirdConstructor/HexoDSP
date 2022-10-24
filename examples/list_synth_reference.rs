use hexodsp::*;

fn format_param(pid: ParamId, val: f32) -> String {
    use std::io::Write;
    let mut buf: [u8; 100] = [0; 100];

    let len = {
        let mut bw = std::io::BufWriter::new(&mut buf as &mut [u8]);

        match pid.format(&mut bw, val) {
            Some(Ok(_)) => bw.buffer().len(),
            _ => 0,
        }
    };

    std::str::from_utf8(&buf[0..len]).unwrap_or("").to_string()
}

fn main() {
    for node_id in hexodsp::dsp::ALL_NODE_IDS.iter() {
        println!("NodeId::{}", node_id);
        let info = NodeInfo::from_node_id(*node_id);
        for in_idx in 0..info.in_count() {
            println!("   - input {}", node_id.inp_name_by_idx(in_idx as u8).unwrap());
            let param = node_id.inp_param_by_idx(in_idx).unwrap();
            if let Some(((min, max), _)) = param.param_min_max() {
                let mid = min + (max - min) * 0.5;
                let fmt_min = format_param(param, min);
                let fmt_mid = format_param(param, mid);
                let fmt_max = format_param(param, max);
                println!(
                    "     min = {:7.4}, denorm = {:9.2}, fmt = {}",
                    min,
                    param.denorm(min),
                    fmt_min
                );
                println!(
                    "     mid = {:7.4}, denorm = {:9.2}, fmt = {}",
                    mid,
                    param.denorm(mid),
                    fmt_mid
                );
                println!(
                    "     max = {:7.4}, denorm = {:9.2}, fmt = {}",
                    max,
                    param.denorm(max),
                    fmt_max
                );
            }
        }

        for at_idx in 0..info.at_count() {
            let param = node_id.atom_param_by_idx(at_idx).unwrap();
            println!("   - setting {}", node_id.inp_name_by_idx(at_idx as u8).unwrap());
            if let Some((min, max)) = param.setting_min_max() {
                if (max - min) > 7 {
                    println!("     min = {} = {}", min, format_param(param, min as f32));
                    println!("     max = {} = {}", max, format_param(param, max as f32));
                } else {
                    for i in min..=max {
                        println!("     {} = {}", i, format_param(param, i as f32));
                    }
                }
            }
        }

        for out_idx in 0..info.out_count() {
            println!("   - output {}", node_id.out_name_by_idx(out_idx as u8).unwrap());
        }
    }
}
