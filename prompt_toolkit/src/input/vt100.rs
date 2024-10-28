#![expect(dead_code)]

use std::os::fd::RawFd;
use std::sync::LazyLock;

use regex::Regex;

pub use crate::input::Input;

static CURSOR_POSITION_RESPONSE_PREFIX_REGEX: LazyLock<Regex> =
    LazyLock::new(|| regex::Regex::new(r"^\x1b\[[\d;]*$").expect("valid regex"));

pub struct KeyPress {
    key: String,
    data: String,
}

pub struct VT100 {
    in_fd: RawFd,
    fileno: i32,
    buffer: Vec<KeyPress>,
    parser: Parser,
}

impl VT100 {
    pub fn new(in_fd: RawFd) -> Self {
        let fileno = in_fd.clone();
        Self {
            in_fd,
            fileno,
            buffer: Vec::new(),
            parser: Parser::new(),
        }
    }
}

impl Input for VT100 {
    fn fileno(&self) -> i32 {
        todo!()
    }

    fn typeahead_hash(&self) -> &str {
        todo!()
    }

    fn read_keys(&mut self) -> Vec<String> {
        todo!()
    }

    fn closed(&self) -> bool {
        todo!()
    }

    fn to_raw_mode(&mut self) {
        todo!()
    }

    fn to_cooked_mode(&mut self) {
        todo!()
    }
}

struct Parser {
    in_bracketed_paste: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            in_bracketed_paste: false,
        }
    }

    pub fn feed(&self, _buffer: &mut [KeyPress], _data: &str) {}
}

struct ParserStateMachine {
    prefix: String,
    retry: bool,
    flush: bool,
}

enum StateMachineInput {
    Character(String),
    Flush,
}

impl ParserStateMachine {
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            retry: false,
            flush: false,
        }
    }

    pub fn send(&mut self, c: StateMachineInput) {
        match c {
            StateMachineInput::Flush => self.flush = true,
            StateMachineInput::Character(c) => {
                self.prefix.push_str(&c);
            }
        }

        if !self.prefix.is_empty() {}
    }
}
