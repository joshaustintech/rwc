# rwc
wc clone written in Rust

# usage
$ rwc [arguments...] [file]

If no `file` is provided as the final argument, `rwc` will read from `stdin`
If no arguments are provided, or only the file is provided, then `rwc` will
default to the equivalent arguments of `-c -l -w`.

## arguments

# todo
- [ ] support for multiple files
- [ ] multithreading