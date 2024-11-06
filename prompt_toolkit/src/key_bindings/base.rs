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

impl KeyBindings {
    pub fn new() -> KeyBindings {
        KeyBindings {
            bindings: HashMap::new(),
            all_keys_bindings: Vec::new(),
        }
    }

    pub fn get(&self, keys: &Keys) -> Option<&Box<dyn Binding>> {
        self.bindings.get(keys)
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
    pub fn new(bindings: KeyBindings) -> Box<KeyProcessor> {
        Box::new(KeyProcessor { bindings })
    }

    pub fn process_key(&mut self, key_press: KeyPressEvent, _app: &mut Application) {
        if let Some(binding) = self.bindings.get_mut(&key_press.key_press.key()) {
            binding.handler(&key_press);
        } else {
            for binding in self.bindings.get_all_keys_bindings_mut() {
                binding.handler(&key_press)
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
