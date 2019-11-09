use std::fs::File;
use std::io::{stdout, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};

const BUFSIZ: usize = 8192;
const DIGITS: usize = 12;

#[inline(always)]
fn increase_str_num(input: &mut [u8; DIGITS]) -> usize {
    let mut i = DIGITS - 1;
    loop {
        input[i] += 1;
        if input[i] - b'0' < 10 {
            return DIGITS - i;
        }
        input[i] = b'0';
        if i == 0 {
            panic!("overflowed input");
        }
        i -= 1;
    }
}

fn main() {
    let stdout = AsRawFd::as_raw_fd(&stdout());
    let mut stdout: File = unsafe { FromRawFd::from_raw_fd(stdout) };
    let mut buffer = [0u8; BUFSIZ];
    let buf0p: *mut u8 = &mut buffer[0];
    let mut bufp = buf0p;
    let mut num = [b'0'; DIGITS];
    let mut nump: *const u8 = &num[DIGITS - 1];
    let mut num_len = 1;
    let prefix = b"Hello, ";
    let prefixp: *const u8 = &prefix[0];
    let nprefix = prefix.len();
    const LINE_MAX_LEN: usize = 17;
    for _ in 0..100_000_000 {
        unsafe {
            prefixp.copy_to_nonoverlapping(bufp, nprefix);
            bufp = bufp.add(nprefix);
            nump.copy_to_nonoverlapping(bufp, num_len);
            bufp = bufp.add(num_len);
        }
        let l = increase_str_num(&mut num);
        if l > num_len {
            num_len = l;
            nump = &num[DIGITS - num_len];
        }
        unsafe { *bufp = b'\t' };
        unsafe { bufp = bufp.offset(1) };
        let fill_len = bufp as usize - buf0p as usize;
        if fill_len + LINE_MAX_LEN > BUFSIZ {
            let _ = stdout.write(&buffer[..fill_len]).unwrap();
            bufp = buf0p;
        }
    }
    let fill_len = bufp as usize - buf0p as usize;
    let _ = stdout.write(&buffer[..fill_len]).unwrap();
}
