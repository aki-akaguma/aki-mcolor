# aki-mcolor

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

the mark up text with color program.

## Features

- mark up text with color.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Command help

```
aki-mcolor --help
```

```
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

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-mcolor
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

### Command line example 1

Makes "`ca`" **red** and "`b`" **green** in standard input text.

```
echo "abcabca" | aki-mcolor -r "ca" -g "b"
```

result output :

![out abcabca image]

[out abcabca image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-abcabca-1.png


### Command line example 2

Extract "`arm`" from the rustup target list and make "`musl`" **green** and "`android`" **cyan**.

```
rustup target list | aki-mline -e arm | aki-mcolor -g "musl" -c "android"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-rustup-1.png

- [aki-mline](https://crates.io/crates/aki-mline): extract match line command like grep.

### Command line example 3

You can also multiple same color match.

```
echo "abcdefg" | aki-mcolor -r "c" -g "d" -r "e"
```

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-mcolor/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aki-mcolor.svg
[crate-link]: https://crates.io/crates/aki-mcolor
[docs-image]: https://docs.rs/aki-mcolor/badge.svg
[docs-link]: https://docs.rs/aki-mcolor/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
