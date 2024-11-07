#![deny(clippy::pedantic)]
#![expect(clippy::module_name_repetitions)]

pub(crate) mod screen;
pub(crate) mod styles;

pub mod application;
pub mod clipboard;
pub mod filters;
pub mod input;
pub mod key_bindings;
pub mod keys;
pub mod layout;
pub mod output;
pub mod render;
pub mod shortcuts;

pub use input::{Input, KeyPress};
pub use output::{Output, Size};
pub use screen::{Char, Point, Screen, WritePosition};
