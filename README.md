# aki-mcolor

*aki-mcolor* program is mark up text with color.

## Features

*aki-mcolor*  is mark up text with color.

* example

command:
```
`aki-mcolor` -H
```

* minimum support rustc 1.38.0

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

command line:
```
cat text-file | aki-mcolor -r "^Error" -g "Ok\(.*\)"
```

command line:
```text
echo "abcabca" | aki-mcolor -r "ca"
```

result output:

ab<span style="color:red;">ca</span>b<span style="color:red;">ca</span>

