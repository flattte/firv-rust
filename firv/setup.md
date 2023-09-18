# Firv Setup

Use the `config.toml` from this directory as a base

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

## Compile for RISC-V
```
$(RUSTC) <src> --target=riscv64gc-unknown-none-elf
```

## Resources

* https://rustc-dev-guide.rust-lang.org/backend/updating-llvm.html
* https://docs.rust-embedded.org/embedonomicon/preface.html
