//
//use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;
use flood_tide::{Arg, Lex, NameVal, Opt, OptNum};
use flood_tide::{OptParseError, OptParseErrors};

use crate::conf::CmdOptConf;
use crate::conf::Color;
use crate::conf::ColorAndPattern;

//----------------------------------------------------------------------
include!("cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
color marker by rust lang.
"#;
//const ARGUMENTS_TEXT: &str = r#""#;
const ENV_TEXT: &str = r#"Env:
  RUST_MCOLOR_RED_ST     red start sequence
  RUST_MCOLOR_GREEN_ST   greep start sequence
  RUST_MCOLOR_BLUE_ST    blue start sequence
  RUST_MCOLOR_CYAN_ST    cyan start sequence
  RUST_MCOLOR_MAGENDA_ST magenda start sequence
  RUST_MCOLOR_YELLOW_ST  yellow start sequence
  RUST_MCOLOR_ED         color end sequence
"#;
//const EXAMPLES_TEXT: &str = r#""#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options]")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message(env!("CARGO_PKG_NAME"));
    //[ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, ARGUMENTS_TEXT, ENV_TEXT, EXAMPLES_TEXT].join("\n")
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, ENV_TEXT].join("\n")
}

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    match CmdOp::from(nv.opt.num) {
        CmdOp::Red
        | CmdOp::Green
        | CmdOp::Blue
        | CmdOp::Cyan
        | CmdOp::Magenda
        | CmdOp::Yellow
        | CmdOp::Unmark => {
            let col = match CmdOp::from(nv.opt.num) {
                CmdOp::Red => Color::Red,
                CmdOp::Green => Color::Green,
                CmdOp::Blue => Color::Blue,
                CmdOp::Cyan => Color::Cyan,
                CmdOp::Magenda => Color::Magenda,
                CmdOp::Yellow => Color::Yellow,
                CmdOp::Unmark => Color::None,
                _ => unreachable!(),
            };
            let pat = value_to_string(nv)?;
            conf.opt_color_and_patterns.push(ColorAndPattern {
                color: col,
                pattern: pat,
            });
        }
        CmdOp::Help => {
            conf.flg_help = true;
        }
        CmdOp::Version => {
            conf.flg_version = true;
        }
    }
    //
    Ok(())
}

pub fn parse_my_style<'a, T, F>(
    conf: &mut T,
    opt_ary: &'a [Opt],
    sho_idx_ary: &'a [(u8, usize)],
    args: &'a [&'a str],
    parse_match: F,
) -> (Option<Vec<String>>, Result<(), OptParseErrors>)
where
    F: Fn(&mut T, &NameVal<'_>) -> Result<(), OptParseError>,
    T: HelpVersion,
{
    let lex = Lex::create_with(opt_ary, sho_idx_ary);
    let tokens = match lex.tokens_from(&args) {
        Ok(t) => t,
        Err(errs) => {
            return (None, Err(errs));
        }
    };
    //
    let mut errs = OptParseErrors::new();
    //
    for nv in tokens.namevals.iter() {
        match parse_match(conf, &nv) {
            Ok(_) => {}
            Err(err) => {
                errs.push(err);
            }
        }
        if conf.is_help() || conf.is_version() {
            break;
        }
    }
    //
    let mut v: Vec<String> = Vec::new();
    v.extend(tokens.free.iter().map(|&s| s.to_string()));
    //
    (Some(v), Err(errs))
}

pub fn parse_cmdopts(program: &str, args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf::create(program);
    let (opt_free, r_errs) =
        parse_my_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    if conf.is_help() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::help_message(&help_message(
            &conf.prog_name,
        )));
        return Err(errs);
    }
    if conf.is_version() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::version_message(&version_message(
            &conf.prog_name,
        )));
        return Err(errs);
    }
    //
    {
        let mut errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        //
        if conf.opt_color_and_patterns.is_empty() {
            errs.push(OptParseError::missing_option("r|g|b|c|m|y|u"));
        }
        //
        if let Some(free) = opt_free {
            if !free.is_empty() {
                errs.push(OptParseError::unexpected_argument(&free[0]));
            }
        };
        if !errs.is_empty() {
            return Err(errs);
        }
    }
    //
    Ok(conf)
}
