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
objects, etc.).

[`lld`]: http://lld.llvm.org/

## Progress

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

### `lld`

To explore the suitability of `lld` as a (in-memory) Rust linker, I'll first try to use `lld` as a
external linker just like `cc` is used today. This is going to require modifying the linker
arguments that Rust passes to `cc`:

```
$ xargo rustc --target x86_64 --release -- -C linker=false
"false"
"-Tlayout.ld"
"-Wl,-n"
"-nostartfiles"
"-L" "/home/japaric/.xargo/lib/rustlib/x86_64/lib"
"/home/japaric/rust/eighty-six/target/x86_64/release/kernel.0.o"
"-o" "/home/japaric/rust/eighty-six/target/x86_64/release/kernel"
"-Wl,--gc-sections"
"-nodefaultlibs"
"-L" "/home/japaric/rust/eighty-six/target/x86_64/release"
"-L" "/home/japaric/rust/eighty-six/target/x86_64/release/deps"
"-L" "/home/japaric/rust/eighty-six/target/release/build/kernel-46b9540e8875ea07/out"
"-L" "/home/japaric/.xargo/lib/rustlib/x86_64/lib"
"-Wl,-Bstatic"
"-Wl,--whole-archive"
"-l" "boot"
"-Wl,--no-whole-archive"
"-Wl,-Bdynamic"
"/home/japaric/.xargo/lib/rustlib/x86_64/lib/libcore-1f21818448cb9abf.rlib"
```

A few arguments need to be dropped because they are (a) no longer necessary (`-nostartfiles`,
`-nodefaultlibs`) or (b) not implemented by `lld` (`-Wl,-n` -- note this flag doesn't come from
`rustc` itself; it's an arbitrary flag requested by the target specification). Also, `-Wl` needs to
be removed from the arguments that have it as a prefix. The rest of arguments can be passed as is:

```
$ ld.lld
"-Tlayout.ld"
"-L" "/home/japaric/.xargo/lib/rustlib/x86_64/lib"
"/home/japaric/rust/eighty-six/target/x86_64/release/kernel.0.o"
"-o" "/home/japaric/rust/eighty-six/target/x86_64/release/kernel"
"--gc-sections"
"-L" "/home/japaric/rust/eighty-six/target/x86_64/release"
"-L" "/home/japaric/rust/eighty-six/target/x86_64/release/deps"
"-L" "/home/japaric/rust/eighty-six/target/release/build/kernel-46b9540e8875ea07/out"
"-L" "/home/japaric/.xargo/lib/rustlib/x86_64/lib"
"-Bstatic"
"--whole-archive"
"-l" "boot"
"--no-whole-archive"
"-Bdynamic"
"/home/japaric/.xargo/lib/rustlib/x86_64/lib/libcore-1f21818448cb9abf.rlib"
```

Inspecting the resulting binary indicates that it's equivalent to the one produced by `cc`. The
binary also works as expected under QEMU.

The next step would be to add a e.g. `-Z use-lld` flag to `rustc` that (a) calls an external `lld`
to begin with or (b) calls `lld` as a library, but with the tweaked linker ags.

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
