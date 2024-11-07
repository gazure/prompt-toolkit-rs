use std::collections::HashMap;

use tracing::info;

use crate::application::Application;
use crate::keys::Keys;
use crate::KeyPress;

#[derive(Debug)]
pub struct KeyPressEvent {
    pub key_press: KeyPress,
}

impl KeyPressEvent {
    #[must_use]
    pub fn new(key_press: KeyPress) -> Self {
        KeyPressEvent { key_press }
    }
}

pub trait Binding {
    fn handler(&mut self, key_press_event: &KeyPressEvent);
}

pub struct KeyBindings {
    bindings: HashMap<Keys, Box<dyn Binding>>,
    all_keys_bindings: Vec<Box<dyn Binding>>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyBindings {
    #[must_use]
    pub fn new() -> KeyBindings {
        KeyBindings {
            bindings: HashMap::new(),
            all_keys_bindings: Vec::new(),
        }
    }

    #[must_use]
    pub fn get(&self, keys: &Keys) -> Option<&dyn Binding> {
        self.bindings.get(keys).map(|v| &**v)
    }

    pub fn get_mut(&mut self, keys: &Keys) -> Option<&mut Box<dyn Binding>> {
        self.bindings.get_mut(keys)
    }

    pub fn add(&mut self, keys: Keys, binding: Box<dyn Binding>) {
        self.bindings.insert(keys, binding);
    }

    pub fn add_for_all_keys(&mut self, binding: Box<dyn Binding>) {
        self.all_keys_bindings.push(binding);
    }

    #[must_use]
    pub fn get_all_keys_bindings(&self) -> &Vec<Box<dyn Binding>> {
        &self.all_keys_bindings
    }

    pub fn get_all_keys_bindings_mut(&mut self) -> &mut Vec<Box<dyn Binding>> {
        &mut self.all_keys_bindings
    }
}

pub struct KeyProcessor {
    bindings: KeyBindings,
}

impl KeyProcessor {
    #[must_use]
    pub fn new(bindings: KeyBindings) -> Box<KeyProcessor> {
        Box::new(KeyProcessor { bindings })
    }

    pub fn process_key(&mut self, key_press: &KeyPressEvent, _app: &mut Application) {
        if let Some(binding) = self.bindings.get_mut(&key_press.key_press.key()) {
            binding.handler(key_press);
        } else {
            for binding in self.bindings.get_all_keys_bindings_mut() {
                binding.handler(key_press);
            }
        }
    }
}

pub struct EchoBinding;

impl Binding for EchoBinding {
    fn handler(&mut self, key_press_event: &KeyPressEvent) {
        info!("key press: {:?}", key_press_event.key_press);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_bindings_new() {
        let bindings = KeyBindings::new();
        assert!(bindings.bindings.is_empty());
        assert!(bindings.all_keys_bindings.is_empty());
    }

    #[test]
    fn test_key_bindings_add() {
        let mut bindings = KeyBindings::new();
        let echo = Box::new(EchoBinding);
        bindings.add(Keys::Character('a'), echo);
        assert_eq!(bindings.bindings.len(), 1);
    }

    #[test]
    fn test_key_bindings_add_for_all_keys() {
        let mut bindings = KeyBindings::new();
        let echo = Box::new(EchoBinding);
        bindings.add_for_all_keys(echo);
        assert_eq!(bindings.all_keys_bindings.len(), 1);
    }

    #[test]
    fn test_key_bindings_get() {
        let mut bindings = KeyBindings::new();
        let echo = Box::new(EchoBinding);
        bindings.add(Keys::Character('a'), echo);
        assert!(bindings.get(&Keys::Character('a')).is_some());
        assert!(bindings.get(&Keys::Character('b')).is_none());
    }

    #[test]
    fn test_key_press_event_new() {
        let key_press = KeyPress::new(Keys::Character('x'), "x".to_string());
        let event = KeyPressEvent::new(key_press);
        assert_eq!(event.key_press.key(), Keys::Character('x'));
    }
}
