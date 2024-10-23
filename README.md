# DICWORD

A command line tool that generates a random dictionary word of arbitrary length.


Usage:

    Usage: dicword.exe [OPTIONS]

    Options:
    -l, --length <LENGTH>  Length of the word [default: 5]
    -h, --help             Print help
    -V, --version          Print version

### Limitations

Cannot generate words longer than 15 characters.

## Installation

Install via `cargo`:

     cargo install https://github.com/maciakl/dicword/ 
 
 On Windows, this tool is also distributed via `scoop` (see [scoop.sh](https://scoop.sh)).

 First, you need to add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket
    scoop update

 Next simply run:
 
    scoop install dicword

