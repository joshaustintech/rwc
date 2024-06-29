# rwc [![Rust](https://github.com/joshaustintech/rwc/actions/workflows/rust.yml/badge.svg)](https://github.com/joshaustintech/rwc/actions/workflows/rust.yml)
wc clone written in Rust

# usages
```
$ rwc [arguments...] [file]
$ cat [file] | rwc
```

# reading from stdin
If no file is provided as the final argument, `rwc` will read from `stdin`.

Example:
```
$ cat test.txt | rwc -l
    7145
```

## no arguments
If no arguments are provided, or only the file is provided, then `rwc` will
default to the equivalent arguments of `-c -l -w`.

Example:
```
$ rwc test.txt
7145    58164   342190  test.txt
```

## table of arguments
| argument  | description                                                       |
| --------  | -----------                                                       |
| -c        | prints number of bytes                                            |
| -l        | prints number of newlines, not including the end of the last line |
| -w        | prints number of words                                            |
| -m        | prints number of characters                                       |

# todo
- [ ] support for multiple files
- [ ] multithreading
