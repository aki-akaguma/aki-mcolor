//!
//! the mark up text with color program.
//!
//! ```text
//! Usage:
//!   aki-mcolor [options]
//!
//! mark up text with color
//!
//! Options:
//!   -r, --red <exp>       regular expression, mark color is red
//!   -g, --green <exp>     regular expression, mark color is green
//!   -b, --blue <exp>      regular expression, mark color is blue
//!   -c, --cyan <exp>      regular expression, mark color is cyan
//!   -m, --magenda <exp>   regular expression, mark color is magenda
//!   -y, --yellow <exp>    regular expression, mark color is yellow
//!   -u, --unmark <exp>    regular expression, unmark color
//!
//!   -H, --help        display this help and exit
//!   -V, --version     display version information and exit
//!
//! Env:
//!   AKI_MCOLOR_RED_ST         red start sequence
//!   AKI_MCOLOR_GREEN_ST       greep start sequence
//!   AKI_MCOLOR_BLUE_ST        blue start sequence
//!   AKI_MCOLOR_CYAN_ST        cyan start sequence
//!   AKI_MCOLOR_MAGENDA_ST     magenda start sequence
//!   AKI_MCOLOR_YELLOW_ST      yellow start sequence
//!   AKI_MCOLOR_ED             color end sequence
//! ```
//!
//! # Examples
//!
//! ### Command line example 1
//!
//! Makes "`ca`" **red** and "`b`" **green** in standard input text.
//!
//! ```text
//! echo "abcabca" | aki-mcolor -r "ca" -g "b"
//! ```
//!
//! result output :
//!
//! ![out abcabca image]
//!
//! [out abcabca image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-abcabca-1.png
//!
//!
//! ### Command line example 2
//!
//! Extract "`arm`" from the rustup target list and make "`musl`" **green** and "`android`" **cyan**.
//!
//! ```text
//! rustup target list | aki-mline -e arm | aki-mcolor -g "musl" -c "android"
//! ```
//!
//! result output :
//!
//! ![out rustup image]
//!
//! [out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-rustup-1.png
//!
//! - [aki-mline](https://crates.io/crates/aki-mline): extract match line command like grep.
//!
//! ### Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute
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
///   - program: program name. etc. "mcolor"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_mcolor::execute(&RunnelIoeBuilder::new().build(),
///     "mcolor", &["-r", "Error", "-g", "Warn"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
