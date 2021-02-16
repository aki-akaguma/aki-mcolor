# aki-mcolor

*aki-mcolor* program is mark up text with color.

## Features

*aki-mcolor*  is mark up text with color.

* command help

```text
`aki-mcolor` -H
```

```text
Usage:
  aki-mcolor [options]

color marker by rust lang.

Options:
  -r, --red <exp>      regular expression, mark color is red
  -g, --green <exp>    regular expression, mark color is green
  -b, --blue <exp>     regular expression, mark color is blue
  -c, --cyan <exp>     regular expression, mark color is cyan
  -m, --magenda <exp>  regular expression, mark color is magenda
  -y, --yellow <exp>   regular expression, mark color is yellow
  -u, --unmark <exp>   regular expression, unmark color

  -H, --help     display this help and exit
  -V, --version  display version information and exit

Env:
  RUST_MCOLOR_RED_ST     red start sequence
  RUST_MCOLOR_GREEN_ST   greep start sequence
  RUST_MCOLOR_BLUE_ST    blue start sequence
  RUST_MCOLOR_CYAN_ST    cyan start sequence
  RUST_MCOLOR_MAGENDA_ST magenda start sequence
  RUST_MCOLOR_YELLOW_ST  yellow start sequence
  RUST_MCOLOR_ED         color end sequence
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

Makes `ca` red and `b` green in standard input text.

```
echo "abcabca" | aki-mcolor -r "ca" -g "b"
```

result output :

![out abcabca image]

[out abcabca image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-abcabca-1.png


#### Command line example 2

```
rustup target list | aki-mline -e arm | aki-mcolor -g "musl" -c "android"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mcolor/main/img/out-rustup-1.png


#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
