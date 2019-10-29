use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    for i in 0..100_000_000 {
        stdout.write(b"Hello, ").unwrap();
        itoa::write(&mut stdout, i).unwrap();
        stdout.write(b"\t").unwrap();
    }
}
