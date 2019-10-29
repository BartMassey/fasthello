use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for i in 0..100_000_000 {
        lock.write(b"Hello, ").unwrap();
        itoa::write(&mut lock, i).unwrap();
        lock.write(b"\t").unwrap();
    }
}
