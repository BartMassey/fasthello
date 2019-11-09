use std::fs::File;
use std::io::{stdout, Write};
use std::ops::DerefMut;
use std::os::unix::io::{AsRawFd, FromRawFd};

use memmap::{MmapMut, MmapOptions};

fn main() {
    let stdout = AsRawFd::as_raw_fd(&stdout());
    let stdout: File = unsafe {
        FromRawFd::from_raw_fd(stdout)
    };
    let mut mmap_options = MmapOptions::new();
    let mut mm_mut: MmapMut = unsafe {
        mmap_options.len(128).map_mut(&stdout).expect("map_mut")
    };
    let mut bytes = [0u8; 9];
    for i in 0..10 {
        let _ = mm_mut.deref_mut().write(b"Hello, ").unwrap();
        let _ = itoa::write(&mut bytes[..], i).unwrap();
        let _ = mm_mut.deref_mut().write(&bytes).unwrap();
        let _ = mm_mut.deref_mut().write(b"\t").unwrap();
    }
}
