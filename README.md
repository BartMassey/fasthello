# IO Buffering performance in Rust
Bart Massey

This code is originally by `/u/bruce3434` on this Reddit
[thread](https://www.reddit.com/r/rust/comments/dogxk8/why_does_buffering_the_already_buffered_stdout/). The
fundamental issue was that dropping a `BufWriter` on top of
`StdoutLocked` sped the code up by a factor of 2× even
though the writes contained no newlines. This Reddit
[comment](https://www.reddit.com/r/rust/comments/dogxk8/why_does_buffering_the_already_buffered_stdout/f5oxnlg?utm_source=share&utm_medium=web2x)
explains what is going on; this codebase is the underlying
code being measured.

* `glacial.rs` uses unlocked `Stdout`. This is pretty slow
  due to all the locking.

* `slow.rs` uses `StdoutLocked`. This is still pretty slow,
  for reasons explained in the comment above.

* `fast.rs` uses a `BufWriter` atop `StdoutLocked`. This is
  the version that is 2× faster than the slow version.

* `speedy.rs` uses a `BufWriter` atop a raw UNIX `File`. It
  is about 10% faster than the fast version, but is portable
  only to UNIX systems and has an `unsafe` in it.

* `turbo.rs` is a fairly straightforward port of `turbo.c`,
  which avoids standard library routines for things in favor
  of hand-calculation. `turbo.rs` is about 60% faster than
  `speedy.rs` and about 30% slower than `turbo.c`.
  
* `turbo.cpp` is another port of `turbo.c` contributed by
  Hossain Adnan. It's a little faster than `turbo.c` on my
  box, but the author reports 20% slower.

* `mappy.rs` is a work-in-progress attempt to use
  memory-mapped I/O. It doesn't run yet.
