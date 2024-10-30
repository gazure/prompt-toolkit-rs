#![deny(clippy::pedantic)]

pub(crate) mod clipboard;
pub(crate) mod filters;
pub(crate) mod key_bindings;
pub(crate) mod keys;
pub(crate) mod layout;
pub(crate) mod screen;
pub(crate) mod styles;

pub mod application;
pub mod input;
pub mod output;
pub mod render;
pub mod shortcuts;

pub use input::{Input, KeyPress};
pub use output::{Output, Size};
pub use screen::{Char, Point, Screen, WritePosition};
