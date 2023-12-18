# Your case-changing companion!

## How to install

 - Make sure you have Rust installed. See [rustup.rs](https://rustup.rs)
 - Run `cargo install --git https://github.com/peterwilli/ToCase.git`
 - That's it!

## How to use

Case can be selected in just 2 or 3 keystrokes.

### In Helix

[![asciicast](https://asciinema.org/a/eyyFL9yhcI8WrvbbofFwjbnRc.svg)](https://asciinema.org/a/eyyFL9yhcI8WrvbbofFwjbnRc)

- Just select a word, type `|to_case cC` (or any other case you wish to use)

### As CLI

-  For camelCase, just run `to_case cC -w "test_word"`

```
To Case is a command line case-changing tool especially designed for the Helix Editor

Usage: to_case [OPTIONS] [CASE]

Arguments:
  [CASE]  Case name (cC, c_c, CC, cc) (if not defined, the last choice will be used)

Options:
  -w, --word <WORD>  Word to change (taken from stdin if not defined)
  -h, --help         Print help
  -V, --version      Print version
```
