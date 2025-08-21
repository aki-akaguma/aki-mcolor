use crate::conf::{CmdOptConf, Color, ColorAndRegex, EnvConf};
use crate::util::err::BrokenPipeError;
use regex::Regex;
use runnel::RunnelIoe;
//use std::io::Write;

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

#[derive(Debug)]
struct ColorSeq<'a> {
    env: &'a EnvConf,
}
impl<'a> ColorSeq<'a> {
    fn new(env: &'a EnvConf) -> Self {
        Self { env }
    }
    fn color_seq_start(&self, color: Color) -> &str {
        match color {
            Color::None => "",
            Color::Red => self.env.color_seq_red_start.as_str(),
            Color::Green => self.env.color_seq_green_start.as_str(),
            Color::Blue => self.env.color_seq_blue_start.as_str(),
            Color::Cyan => self.env.color_seq_cyan_start.as_str(),
            Color::Magenda => self.env.color_seq_magenda_start.as_str(),
            Color::Yellow => self.env.color_seq_yellow_start.as_str(),
        }
    }
    fn color_seq_end(&self) -> &str {
        self.env.color_seq_end.as_str()
    }
}

fn do_match_proc(
    sioe: &RunnelIoe,
    _conf: &CmdOptConf,
    env: &EnvConf,
    colregs: &[ColorAndRegex],
) -> anyhow::Result<()> {
    let color_seq = ColorSeq::new(env);
    for line in sioe.pg_in().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        //
        let (b_found, line_color_mark) = make_line_color_mark(colregs, line_ss)?;
        if b_found {
            let out_s = make_out_s(&color_seq, line_ss, &line_color_mark)?;
            sioe.pg_out().write_line(out_s)?;
        } else {
            sioe.pg_out().write_line(line_s)?;
        }
    }
    //
    sioe.pg_out().flush_line()?;
    Ok(())
}

fn make_line_color_mark(
    colregs: &[ColorAndRegex],
    line_ss: &str,
) -> anyhow::Result<(bool, Vec<Color>)> {
    let line_len: usize = line_ss.len();
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
            let (st, ed): (usize, usize) = match cap.get(usize::from(cap_len > 1)) {
                Some(m) => (m.start(), m.end()),
                None => (0, 0),
            };
            for m in line_color_mark.iter_mut().take(ed).skip(st) {
                *m = color;
            }
        }
    }
    Ok((b_found, line_color_mark))
}

fn make_out_s(
    color_seq: &ColorSeq,
    line_ss: &str,
    line_color_mark: &[Color],
) -> anyhow::Result<String> {
    /*
    let color_start_s = "<S>";
    let color_end_s = "<E>";
    */
    let color_end_s = color_seq.color_seq_end();
    let line_len: usize = line_ss.len();
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
                let color_start_s = color_seq.color_seq_start(color);
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
    Ok(out_s)
}
