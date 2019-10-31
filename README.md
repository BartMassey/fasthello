# IO Buffering performance in Rust
Bart Massey

This code is originally by `/u/bruce3434` on this Reddit
[thread](https://www.reddit.com/r/rust/comments/dogxk8/why_does_buffering_the_already_buffered_stdout/).
The fundamental issue was that dropping a `BufWriter` on top
of `StdoutLocked` sped the code up by a factor of 2× even
though the writes contained no newlines. This Reddit
[comment](https://www.reddit.com/r/rust/comments/dogxk8/why_does_buffering_the_already_buffered_stdout/f5oxnlg?utm_source=share&utm_medium=web2x)
explains what is going on; this codebase is the underlying
code being measured.

* `glacial.rs` uses unlocked `Stdout`. This is really slow
  due to all the locking.

* `slow.rs` uses `StdoutLocked`. This is still pretty slow,
  for reasons explained in the comment above.

* `fast.rs` uses a `BufWriter` atop `StdoutLocked`. This is
  the version that is 2× faster than the slow version.

* `speedy.rs` uses a `BufWriter` atop a raw UNIX `File`. It
  is about 10% faster than the fast version, but is portable
  only to UNIX systems and has an `unsafe` in it.

* `turbo.c` is the original inspiration and currently the
  fastest, a C implementation authored by
  [Aleksi Lindeman](https://github.com/DEC05EBA). Its speedup
  tricks are used by the other fast versions here.

* `turbo.rs` is a fairly straightforward port of `turbo.c`,
  which avoids standard library routines for things in favor
  of hand-calculation. `turbo.rs` is about 60% faster than
  `speedy.rs`.
  
* `lightning.cpp` is a port of `turbo.rs` authored by
  [Aleksi Lindeman](https://github.com/DEC05EBA) and contributed by
  [Hossain Adnan](https://github.com/HossainAdnan). It uses
  a manual buffer. It is comparable in performance to
  `turbo.c`.

* `lightning.rs` is a port of `lightning.cpp` contributed by
  [Hossain Adnan](https://github.com/98982872). It
  uses a manual buffer currently backed by `std::Vec::<u8>`
  along with POSIX `write()`. It's about 30% slower than
  `turbo.rs`.

* `ludicrous.rs` is a version by
  [Aleksi Lindeman](https://github.com/DEC05EBA) that uses a
  handmade buffer. It is the same speed as `turbo.c`.

* `serious.rs` (not actually serious) is a C-like Rust
  implementation with tons of `unsafe` employing all the
  tricks. It is the same speed as `turbo.c`, which is
  reasonable given that it's even uglier and no safer.
  "You can write FORTRAN in any language."

* `mappy.rs` is a work-in-progress attempt to use
  memory-mapped I/O. It doesn't run yet.

Many of these will run only on a POSIX system. I have tried
them only on Linux.

Compiler choice matters for the faster C / C++ benchmarks
here. `clang` / `gcc` and `clang++` / `g++` will give
different answers. By default `clang` and `clang++` are used
to increase comparability with Rust's LLVM toolchain.

## Perfomance Comparison

* To run the benchmarks:

  * Install [Hyperfine](https://github.com/sharkdp/hyperfine)
    with `cargo install hyperfine`

  * Build the Rust benchmarks with `cargo build --release`

  * Say `make bench`

  The results will be available in `BENCH.md`. Here are
  [my results](BENCH.md) on a 3.5GHz i7-4770 ("Haswell").

* To check that the benchmarks produce the same output
  say `make check`. The `md5sum`s should match.
