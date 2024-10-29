use crate::keys::Keys;

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
}

pub trait Input {
    fn fileno(&self) -> i32;
    fn typeahead_hash(&self) -> String;
    fn read_keys(&mut self) -> Vec<KeyPress>;
    fn flush_keys(&mut self) -> Vec<KeyPress>;
    fn closed(&self) -> bool;
    fn to_raw_mode(&mut self);
    fn to_cooked_mode(&mut self);
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

    fn to_raw_mode(&mut self) {}

    fn to_cooked_mode(&mut self) {}
}
