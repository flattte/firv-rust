# FIRV HARDEN

Attribute with similar approach present: `#[cold]`

## Required changes
* compiler\rustc_span\src\symbol.rs => `Symbols`
    Definition of `firv_harden` token

* `compiler\rustc_middle\src\middle\codegen_fn_attrs.rs` => `CodegenFnAttrFlags`
    Codegen Attribute Flag definition

* `compiler\rustc_codegen_ssa\src\codegen_attrs.rs` => `codegen_fn_attrs`
    Symbol to Codegen Flag

* `compiler\rustc_codegen_llvm\src\attributes.rs` => `from_fn_attrs`
    Flag to LLVM Attribute

* `compiler\rustc_feature\src\builtin_attrs.rs` => `BUILTIN_ATTRIBUTES`
    Declaration of builtin attribute

### Changes for the support tools

* `src\tools\rust-analyzer\crates\hir-def\src\attr\builtin.rs` => `INERT_ATTRIBUTES`
    Declaration of builtin attribute (why is that a copy of above?)

* `src\tools\rust-analyzer\crates\ide-completion\src\completions\attribute.rs` => `KIND_TO_ATTRIBUTES`, `ATTRIBUTES`
    Declaration of attributes (possibly can be skipped)
