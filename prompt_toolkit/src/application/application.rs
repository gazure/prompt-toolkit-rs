#![expect(dead_code)]

use crate::{
    clipboard::Clipboard, filters::Filter, key_bindings::KeyBindings, layout::Layout,
    output::ColorDepth,
};

pub struct Application {
    layout: Layout,
    key_bindings: KeyBindings,
    clipboard: Clipboard,
    color_depth: ColorDepth,
    erase_when_done: bool,
    filter: Filter,
}
