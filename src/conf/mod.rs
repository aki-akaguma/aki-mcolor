pub use self::parse::parse_cmdopts;

mod parse;

use regex::Regex;
use std::default::Default;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    None,
    Red,
    Green,
    Blue,
    Cyan,
    Magenda,
    Yellow,
}
impl ::std::default::Default for Color {
    fn default() -> Color {
        Color::None
    }
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

#[derive(Debug, Default)]
pub struct CmdOptConf {
    pub prog_name: String,
    //
    pub flg_help: bool,
    pub flg_version: bool,
    //
    pub opt_color_and_patterns: Vec<ColorAndPattern>,
    //
    pub opt_color_seq_red_start: String,
    pub opt_color_seq_green_start: String,
    pub opt_color_seq_blue_start: String,
    pub opt_color_seq_cyan_start: String,
    pub opt_color_seq_magenda_start: String,
    pub opt_color_seq_yellow_start: String,
    pub opt_color_seq_end: String,
    //
    pub arg_params: Vec<String>,
}

impl flood_tide::HelpVersion for CmdOptConf {
    fn is_help(&self) -> bool {
        self.flg_help
    }
    fn is_version(&self) -> bool {
        self.flg_version
    }
}

static COLOR_RED_START: &str = "\u{1B}[1;31m";
static COLOR_GREEN_START: &str = "\u{1B}[1;32m";
static COLOR_BLUE_START: &str = "\u{1B}[1;34m";
static COLOR_CYAN_START: &str = "\u{1B}[1;36m";
static COLOR_MAGENDA_START: &str = "\u{1B}[1;35m";
static COLOR_YELLOW_START: &str = "\u{1B}[1;33m";
static COLOR_END: &str = "\u{1B}[0m";

impl CmdOptConf {
    pub fn create(a_prog_name: &str) -> Self {
        //
        let a_color_red_start = match env::var("AKI_MCOLOR_RED_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_RED_START),
        };
        let a_color_green_start = match env::var("AKI_MCOLOR_GREEN_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_GREEN_START),
        };
        let a_color_blue_start = match env::var("AKI_MCOLOR_BLUE_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_BLUE_START),
        };
        let a_color_cyan_start = match env::var("AKI_MCOLOR_CYAN_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_CYAN_START),
        };
        let a_color_magenda_start = match env::var("AKI_MCOLOR_MAGENDA_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_MAGENDA_START),
        };
        let a_color_yellow_start = match env::var("AKI_MCOLOR_YELLOW_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_YELLOW_START),
        };
        let a_color_end = match env::var("AKI_MCOLOR_ED") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_END),
        };
        //
        Self {
            prog_name: a_prog_name.to_string(),
            opt_color_seq_red_start: a_color_red_start,
            opt_color_seq_green_start: a_color_green_start,
            opt_color_seq_blue_start: a_color_blue_start,
            opt_color_seq_cyan_start: a_color_cyan_start,
            opt_color_seq_magenda_start: a_color_magenda_start,
            opt_color_seq_yellow_start: a_color_yellow_start,
            opt_color_seq_end: a_color_end,
            //
            ..Default::default()
        }
    }
}
