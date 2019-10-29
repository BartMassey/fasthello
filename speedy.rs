use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

fn main() {
    let stdout: File = unsafe {
        FromRawFd::from_raw_fd(1)
    };
    let mut buf = std::io::BufWriter::with_capacity(32*1024, stdout);
    for i in 0..100_000_000 {
        buf.write(b"Hello, ").unwrap();
        itoa::write(&mut buf, i).unwrap();
        buf.write(b"\t").unwrap();
    }
}
