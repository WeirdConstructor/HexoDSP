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

fn reformat_inline_desc(s: &str, title: bool) -> String {
    let mut first = true;
    let mut out = vec![];
    for line in s.split("\n") {
        if title && first {
            out.push(format!("**{}**: ", line.replace("~~", "`")));
            first = false;
        } else {
            out.push(line.replace("~~", "`"));
        }
    }
    out.join(" ")
}

fn reformat_desc(s: &str, title: bool) -> String {
    let mut first = true;
    let mut out = vec![];
    for line in s.split("\n") {
        if title && first {
            out.push(format!("**{}**", line));
            first = false;
        } else {
            out.push(line.replace("~~", "`"));
        }
    }
    out.join("\n")
}

fn main() {
    let mut node_ids = hexodsp::dsp::ALL_NODE_IDS.to_vec();
    node_ids.sort_by(|a, b| {
        if a.ui_category() == b.ui_category() {
            a.name().cmp(b.name())
        } else {
            a.ui_category().cmp(&b.ui_category())
        }
    });

    println!("| Category | Dscription |");
    println!("|----------|------------|");
    println!("| Osc | Oscillators / Signal generators (sine oscillator, noise generator, ...) |");
    println!("| Mod | Modulation Signal generators (envelope generators, sequencers, ...) |");
    println!("| NtoM | Signal routing, routing N signals to M outputs (mixer, router, ...) |");
    println!("| Signal | Signal filtering and shaping (filters, delays, reverbs, ...) |");
    println!("| Ctrl | Control signal generators and shapers (for instance quantizers) |");
    println!("| IOUtil | Input, output and utility nodes (audio output, feedback nodes, MIDI CC, ...) |");
    println!("");

    println!("| Node | Category | Description |");
    println!("|-|-|-|");
    for node_id in node_ids.iter() {
        let info = NodeInfo::from_node_id(*node_id);

        println!(
            "| [**{}**](#nodeid{}) | {} | {} |",
            node_id.label(),
            node_id.name(),
            node_id.ui_category().to_str(),
            reformat_inline_desc(info.desc(), true)
        );
    }

    for node_id in node_ids.iter() {
        let info = NodeInfo::from_node_id(*node_id);

        println!("### NodeId::{}", node_id.label());
        println!("{}", reformat_desc(info.desc(), true));

        for in_idx in 0..info.in_count() {
            let param = node_id.inp_param_by_idx(in_idx).unwrap();
            println!(
                "- [input **{}**](#nodeid{}-input-{}) - {}",
                param.name(),
                node_id.name(),
                param.name(),
                reformat_inline_desc(info.in_help(in_idx).unwrap_or(""), false)
            );
        }

        for at_idx in 0..info.at_count() {
            let param = node_id.atom_param_by_idx(at_idx).unwrap();
            println!(
                "- [setting **{}**](#nodeid{}-setting-{}) - {}",
                param.name(),
                node_id.name(),
                param.name(),
                reformat_inline_desc(info.in_help(param.inp() as usize).unwrap_or(""), false)
            );
        }

        for out_idx in 0..info.out_count() {
            println!("- output **{}**", node_id.out_name_by_idx(out_idx as u8).unwrap());
            println!("{}", reformat_inline_desc(info.out_help(out_idx).unwrap_or(""), false));
            println!(
                " `{}(0).output().{}()`",
                node_id.name(),
                node_id.out_name_by_idx(out_idx as u8).unwrap()
            );
        }

        println!("#### NodeId::{} Help", node_id.label());
        println!("{}", reformat_desc(info.help(), true));

        for in_idx in 0..info.in_count() {
            let param = node_id.inp_param_by_idx(in_idx).unwrap();
            println!("#### NodeId::{} input {}", node_id.label(), param.name());
            println!("{}", reformat_desc(info.in_help(in_idx).unwrap_or(""), false));
            println!("");
            println!("API example for connecting the input:");
            println!("`{}(0).input().{}(&amp(1).output().sig())`", node_id.name(), param.name());
            println!("");

            println!("| | value | denormalized | fmt | build API | [crate::ParamId] |");
            println!("|-|-------|--------------|-----|-----------|------------------|");
            if let Some(((min, max), _)) = param.param_min_max() {
                let default = param.norm_def();
                let mid = min + (max - min) * 0.5;
                for (name, val) in [("default", default), ("min", min), ("mid", mid), ("max", max)]
                {
                    let fmt = format_param(param, val);
                    println!(
                        "| **{}** | {:7.4} | {:9.2} | {} | `{}(0).set().{}({})` | \
                        `NodeId::{}(0).inp_param(\"{}\")` |",
                        name,
                        val,
                        param.denorm(val),
                        fmt,
                        node_id.name(),
                        param.name(),
                        param.denorm(val),
                        node_id.label(),
                        param.name(),
                    );
                }
            }
        }

        for at_idx in 0..info.at_count() {
            let param = node_id.atom_param_by_idx(at_idx).unwrap();

            println!("#### NodeId::{} setting {}", node_id.label(), param.name(),);
            println!("{}", reformat_desc(info.in_help(param.inp() as usize).unwrap_or(""), false));
            println!("");

            println!("| setting | fmt | build API | [crate::ParamId] |");
            println!("|---------|-----|-----------|------------------|");
            if let Some((min, max)) = param.setting_min_max() {
                for i in min..=max {
                    println!(
                        "| {} | {} | `{}(0).set().{}({})` | \
                        `NodeId::{}(0).inp_param(\"{}\")` |",
                        i,
                        format_param(param, i as f32),
                        node_id.name(),
                        param.name(),
                        i,
                        node_id.label(),
                        param.name()
                    );
                }
            }
        }
    }
}
