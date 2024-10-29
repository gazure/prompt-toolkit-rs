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
