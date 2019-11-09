pub struct Buffer {
    pub bytes: Vec<u8>,
    pub fd: std::os::unix::io::RawFd,
}

impl Buffer {
    #[inline(always)]
    pub fn new(
        byte_capacity: usize,
        fd: std::os::unix::io::RawFd,
    ) -> Self {
        Self {
            bytes: Vec::with_capacity(byte_capacity),
            fd,
        }
    }

    #[inline(always)]
    pub fn flush(&mut self) {
        let _ = nix::unistd::write(self.fd, &self.bytes).unwrap();
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    #[inline(always)]
    pub fn enqueue(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
}

#[inline(always)]
fn push_hello(buf: &mut Buffer, num: u32) {
    buf.enqueue(b"\tHello, ");
    let _ = itoa::write(&mut buf.bytes, num).unwrap();
}

fn main() {
    const BUFSIZ: usize = libc::BUFSIZ as usize;
    const TARGET: u32 = 100_000_000;
    let mut buf = Buffer::new(BUFSIZ, libc::STDOUT_FILENO);
    buf.enqueue(b"Hello, 0");
    let mut i = 1;
    const PREFIX_SIZE: usize = 1 + 7; // b"\tHello, ".len();
    const NUM_LEN: usize = 9; // b"100000000".len();
    'done: while i < TARGET {
        while i < TARGET
            && BUFSIZ > buf.bytes.len() + PREFIX_SIZE + NUM_LEN
        {
            push_hello(&mut buf, i);
            i += 1;
            if i >= TARGET {
                break 'done;
            }
        }
        buf.flush();
        buf.clear();
        push_hello(&mut buf, i);
        i += 1;
    }
    buf.enqueue(b"\t");
    buf.flush();
}
