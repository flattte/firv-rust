# Firv Setup

## Prerequisites

The build process has been tested on machines running Windows 10 and Ubuntu 22.04. In the case of Windows environment the dependencies were installed primarily using `chocolatey` package manager, while in Ubuntu `apt` was used. 

### Environment requirements
* 30GB+ of disk space ([rust-dev-guide](https://rustc-dev-guide.rust-lang.org/building/prerequisites.html)), FIRV authors recommend having around 50GB of free space
* 8GB+ RAM
* 2+ CPU cores

### Base Rust and LLVM prerequisites
This list has been prepared based on Rust project's [README.md](../README.md).

* `g++`, `clang++`, or MSVC (consult [README.md](../README.md) for version constraints)
* `python` 3 or 2.7
* `git`
* `ninja` or GNU `make` (LLVM documentation heavily suggests using `ninja`, especially in Windows-based environments)
* `cmake`

For Linux environments following dependencies might be required:
* `curl` (not needed on Windows)
* `pkg-config` if you are compiling on Linux and targeting Linux
* `libiconv` (already included with glibc on Debian-based distros)

### RISC-V related prerequisites
* RISC-V GNU Toolchain, e.g.
    + Ubuntu `gcc-riscv64-linux-gnu`
    + Windows [gnutoolchains.com](https://gnutoolchains.com/risc-v/)

## Rust build config

Copy [`firv/config.toml`](./config.toml) to the root of the repository (`config.toml`)

## LLVM Project fork setup
```
git submodule sync
git submodule update --remote --init src/llvm-project
```

## Firv-Rust Build process

### Build Rust Compiler
```
./x build
```

### Build RISC-V library
```
./x build library --target riscv64gc-unknown-none-elf
```

## Example usage

`$(RUSTC)` is the path to the built host `rustc` binary, e.g.
```
build/host/stage1/bin/rustc
```

### Compile for RISC-V
```
$(RUSTC) <src> --target=riscv64gc-unknown-none-elf
```

## Resources

* https://rustc-dev-guide.rust-lang.org/backend/updating-llvm.html
* https://docs.rust-embedded.org/embedonomicon/preface.html
