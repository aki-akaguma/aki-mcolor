//!
//! the substitude text program.
//!
//! ```text
//! Usage:
//!   aki-mcolor [options]
//! 
//! color marker by rust lang.
//! 
//! Options:
//!   -r, --red <exp>      regular expression, mark color is red
//!   -g, --green <exp>    regular expression, mark color is green
//!   -b, --blue <exp>     regular expression, mark color is blue
//!   -c, --cyan <exp>     regular expression, mark color is cyan
//!   -m, --magenda <exp>  regular expression, mark color is magenda
//!   -y, --yellow <exp>   regular expression, mark color is yellow
//!   -u, --unmark <exp>   regular expression, unmark color
//! 
//!   -H, --help     display this help and exit
//!   -V, --version  display version information and exit
//! 
//! Env:
//!   RUST_MCOLOR_RED_ST     red start sequence
//!   RUST_MCOLOR_GREEN_ST   greep start sequence
//!   RUST_MCOLOR_BLUE_ST    blue start sequence
//!   RUST_MCOLOR_CYAN_ST    cyan start sequence
//!   RUST_MCOLOR_MAGENDA_ST magenda start sequence
//!   RUST_MCOLOR_YELLOW_ST  yellow start sequence
//!   RUST_MCOLOR_ED         color end sequence
//! ```
//!

#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
/// execute mcolor
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "gsub"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```
/// use runnel::medium::stdioe::{StreamInStdin,StreamOutStdout,StreamErrStderr};
/// use runnel::StreamIoe;
///
/// let r = libaki_mcolor::execute(&StreamIoe{
///     sin: Box::new(StreamInStdin::default()),
///     sout: Box::new(StreamOutStdout::default()),
///     serr: Box::new(StreamErrStderr::default()),
/// }, "mcolor", &["-r", "Error", "-g", "Warn"]);
/// ```
///
pub fn execute(sioe: &StreamIoe, program: &str, args: &[&str]) -> anyhow::Result<()> {
    //
    let conf = match conf::parse_cmdopts(program, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.sout.lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
