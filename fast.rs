use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut buf = std::io::BufWriter::with_capacity(32*1024, lock);
    for i in 0..100_000_000 {
        buf.write(b"Hello, ").unwrap();
        itoa::write(&mut buf, i).unwrap();
        buf.write(b"\t").unwrap();
    }
}
