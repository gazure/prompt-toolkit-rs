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

impl Application {
    #[must_use]
    pub fn new(
        layout: Layout,
        key_bindings: KeyBindings,
        clipboard: Clipboard,
        color_depth: ColorDepth,
        erase_when_done: bool,
        filter: Filter,
    ) -> Self {
        Self {
            layout,
            key_bindings,
            clipboard,
            color_depth,
            erase_when_done,
            filter,
        }
    }

    #[must_use]
    pub fn color_depth(&self) -> ColorDepth {
        self.color_depth
    }
}
