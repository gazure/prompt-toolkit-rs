use std::{
    io,
    os::fd::{BorrowedFd, RawFd},
};

use nix::libc;
use tracing::info;

pub struct PosixStdinReader {
    fd: RawFd,
    closed: bool,
}

impl PosixStdinReader {
    pub fn new(fd: RawFd) -> Self {
        Self { fd, closed: false }
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn read(&mut self, count: usize) -> Result<String, io::Error> {
        if self.closed {
            return Ok(String::default());
        }

        let mut read_fds = nix::sys::select::FdSet::new();
        read_fds.insert(unsafe { BorrowedFd::borrow_raw(self.fd) });

        let mut timeout = nix::sys::time::TimeVal::new(0, 0);

        let result = nix::sys::select::select(
            self.fd + 1,
            Some(&mut read_fds),
            None,
            None,
            Some(&mut timeout),
        );

        match result {
            Ok(n) if n < 0 => {
                self.closed = true;
                return Err(io::Error::last_os_error());
            }
            Ok(0) => {
                info!("no FDs were ready!");
                return Ok(String::default());
            }
            Ok(_) => {}
            Err(e) => {
                self.closed = true;
                return Err(io::Error::from(e));
            }
        }

        let mut buf = vec![0u8; count];

        unsafe {
            // todo async
            let bytes_read =
                libc::read(self.fd, buf.as_mut_ptr().cast::<libc::c_void>(), buf.len());
            if bytes_read < 0 {
                Err(io::Error::last_os_error())
            } else {
                buf.truncate(usize::try_from(bytes_read).expect("already checked if isize < 0"));
                // TODO: Make this not lossy
                Ok(String::from_utf8_lossy(&buf).into_owned())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use io::Seek;
    use std::io::Write;
    use std::os::unix::io::AsRawFd;
    use tempfile::tempfile;

    use super::*;

    #[test]
    fn test_new_reader() {
        let stdin = io::stdin();
        let fd = stdin.as_raw_fd();
        let reader = PosixStdinReader::new(fd);
        assert!(!reader.closed());
    }

    #[test]
    fn test_closed_reader() {
        let stdin = io::stdin();
        let fd = stdin.as_raw_fd();
        let mut reader = PosixStdinReader::new(fd);
        reader.closed = true;
        assert!(reader.closed());
        assert_eq!(reader.read(1024).expect("read error"), String::default());
    }

    #[test]
    fn test_read_empty() {
        let stdin = io::stdin();
        let fd = stdin.as_raw_fd();
        let mut reader = PosixStdinReader::new(fd);
        assert_eq!(reader.read(1024).expect("read error"), String::default());
    }

    #[test]
    fn test_read_from_temp_file() {
        let mut file = tempfile().expect("temp file");
        let test_data = "hello world";
        file.write_all(test_data.as_bytes()).expect("write error");
        file.flush().expect("expected flush to work");
        file.seek(std::io::SeekFrom::Start(0)).expect("seek error");

        let fd = file.as_raw_fd();
        let mut reader = PosixStdinReader::new(fd);

        let result = reader.read(1024).expect("read error");
        assert_eq!(result, test_data);
    }
}
