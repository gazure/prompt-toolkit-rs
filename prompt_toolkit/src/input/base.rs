use crate::keys::Keys;
use nix::sys::termios::Termios;

#[derive(Debug)]
pub struct KeyPress {
    key: Keys,
    text: String,
}

impl PartialEq for KeyPress {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.text == other.text
    }
}

impl KeyPress {
    #[must_use]
    pub fn new(key: Keys, text: String) -> Self {
        Self { key, text }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn key(&self) -> Keys {
        self.key
    }
}

pub struct RawTermGuard<'a> {
    input: &'a mut dyn Input,
    original_mode: Option<Termios>,
}

impl<'a> RawTermGuard<'a> {
    #[must_use]
    pub fn new(input: &'a mut dyn Input) -> Self {
        let original_mode = input.to_raw_mode();
        Self {
            input,
            original_mode,
        }
    }
}

impl<'a> Drop for RawTermGuard<'a> {
    fn drop(&mut self) {
        self.input.to_cooked_mode(self.original_mode.clone());
    }
}

pub trait Input {
    fn fileno(&self) -> i32;
    fn typeahead_hash(&self) -> String;
    fn read_keys(&mut self) -> Vec<KeyPress>;
    fn flush_keys(&mut self) -> Vec<KeyPress>;
    fn closed(&self) -> bool;
    fn to_raw_mode(&mut self) -> Option<Termios>;
    fn raw_mode(&mut self) -> RawTermGuard;
    fn to_cooked_mode(&mut self, original_mode: Option<Termios>);
}

pub struct DummyInput;

impl Input for DummyInput {
    fn fileno(&self) -> i32 {
        -1
    }

    fn typeahead_hash(&self) -> String {
        String::default()
    }

    fn read_keys(&mut self) -> Vec<KeyPress> {
        vec![]
    }

    fn flush_keys(&mut self) -> Vec<KeyPress> {
        vec![]
    }

    fn closed(&self) -> bool {
        true
    }

    fn raw_mode(&mut self) -> RawTermGuard {
        RawTermGuard::new(self)
    }

    fn to_raw_mode(&mut self) -> Option<Termios> {
        None
    }

    fn to_cooked_mode(&mut self, _original_mode: Option<Termios>) {}
}

impl Input for RawTermGuard<'_> {
    fn fileno(&self) -> i32 {
        self.input.fileno()
    }

    fn typeahead_hash(&self) -> String {
        self.input.typeahead_hash()
    }

    fn read_keys(&mut self) -> Vec<KeyPress> {
        self.input.read_keys()
    }

    fn flush_keys(&mut self) -> Vec<KeyPress> {
        self.input.flush_keys()
    }

    fn closed(&self) -> bool {
        self.input.closed()
    }

    fn to_raw_mode(&mut self) -> Option<Termios> {
        None
    }

    fn raw_mode(&mut self) -> RawTermGuard {
        panic!("can't do this twice")
    }

    fn to_cooked_mode(&mut self, _original_mode: Option<Termios>) {}
}
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_dummy_input() {
        let mut di = DummyInput;
        assert_eq!(di.fileno(), -1);
        assert_eq!(di.typeahead_hash(), String::default());
        assert_eq!(di.read_keys(), vec![]);
        assert_eq!(di.flush_keys(), vec![]);
        assert!(di.closed());
        assert_eq!(di.to_raw_mode(), None);
        di.to_cooked_mode(None);
        let raw_term = di.raw_mode();
        assert_eq!(raw_term.fileno(), -1);
    }
}
