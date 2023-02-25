# Presentation

This is a summary of the basics needed to understand and use the Rust language. If you master this whole cheat sheet, you will have all the tools you need to write most of the programs you will be asked to write.

This cheat sheet is meant to be compiled and run. You can modify it to do your own tests and confirm or deny what you think you have understood.


# Prerequisites

You must have `rustup` installed. It is the Rust toolchain manager.  
This tool chain comes with :
- `rustc`: the rust compiler (equivalent to `gcc`)
- `cargo`: the rust project manager (equivalent to `npm`)

## To check if rustup is installed

Type the command `rustup --version`  
If you have an output that looks like : `rustup 1.24.3 (ce5817a94 2021-05-31)` you can skip to the last section.


## To Install rustup

### Macos and Linux

Type the command:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`


### Windows

Type the command:
```sh
curl 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe' --output rustup-init.exe && chmod +x rustup-init.exe && ./rustup-init.exe --default-host=x86_64-pc-windows-gnu
```


## Update your rust version

To update your rust version (and every tools in the rust toolchain) type:
`rustup update`



# Cargo

`cargo` is a tool similar to `npm`. It serves as both a build manager (e.g. Makefile, CMake; something that helps turn your code into a runnable program) and a package manager (e.g. `apt`, `brew`; something that gets and updates software packages online for you).

`cargo` needs a file called `Cargo.toml` to run. This file describes the **features** and **dependencies** of the project. It must be present in the root of every Rust project.

NB: Rust projects are also called `crates`.

## Cargo commands
- `cargo --version`: check the version of cargo
- `cargo new [name]`: create a new rust projet (useful for tests)
- `cargo build`: compile the default target (based on your file tree and you `Cargo.toml`)
- `cargo run`: run the default target (based on your file tree and you `Cargo.toml`)
- `cargo clean`: remove the `target` directory which contains every compiled files.


# To compile and launch the cheat sheet

   - open a terminal

   - go to the `rust_cheat_sheet` folder

   - type the command `cargo run`
