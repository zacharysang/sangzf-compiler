# Overview

This is a repo containing source code for a compiler written for a compilers class I took in the spring of 2019.
Given a source file written in a language specific to this class, the compiler will (eventually) output an executable program.

# Work done and work remaining
- [x] Lexer
- [x] Parser
- [ ] Type-checker
- [ ] Code generation
- [ ] Runtime

# Setting up the project

## Installing rust
This project is written in rust and so to build it, you'll need to install some tooling.
[Here](https://www.rust-lang.org/tools/install) is an authoritative source on the instructions if more information is needed.

1. Install rustup (rust's installer tool) using the following command: `curl https://sh.rustup.rs -sSf | sh`
2. Update rust by running the following command (rustc/cargo version 1.33.0 is what's used for this project): `rustup install 1.33.0`
3. Add to PATH env var by running the following/ adding it to your ~/.bashrc file: `export PATH="$HOME/.cargo/bin:${PATH}"`

## Building and running
After you have cloned this repo, following the following steps to build and run the compiler:
1. Make sure you are in the directory of the project (the folder you cloned)
2. Build and run using `cargo run <filename>`, where filename is the path to the source file you're using relative to your current directory.
  * Note: For my own development purposes, if no source file is selected `sample_programs/correct/source.src` is used by default.

## Uninstalling rust
If you'd like to remove rust from your machine, simply run: `rustup self uninstall`.


# Reflections on the project
