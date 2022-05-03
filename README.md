The
[RFC](https://github.com/rust-lang/rfcs/blob/master/text/2951-native-link-modifiers.md)
for https://github.com/rust-lang/rust/issues/81490 says that the default
mode for linking static libs is "--no-whole-archive". However, when
linking static libraries from build scripts, all links are done as
follows:

```
"-Wl,-Bstatic" "-Wl,--whole-archive" "-lmystatic" "-Wl,--no-whole-archive"
```

There is no way to opt out of this behavior as far as I can tell.

# Repro steps

1. Clone this repo
1. Run `RUSTC_LOG=rustc_codegen_ssa::back::link=info cargo build`
1. Notice the part of the linker line:
```
"-Wl,-Bstatic" "-Wl,--whole-archive" "-lmystatic" "-Wl,--no-whole-archive"
```

I think the RFC is incorrect in that the default behavior is to link
static libs as whole archives. I'm currently looking for a way to opt out
on a library-by-library basis.