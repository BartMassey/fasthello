use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

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

const BUFSIZ: usize = 8192;

struct Buffer {
    data: [u8; BUFSIZ],
    index: usize
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            data: [0; BUFSIZ],
            index: 0
        }
    }

    pub fn extend(&mut self, str: &[u8]) {
        self.data[self.index..self.index+str.len()].copy_from_slice(str);
        self.index += str.len();
    }

    pub fn push(&mut self, c: u8) {
        self.data[self.index] = c;
        self.index += 1;
    }

    pub fn clear(&mut self) {
        self.index = 0;
    }

    pub fn size(&self) -> usize {
        self.index
    }
}

fn main() {
    let mut stdout: File = unsafe {
        FromRawFd::from_raw_fd(1)
    };
    let mut buffer = Buffer::new();
    let mut num = [b'0';12];
    let mut num_len = 1;
    let prefix = b"Hello, ";
    const LINE_MAX_LEN: usize = 17;
    for _ in 0..100_000_000 {
        buffer.extend(prefix);
        buffer.extend(&num[num.len() - num_len..]);
        num_len = num_len.max(increase_str_num(&mut num));
        buffer.push(b'\t');
        if buffer.size() + LINE_MAX_LEN > BUFSIZ {
            stdout.write(&buffer.data[..buffer.size()]).unwrap();
            buffer.clear();
        }
    }
    stdout.write(&buffer.data[..buffer.size()]).unwrap();
}
