# Firv Setup

## Prerequisites

* Ninja
* Make
* CMake
* RISC-V GNU Toolchain 

## Config

Copy `firv/config.toml` to root of the repository (`config.toml`)

## LLVM Project fork setup
```
git submodule sync
git submodule update --remote --init src/llvm-project
```

## Build rust
```
./x build
```

## Build RISC-V library
```
./x build library --target riscv64gc-unknown-none-elf
```

or

```
./x build library --target riscv32imac-unknown-none-elf
```

## Compile for RISC-V
```
$(RUSTC) <src> --target=riscv32imac-unknown-none-elf
```

## Resources

* https://rustc-dev-guide.rust-lang.org/backend/updating-llvm.html
* https://docs.rust-embedded.org/embedonomicon/preface.html
