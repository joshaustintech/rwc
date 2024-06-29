# rwc
wc clone written in Rust

# usages
```
$ rwc [arguments...] [file]
$ cat [file] | rwc
```

If no `file` is provided as the final argument, `rwc` will read from `stdin`.

If no arguments are provided, or only the file is provided, then `rwc` will
default to the equivalent arguments of `-c -l -w`.

## arguments
| argument  | description                                                       |
| --------  | -----------                                                       |
| -c        | prints number of bytes                                            |
| -l        | prints number of newlines, not including the end of the last line |
| -w        | prints number of words                                            |
| -m        | prints number of characters                                       |


# todo
- [ ] support for multiple files
- [ ] multithreading