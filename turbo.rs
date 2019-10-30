fn increase_str_num(input: &mut [u8]) -> usize {
    let input_size = input.len();
    for i in (0..input_size).rev() {
        input[i] += 1;
        if input[i] - b'0' < 10 {
            return input_size - i;
        }
        input[i] = b'0';
    }
    panic!("overflowed input");
}

#[allow(unused)]
fn main() {
    const BUFSIZ: usize = 8192;
    let mut buffer = Vec::with_capacity(BUFSIZ);
    let mut num = [b'0';12];
    let mut num_len = 1;
    let prefix = b"Hello, ";
    const LINE_MAX_LEN: usize = 17;
    for _ in 0..100_000_000 {
        buffer.extend_from_slice(prefix);
        num_len = num_len.max(increase_str_num(&mut num));
        buffer.extend_from_slice(&num[num.len() - num_len..]);
        buffer.push(b'\t');
        if buffer.len() + LINE_MAX_LEN > BUFSIZ {
            nix::unistd::write(1, &buffer);
            buffer.clear();
        }
    }
    nix::unistd::write(1, &buffer);
}
