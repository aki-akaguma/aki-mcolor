/*!
the mark up text with color program.

# Features

- mark up text with color.
- minimum support rustc 1.65.0 (897e37553 2022-11-02)

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
///     "mcolor", ["-r", "Error", "-g", "Warn"]);
/// ```
///
pub fn execute<I, S>(sioe: &RunnelIoe, prog_name: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    execute_with_env(sioe, prog_name, args, vec![("", "")])
}

///
/// execute mcolor with environments
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "mcolor"
///   - args: parameter arguments.
///   - env: environments array.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_mcolor::execute_with_env(&RunnelIoeBuilder::new().build(),
///     "mcolor",
///     ["-r", "Error", "-g", "Warn"],
///     vec![
///         ("AKI_MCOLOR_COLOR_SEQ_RED_ST", "<R>"),
///         ("AKI_MCOLOR_COLOR_SEQ_GREEN_ST", "<G>"),
///         ("AKI_MCOLOR_COLOR_SEQ_BLUE_ST", "<B>"),
///         ("AKI_MCOLOR_COLOR_SEQ_CYAN_ST", "<C>"),
///         ("AKI_MCOLOR_COLOR_SEQ_MAGENDA_ST", "<M>"),
///         ("AKI_MCOLOR_COLOR_SEQ_YELLOW_ST", "<Y>"),
///         ("AKI_MCOLOR_COLOR_SEQ_ED","<E>"),
///     ]
/// );
/// ```
///
pub fn execute_with_env<I, S, IKV, K, V>(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: I,
    env: IKV,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
    IKV: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let env_cnf: conf::EnvConf = env.into();
    //
    match conf::parse_cmdopts(prog_name, &args_str) {
        Ok(conf) => run::run(sioe, &conf, &env_cnf),
        Err(errs) => {
            if let Some(err) = errs.iter().find(|e| e.is_help() || e.is_version()) {
                sioe.pg_out().write_line(err.to_string())?;
                Ok(())
            } else {
                Err(anyhow!("{errs}\n{TRY_HELP_MSG}"))
            }
        }
    }
}
