/*!
the mark up text with color program.

# Features

- mark up text with color.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

# Command help

```text
aki-mcolor --help
```

```text
Usage:
  aki-mcolor [options]

mark up text with color

Options:
  -r, --red <exp>       write it in red
  -g, --green <exp>     write it in green
  -b, --blue <exp>      write it in blue
  -c, --cyan <exp>      write it in cyan
  -m, --magenda <exp>   write it in magenda
  -y, --yellow <exp>    write it in yellow
  -u, --unmark <exp>    write it in non-color

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Option Parameters:
  <exp>     regular expression, color the entire match.

Environments:
  AKI_MCOLOR_COLOR_SEQ_RED_ST       red start sequence specified by ansi
  AKI_MCOLOR_COLOR_SEQ_GREEN_ST     greep start sequence specified by ansi
  AKI_MCOLOR_COLOR_SEQ_BLUE_ST      blue start sequence specified by ansi
  AKI_MCOLOR_COLOR_SEQ_CYAN_ST      cyan start sequence specified by ansi
  AKI_MCOLOR_COLOR_SEQ_MAGENDA_ST   magenda start sequence specified by ansi
  AKI_MCOLOR_COLOR_SEQ_YELLOW_ST    yellow start sequence specified by ansi
  AKI_MCOLOR_COLOR_SEQ_ED           color end sequence specified by ansi
```

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-mcolor
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

## Command line example 1

Makes "`ca`" **red** and "`b`" **green** in standard input text.

```text
echo "abcabca" | aki-mcolor -r "ca" -g "b"
```

result output :

![out abcabca image]

[out abcabca image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-abcabca-1.png


## Command line example 2

Extract "`arm`" from the rustup target list and make "`musl`" **green** and "`android`" **cyan**.

```text
rustup target list | aki-mline -e arm | aki-mcolor -g "musl" -c "android"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-rustup-1.png

- [aki-mline](https://crates.io/crates/aki-mline): extract match line command like grep.

## Command line example 3

You can also multiple same color match.

```text
echo "abcdefg" | aki-mcolor -r "c" -g "d" -r "e"
```

# Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

*/
#[macro_use]
extern crate anyhow;

pub mod conf;
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
    let env = conf::EnvConf::new();
    execute_env(sioe, prog_name, args, &env)
}

pub fn execute_env(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: &[&str],
    env: &conf::EnvConf,
) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{err}\n"));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf, env)
}
