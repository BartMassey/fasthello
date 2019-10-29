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
