use std::fs::File;
use std::io::Write;
use std::ops::DerefMut;
use std::os::unix::io::FromRawFd;

use memmap::{MmapMut, MmapOptions};

fn main() {
    let stdout: File = unsafe { FromRawFd::from_raw_fd(1) };
    let mut mmap_options = MmapOptions::new();
    let mut mm_mut: MmapMut = unsafe {
        mmap_options.len(128).map_mut(&stdout).expect("map_mut")
    };
    let mut bytes = [0u8; 9];
    for i in 0..10 {
        mm_mut.deref_mut().write(b"Hello, ").unwrap();
        itoa::write(&mut bytes[..], i).unwrap();
        mm_mut.deref_mut().write(&bytes).unwrap();
        mm_mut.deref_mut().write(b"\t").unwrap();
    }
}
