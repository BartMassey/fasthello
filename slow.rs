use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for i in 0..100_000_000 {
        let _ = lock.write(b"Hello, ").unwrap();
        let _ = itoa::write(&mut lock, i).unwrap();
        let _ = lock.write(b"\t").unwrap();
    }
}
