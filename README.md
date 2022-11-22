# rignore

<img width="371" alt="image" src="https://user-images.githubusercontent.com/112097111/189715281-6287eecc-b939-4f5c-9e59-b1ce54af14fa.png">

[![Crates.io](https://img.shields.io/crates/v/rignore?style=flat-square)](https://crates.io/crates/rignore)
[![Crates.io](https://img.shields.io/crates/d/rignore?style=flat-square)](https://crates.io/crates/rignore)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

### TODO:

Add cache support ✅\
Orginize the code ✅

### Generate .gitignore files using [gitignore.io](https://gitignore.io) api

---

First you need to build the binary

```bash

cargo build --release

```

or

```bash

cargo install rignore

```

---

```bash

rignore -help

USAGE:
    rignore [LANGUAGE] [SUBCOMMAND]

ARGS:
    <LANGUAGE>    Dowloads the chosen language from the api

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    clear    Clears the cache
    help     Print this message or the help of the given subcommand(s)
    list     Lists suported languages

```

---

For example:

```bash

rignore rust

```

If you don't provide a language it will print all the languages
that gitignore.io supportes.

---

But if you want to list by cli command you can use

```bash

rignore list

```

---

You can clear the cache with clear command

```bash

rignore clear

```
