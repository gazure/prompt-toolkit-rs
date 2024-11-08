#![expect(dead_code)]

use nix::sys::termios::Termios;
use std::os::fd::RawFd;
use tracing::warn;

use crate::input::{
    base::KeyPress, posix_utils::PosixStdinReader, vt100_parser::Parser, Input, RawTermGuard,
};

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

    fn raw_mode(&mut self) -> RawTermGuard {
        RawTermGuard::new(self)
    }
    fn to_raw_mode(&mut self) -> Option<Termios> {
        termios::enable_raw_mode(self.in_fd, false).ok()
    }

    fn to_cooked_mode(&mut self, original_mode: Option<Termios>) {
        if let Some(original_mode) = original_mode {
            termios::disable_raw_mode(self.in_fd, &original_mode)
                .expect("expected cooked mode to work");
        }
    }
}

mod termios {
    /// Lifted from Rustyline
    use anyhow::Result;
    use nix::sys::termios::{self, SetArg, SpecialCharacterIndices as SCI, Termios};
    use std::os::unix::io::{BorrowedFd, RawFd};
    pub fn disable_raw_mode(tty_in: RawFd, termios: &Termios) -> Result<()> {
        let fd = unsafe { BorrowedFd::borrow_raw(tty_in) };
        Ok(termios::tcsetattr(fd, SetArg::TCSADRAIN, termios)?)
    }
    pub fn enable_raw_mode(tty_in: RawFd, enable_signals: bool) -> Result<Termios> {
        use nix::sys::termios::{ControlFlags, InputFlags, LocalFlags};

        let fd = unsafe { BorrowedFd::borrow_raw(tty_in) };
        let original_mode = termios::tcgetattr(fd)?;
        let mut raw = original_mode.clone();
        // disable BREAK interrupt, CR to NL conversion on input,
        // input parity check, strip high bit (bit 8), output flow control
        raw.input_flags &= !(InputFlags::BRKINT
            | InputFlags::ICRNL
            | InputFlags::INPCK
            | InputFlags::ISTRIP
            | InputFlags::IXON);
        // we don't want raw output, it turns newlines into straight line feeds
        // disable all output processing
        // raw.c_oflag = raw.c_oflag & !(OutputFlags::OPOST);

        // character-size mark (8 bits)
        raw.control_flags |= ControlFlags::CS8;
        // disable echoing, canonical mode, extended input processing and signals
        raw.local_flags &=
            !(LocalFlags::ECHO | LocalFlags::ICANON | LocalFlags::IEXTEN | LocalFlags::ISIG);

        if enable_signals {
            raw.local_flags |= LocalFlags::ISIG;
        }

        raw.control_chars[SCI::VMIN as usize] = 1; // One character-at-a-time input
        raw.control_chars[SCI::VTIME as usize] = 0; // with blocking read

        // let mut key_map: HashMap<KeyEvent, Cmd> = HashMap::with_capacity(4);
        // map_key(&mut key_map, &raw, SCI::VEOF, "VEOF", Cmd::EndOfFile);
        // map_key(&mut key_map, &raw, SCI::VINTR, "VINTR", Cmd::Interrupt);
        // map_key(&mut key_map, &raw, SCI::VQUIT, "VQUIT", Cmd::Interrupt);
        // map_key(&mut key_map, &raw, SCI::VSUSP, "VSUSP", Cmd::Suspend);

        termios::tcsetattr(fd, SetArg::TCSADRAIN, &raw)?;
        Ok(original_mode)
    }
    // fn map_key(
    //     key_map: &mut HashMap<KeyEvent, Cmd>,
    //     raw: &Termios,
    //     index: SCI,
    //     name: &str,
    //     cmd: Cmd,
    // ) {
    //     let cc = char::from(raw.control_chars[index as usize]);
    //     let key = KeyEvent::new(cc, M::NONE);
    //     log::debug!(target: "rustyline", "{}: {:?}", name, key);
    //     key_map.insert(key, cmd);
    // }
}

#[cfg(test)]
mod test {

    use std::{io::Seek, os::fd::AsRawFd};
    use tempfile::tempfile;

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

    #[test]
    fn test_raw_mode() {
        let mut vt = VT100::new(tempfile().expect("temp file").as_raw_fd());
        let mut guard = vt.raw_mode();
        let keys = guard.read_keys();
        guard.flush_keys();
        assert!(keys.is_empty());
        assert!(guard.closed());

        drop(guard);
    }

    #[test]
    #[should_panic(expected = "input already in raw mode")]
    fn test_raw_term_guard_should_panic() {
        let mut vt = VT100::new(tempfile().expect("temp file").as_raw_fd());
        let mut guard = RawTermGuard::new(&mut vt);
        guard.raw_mode();
    }

    #[test]
    fn test_with_temp_file() {
        use std::io::Write;

        let test_data = "hello world this is an arbitrary input from a FD";
        let mut file = tempfile().expect("temp file error");

        file.write_all(test_data.as_bytes()).expect("write error");
        file.flush().expect("expected flush to work");
        file.seek(std::io::SeekFrom::Start(0)).expect("seek error");

        let fd = file.as_raw_fd();

        let mut vt = VT100::new(fd);
        let key_presses = vt.read_keys();
        vt.flush_keys();
        assert_eq!(key_presses.len(), test_data.len());
        let s: String = key_presses
            .iter()
            .map(super::super::base::KeyPress::text)
            .collect();
        assert_eq!(s, "hello world this is an arbitrary input from a FD");
    }

    #[test]
    fn test_closed() {
        let vt = VT100::new(0);
        assert!(!vt.closed());
    }
}
