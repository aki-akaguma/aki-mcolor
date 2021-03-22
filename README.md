# aki-mcolor

*aki-mcolor* program is mark up text with color.

## Features

*aki-mcolor*  is mark up text with color.

* command help

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

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-mcolor
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.


## Examples

#### Command line example 1

Makes "`ca`" **red** and "`b`" **green** in standard input text.

```
echo "abcabca" | aki-mcolor -r "ca" -g "b"
```

result output :

![out abcabca image]

[out abcabca image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-abcabca-1.png


#### Command line example 2

Extract "`arm`" from the rustup target list and make "`musl`" **green** and "`android`" **cyan**.

```
rustup target list | aki-mline -e arm | aki-mcolor -g "musl" -c "android"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-rustup-1.png

- [aki-mline](https://crates.io/crates/aki-mline): extract match line command like grep.

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-mcolor/blob/main/CHANGELOG.md)
