#![expect(dead_code)]

use tracing::warn;

pub use crate::input::Input;
use std::os::fd::RawFd;

use super::base::KeyPress;
use super::posix_utils::PosixStdinReader;
use super::vt100_parser::Parser;

pub struct VT100 {
    in_fd: RawFd,
    fileno: i32,
    buffer: Vec<KeyPress>,
    reader: PosixStdinReader,
    parser: Parser,
}

impl VT100 {
    #[must_use]
    pub fn new(in_fd: RawFd) -> Self {
        let fileno = in_fd;
        Self {
            in_fd,
            fileno,
            buffer: Vec::new(),
            reader: PosixStdinReader::new(in_fd),
            parser: Parser::new(),
        }
    }
}

impl Input for VT100 {
    fn fileno(&self) -> i32 {
        self.fileno
    }

    fn typeahead_hash(&self) -> String {
        format!("fd-{}", self.fileno())
    }

    fn read_keys(&mut self) -> Vec<KeyPress> {
        match self.reader.read(1024) {
            Ok(data) => self.parser.feed(&data),
            Err(e) => {
                warn!("Got an error when trying to read: {e}");
                vec![]
            }
        }
    }

    fn flush_keys(&mut self) -> Vec<KeyPress> {
        self.parser.flush()
    }

    fn closed(&self) -> bool {
        self.reader.closed()
    }

    fn to_raw_mode(&mut self) {
        todo!()
    }

    fn to_cooked_mode(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::keys::Keys;

    #[test]
    fn test_single_char() {
        let mut vt = VT100::new(0);
        let keys = vt.parser.feed("x");
        assert_eq!(keys.len(), 1);
        assert_eq!(
            keys[0],
            KeyPress::new(Keys::Character('x'), "x".to_string())
        );
    }

    #[test]
    fn test_escape_seq() {
        let mut vt = VT100::new(0);
        let keys = vt.parser.feed("\x1b[A");
        assert_eq!(keys, vec![KeyPress::new(Keys::Up, "\x1B[A".to_string())]);
    }

    // #[test]
    // fn test_escape() {
    //     let mut vt = VT100::new(0);
    //     let keys = vt.parser.feed("\x1b");
    //     assert_eq!(keys, vec![KeyPress::new(Keys::Escape, "\x1B".to_string())]);
    // }

    #[test]
    fn test_invalid_seq() {
        let mut vt = VT100::new(0);
        let keys = vt.parser.feed("\x1b[");
        assert_eq!(keys, vec![]);
    }
}
