#![expect(unused_imports)]

mod base;
mod color;

pub use base::{Attrs, DummyStyle, DynamicStyle, Style};
pub use color::{AnsiColor, Color, NAMED_COLORS};
