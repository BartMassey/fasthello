use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    for i in 0..100_000_000 {
        let _ = stdout.write(b"Hello, ").unwrap();
        let _ = itoa::write(&mut stdout, i).unwrap();
        let _ = stdout.write(b"\t").unwrap();
    }
}
