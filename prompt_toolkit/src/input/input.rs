#![expect(dead_code)]

pub trait Input {
    fn fileno(&self) -> i32;
    fn typeahead_hash(&self) -> &str;
    fn read_keys(&mut self) -> Vec<String>;
    fn closed(&self) -> bool;
    fn to_raw_mode(&mut self);
    fn to_cooked_mode(&mut self);
}

pub struct DummyInput;

impl Input for DummyInput {
    fn fileno(&self) -> i32 {
        -1
    }

    fn typeahead_hash(&self) -> &str {
        ""
    }

    fn read_keys(&mut self) -> Vec<String> {
        vec![]
    }

    fn closed(&self) -> bool {
        true
    }

    fn to_raw_mode(&mut self) {}

    fn to_cooked_mode(&mut self) {}
}
