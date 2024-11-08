#![expect(dead_code)]
#![expect(unused_variables)]
#![expect(clippy::unused_self)]

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{
    application::Application,
    layout::Layout,
    styles::{Attrs, Style},
    Char, Output, Point, Screen, Size,
};

#[derive(Debug)]
enum CPRSupport {
    Unknown,
    Supported,
    Unsupported,
}

struct Renderer<O: Output> {
    style: Box<dyn Style>,
    output: O,
    bracketed_paste_enabled: bool,
    cursor_key_mode_reset: bool,
    cpr_support: CPRSupport,
}

impl<O: Output> Renderer<O> {
    pub fn new(output: O, style: Box<dyn Style>) -> Self {
        let cpr_support = output.supports_cursor_position_requests();
        let mut renderer = Self {
            style,
            output,
            bracketed_paste_enabled: false,
            cursor_key_mode_reset: false,
            cpr_support: if cpr_support {
                CPRSupport::Unknown
            } else {
                CPRSupport::Unsupported
            },
        };
        renderer.reset();
        renderer
    }

    pub fn reset(&mut self) {
        self.output.flush();
    }

    pub fn height_is_known(&self) -> bool {
        false
    }

    /// Number of rows visible to the terminal above the layout
    pub fn rows_above_layout(&self) -> Result<usize> {
        Err(anyhow!("unknown rows above layout"))
    }

    /// TODO: handle async flow for Cursor Position Requests/Responses
    pub fn request_absolute_cursor_position(&self) {}

    pub fn render(&mut self, app: &Application, layout: &Layout, is_done: bool) {
        if is_done {
            self.reset();
        }
    }
}

fn get_max_column_index(row: &HashMap<usize, Char>, width: usize) -> usize {
    row.keys().max().copied().unwrap_or(width - 1)
}

fn move_cursor(
    output: &mut dyn Output,
    size: &Size,
    new: Point,
    mut current_position: Point,
) -> Point {
    if new.y > current_position.y {
        output.reset_attributes();
        let newlines = "\r\n".repeat(new.y - current_position.y);
        output.write(&newlines);
        current_position.x = 0;
        output.cursor_forward(new.x);
        return new;
    }
    if new.y < current_position.y {
        output.cursor_up(current_position.y - new.y);
    }

    if current_position.x >= size.columns - 1 {
        output.write("\r");
        output.cursor_forward(new.x);
    } else if new.x < current_position.x || current_position.x >= size.columns - 1 {
        output.cursor_back(current_position.x - new.x);
    } else if new.x > current_position.x {
        output.cursor_forward(new.x - current_position.x);
    }
    new
}

pub fn output_screen(output: &mut dyn Output, screen: &Screen, size: &Size) -> Point {
    let mut current_position = Point::new(0, 0);
    output.hide_cursor();
    output.disable_autowrap();

    let width = size.columns;
    let current_height = std::cmp::min(size.rows, screen.height);
    let row_count = current_height;

    let blank_row = HashMap::new();
    let default_char = screen.default_char();

    for y in 0..row_count {
        let row = screen.data_buffer.get(&y).unwrap_or(&blank_row);
        let max_line_len = std::cmp::min(width - 1, get_max_column_index(row, width));
        let mut c = 0usize;

        while c <= max_line_len {
            let render_char = row.get(&c).unwrap_or(&default_char);
            let attrs = Attrs::from_style_string(&render_char.style);
            let char_width = render_char.width;
            current_position = move_cursor(output, size, Point::new(c, y), current_position);
            output.set_attributes(attrs, crate::output::ColorDepth::True);
            output.write(&render_char.char.to_string());
            current_position.x += char_width;
            c += char_width;
        }
    }

    output.reset_attributes();
    if screen.show_cursor() {
        output.show_cursor();
    }
    current_position
}
