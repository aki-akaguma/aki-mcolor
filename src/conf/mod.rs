pub use self::parse::parse_cmdopts;
use crate::util::OptUcXParam;
pub use parse::CmdOptConf;

mod parse;

impl CmdOptConf {
    /*
    pub fn base_dir(&self) -> String {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::BaseDir(s) = o {
                return s.clone();
            }
        }
        String::new()
    }
    */
    pub fn is_opt_uc_x_help(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::Help = o {
                return true;
            }
        }
        false
    }
    pub fn is_opt_uc_x_package_version_info(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::RustVersionInfo = o {
                return true;
            }
        }
        false
    }
}

use regex::Regex;
use std::default::Default;
use std::env;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    #[default]
    None,
    Red,
    Green,
    Blue,
    Cyan,
    Magenda,
    Yellow,
}

#[derive(Debug, Default)]
pub struct ColorAndPattern {
    pub color: Color,
    pub pattern: String,
}

#[derive(Debug)]
pub struct ColorAndRegex {
    pub color: Color,
    pub regex: Regex,
}

//
// ref.) 3-bit and 4-bit color sequence
//   https://en.wikipedia.org/wiki/ANSI_escape_code
// * black letters on white background use: ESC[30;47m
// * red use: ESC[31m
// * bright red use: ESC[1;31m
// * reset colors to their defaults: ESC[39;49m (not supported on some terminals)
// * reset all attributes: ESC[0m
//
static COLOR_RED_START: &str = "\u{1B}[1;31m";
static COLOR_GREEN_START: &str = "\u{1B}[1;32m";
static COLOR_BLUE_START: &str = "\u{1B}[1;34m";
static COLOR_CYAN_START: &str = "\u{1B}[1;36m";
static COLOR_MAGENDA_START: &str = "\u{1B}[1;35m";
static COLOR_YELLOW_START: &str = "\u{1B}[1;33m";
static COLOR_END: &str = "\u{1B}[0m";

#[derive(Debug)]
pub struct EnvConf {
    pub color_seq_red_start: String,
    pub color_seq_green_start: String,
    pub color_seq_blue_start: String,
    pub color_seq_cyan_start: String,
    pub color_seq_magenda_start: String,
    pub color_seq_yellow_start: String,
    pub color_seq_end: String,
}
impl EnvConf {
    pub fn new() -> Self {
        //
        let a_color_red_start = match env::var("AKI_MCOLOR_COLOR_SEQ_RED_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_RED_START),
        };
        let a_color_green_start = match env::var("AKI_MCOLOR_COLOR_SEQ_GREEN_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_GREEN_START),
        };
        let a_color_blue_start = match env::var("AKI_MCOLOR_COLOR_SEQ_BLUE_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_BLUE_START),
        };
        let a_color_cyan_start = match env::var("AKI_MCOLOR_COLOR_SEQ_CYAN_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_CYAN_START),
        };
        let a_color_magenda_start = match env::var("AKI_MCOLOR_COLOR_SEQ_MAGENDA_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_MAGENDA_START),
        };
        let a_color_yellow_start = match env::var("AKI_MCOLOR_COLOR_SEQ_YELLOW_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_YELLOW_START),
        };
        let a_color_end = match env::var("AKI_MCOLOR_COLOR_SEQ_ED") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_END),
        };
        //
        Self {
            color_seq_red_start: a_color_red_start,
            color_seq_green_start: a_color_green_start,
            color_seq_blue_start: a_color_blue_start,
            color_seq_cyan_start: a_color_cyan_start,
            color_seq_magenda_start: a_color_magenda_start,
            color_seq_yellow_start: a_color_yellow_start,
            color_seq_end: a_color_end,
        }
    }
}
impl std::default::Default for EnvConf {
    fn default() -> EnvConf {
        EnvConf::new()
    }
}
