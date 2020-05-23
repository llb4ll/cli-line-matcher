# cli-line-matcher
Small rust cli line matcher - mainly to play around with rust. It takes one file and finds all lines duplicated in other files.


linematcher 0.1.0

![Rust](https://github.com/llb4ll/cli-line-matcher/workflows/Rust/badge.svg?branch=master)

Matches lines in files. The first file is the reference file - this tools finds duplicate lines in the additional files and outputs them as csv.

USAGE:
    linematcher <paths>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <paths>...    A list of files to compare lines
