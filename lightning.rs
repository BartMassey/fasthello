const BUFSIZ: usize = libc::BUFSIZ as usize;
const PREFIX: &[u8] = b"\tHello, ";
const PREFIX_SIZE: usize = 8; // b"\tHello, ".len();
const TARGET: u32 = 100_000_000;

pub struct WriteBuffer {
    pub bytes: Vec<u8>,
    pub fd: std::os::unix::io::RawFd,
}

impl WriteBuffer {
    #[inline(always)]
    pub fn new(byte_capacity: usize, fd: std::os::unix::io::RawFd) -> Self {
        Self {
            bytes: Vec::<u8>::with_capacity(byte_capacity),
            fd: fd,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.bytes.clear()
    }

    #[inline(always)]
    pub fn enqueue_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    #[inline(always)]
    pub fn enqueue_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn flush(&mut self) {
        nix::unistd::write(self.fd, &self.bytes);
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

fn main() {
    let mut buf = WriteBuffer::new(BUFSIZ, libc::STDOUT_FILENO);
    buf.enqueue_bytes(b"Hello, 0");

    for i in 0..TARGET {
        buf.enqueue_bytes(PREFIX);
        let num_len = itoa::write(&mut buf.bytes, i).unwrap();
        if BUFSIZ < buf.len() + PREFIX_SIZE + num_len {
            buf.flush();
            buf.clear();
        }
    }
    buf.enqueue_byte(b'\t');
    buf.flush();
}
