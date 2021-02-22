use crate::conf::{CmdOptConf, Color, ColorAndRegex};
use crate::util::err::BrokenPipeError;
use regex::Regex;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

/*
use regex::Regex;

use crate::conf::CmdOptConf;
use crate::conf::Color;
use crate::conf::ColorAndRegex;
//use crate::util::AppError;

use std::io;
use std::io::BufRead;
use std::io::Write;
*/

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let mut colregs: Vec<ColorAndRegex> = Vec::new();
    for colpat in &conf.opt_color_and_patterns {
        let re = Regex::new(&colpat.pattern)?;
        colregs.push(ColorAndRegex {
            color: colpat.color,
            regex: re,
        });
    }
    let r = do_match_proc(sioe, conf, &colregs);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn color_seq(conf: &CmdOptConf, color: Color) -> &str {
    match color {
        Color::None => "",
        Color::Red => conf.opt_color_seq_red_start.as_str(),
        Color::Green => conf.opt_color_seq_green_start.as_str(),
        Color::Blue => conf.opt_color_seq_blue_start.as_str(),
        Color::Cyan => conf.opt_color_seq_cyan_start.as_str(),
        Color::Magenda => conf.opt_color_seq_magenda_start.as_str(),
        Color::Yellow => conf.opt_color_seq_yellow_start.as_str(),
    }
}

fn do_match_proc(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    colregs: &[ColorAndRegex],
) -> anyhow::Result<()> {
    //let color_start_s = conf.opt_color_seq_red_start.as_str();
    let color_end_s = conf.opt_color_seq_end.as_str();
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
                        let color_start_s = color_seq(conf, color);
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
