[![Build Status][status]](https://travis-ci.org/japaric/eighty-six)

[status]: https://travis-ci.org/japaric/eighty-six.svg?branch=master

# `eighty-six`

> x86 bare metal Rust thing

I'm following the [intermezzOS] book to get a feel about how Rust can be used for x86 kernel
development. I'm particularly interested in studying the tooling involved as I heard that developers
make use of Makefiles and external assembly files to build their kernels, and I feel that neither
(Makefiles or external assembly files) should be necessary if one uses Cargo to build the kernel
(but I might be wrong).

[intermezzOS]: http://intermezzos.github.io/book

I have another goal: I'd love the replace `rustc` dependence on an external linker (usually `gcc`)
with a [`lld`] that's embedded in `rustc` itself. The x86 kernel dev space is an excellent testing
ground for this as its "executables" (the kernels) are free of C dependencies (libraries, startup
objects, etc.). Check the [lld branch] for more details.

[`lld`]: http://lld.llvm.org/
[lld branch]: https://github.com/japaric/eighty-six/tree/lld#lld

## Progress

This section is written in journal style and in chronological order.

### Chapter 3

I've just finished [Chapter 3], and my "kernel" prints "Hello, world!" to the screen and does
nothing more :tada:. Some relevant differences between my build process and intermezzOS':

[Chapter 3]: http://intermezzos.github.io/book/booting-up.html

```
# Build the kernel
$ xargo build --target x86_64
```

- No external assembly files. The multiboot header (`multiboot_header.asm`) has been fully
  implemented in the linker script. And `boot.asm` has been implemented as Rust code
  (`src/main.rs`).
- There's only a single line of assembly: `asm!("hlt")`, which can't be emitted using pure Rust code
  (AFAIK).
- `core` is not explicitly build. [Xargo] takes care of compiling it without user intervention.

[Xargo]: https://crates.io/crates/xargo

### Chapter 4

We are now in long mode :tada:. Notable changes:

- We were always emitting 64-bit instructions even though we are only supposed to use 32-bit
  instructions because we start in restricted mode :scream_cat:. This has been fixed by changing the
  target specification from a 64-bit target (`x86_64.json`) to a 32-bit one (`x86.json`).
- All the data layout related stuff that intermezzos does in assembly has been implemented in the
  linker script to reduce the amount of assembly. Notably, the layout of the page tables and the GDT
  are done in the linker script.
- Linking the page tables is done in pure Rust :+1;.
- Actually entering long mode requires manipulation of (control) registers and those can't be
  accessed via pure Rust, AFAIK. For that reason, this part is written in inline assembly.
- Caveat: Because we are telling `rustc` to use a 32-bit target, `rustc` always emit 32-bit
  instructions and we can't actually use 64-bit instructions/registers in the section of the program
  where the CPU is already in long mode. Yikes, I'll have to think about how to solve this.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
