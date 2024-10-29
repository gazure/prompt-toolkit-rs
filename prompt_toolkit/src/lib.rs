#![deny(clippy::pedantic)]

mod clipboard;
mod filters;
mod key_bindings;
mod screen;

pub mod output;
pub mod render;
pub mod application;
pub mod input;
pub mod shortcuts;

pub(crate) mod keys;
pub(crate) mod layout;

pub use input::{Input, KeyPress};
pub use output::{Output, Size};
pub use screen::{Char, Point, Screen, WritePosition};
