# rignore

<img width="371" alt="image" src="https://user-images.githubusercontent.com/112097111/189715281-6287eecc-b939-4f5c-9e59-b1ce54af14fa.png">


### Generate .gitignore files using gitignore.io api

-----
First you need to build the binary


```bash

cargo build --release

```

-----

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
    help    Print this message or the help of the given subcommand(s)
    list    Lists suported languages

```

-----

For example:
```bash

rignore rust

```

If you don't provide a language it will print all the languages 
that gitignore.io supportes.

-----

But if you want to list by cli command you can use
```bash

rignore list

```
