use crate::conf::{CmdOptConf, Color, ColorAndRegex, EnvConf};
use crate::util::err::BrokenPipeError;
use regex::Regex;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

macro_rules! make_color_and_regex {
    ($vec:expr, $patterns:expr, $color:expr) => {{
        if !$patterns.is_empty() {
            for pattern in $patterns {
                let re = Regex::new(&pattern)?;
                $vec.push(ColorAndRegex {
                    color: $color,
                    regex: re,
                });
            }
        }
    }};
}

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf, env: &EnvConf) -> anyhow::Result<()> {
    let mut colregs: Vec<ColorAndRegex> = Vec::new();
    make_color_and_regex!(colregs, &conf.opt_red, Color::Red);
    make_color_and_regex!(colregs, &conf.opt_green, Color::Green);
    make_color_and_regex!(colregs, &conf.opt_blue, Color::Blue);
    make_color_and_regex!(colregs, &conf.opt_cyan, Color::Cyan);
    make_color_and_regex!(colregs, &conf.opt_magenda, Color::Magenda);
    make_color_and_regex!(colregs, &conf.opt_yellow, Color::Yellow);
    make_color_and_regex!(colregs, &conf.opt_unmark, Color::None);
    //
    let r = do_match_proc(sioe, conf, env, &colregs);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn color_seq(env: &EnvConf, color: Color) -> &str {
    match color {
        Color::None => "",
        Color::Red => env.color_seq_red_start.as_str(),
        Color::Green => env.color_seq_green_start.as_str(),
        Color::Blue => env.color_seq_blue_start.as_str(),
        Color::Cyan => env.color_seq_cyan_start.as_str(),
        Color::Magenda => env.color_seq_magenda_start.as_str(),
        Color::Yellow => env.color_seq_yellow_start.as_str(),
    }
}

fn do_match_proc(
    sioe: &RunnelIoe,
    _conf: &CmdOptConf,
    env: &EnvConf,
    colregs: &[ColorAndRegex],
) -> anyhow::Result<()> {
    //let color_start_s = env.color_seq_red_start.as_str();
    let color_end_s = env.color_seq_end.as_str();
    /*
    let color_start_s = "<S>";
    let color_end_s = "<E>";
    */
    //
    for line in sioe.pin().lock().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        let line_len: usize = line_ss.len();
        //
        let mut line_color_mark: Vec<Color> = Vec::with_capacity(line_len);
        line_color_mark.resize(line_len, Color::None);
        let mut b_found = false;
        //
        for colreg in colregs {
            let color = colreg.color;
            let re = &colreg.regex;
            for cap in re.captures_iter(line_ss) {
                b_found = true;
                //
                let cap_len = cap.len();
                let (st, ed): (usize, usize) = match cap.get(if cap_len > 1 { 1 } else { 0 }) {
                    Some(m) => (m.start(), m.end()),
                    None => (0, 0),
                };
                for m in line_color_mark.iter_mut().take(ed).skip(st) {
                    *m = color;
                }
            }
        }
        if b_found {
            //
            let mut out_s: String = String::new();
            let mut color = Color::None;
            let mut st: usize = 0;
            loop {
                let next_pos = match line_color_mark.iter().skip(st).position(|c| *c != color) {
                    Some(pos) => st + pos,
                    None => line_len,
                };
                if st != next_pos {
                    if color != Color::None {
                        let color_start_s = color_seq(env, color);
                        out_s.push_str(color_start_s);
                    }
                    out_s.push_str(&line_ss[st..next_pos]);
                    if color != Color::None {
                        out_s.push_str(color_end_s);
                    }
                }
                //
                if next_pos >= line_len {
                    break;
                }
                st = next_pos;
                color = line_color_mark[st];
            }
            //
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", out_s))?;
        } else {
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
        }
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
