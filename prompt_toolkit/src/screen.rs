#![expect(dead_code)]

use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Char {
    pub char: char,
    pub style: String,
    pub width: usize,
}

impl Char {
    fn new(c: char, style: &str) -> Self {
        let display_mappings = HashMap::from([
            ('\x00', "^@"),
            ('\x01', "^A"),
            ('\x02', "^B"),
            ('\x03', "^C"),
            ('\x04', "^D"),
            ('\x05', "^E"),
            ('\x06', "^F"),
            ('\x07', "^G"),
            ('\x08', "^H"),
            ('\x09', "^I"),
            ('\x0a', "^J"),
            ('\x0b', "^K"),
            ('\x0c', "^L"),
            ('\x0d', "^M"),
            ('\x0e', "^N"),
            ('\x0f', "^O"),
            ('\x10', "^P"),
            ('\x11', "^Q"),
            ('\x12', "^R"),
            ('\x13', "^S"),
            ('\x14', "^T"),
            ('\x15', "^U"),
            ('\x16', "^V"),
            ('\x17', "^W"),
            ('\x18', "^X"),
            ('\x19', "^Y"),
            ('\x1a', "^Z"),
            ('\x1b', "^["),
            ('\x1c', "^\\"),
            ('\x1d', "^]"),
            ('\x1e', "^^"),
            ('\x1f', "^_"),
            ('\x7f', "^?"),
        ]);

        let (c, style) = if let Some(mapped) = display_mappings.get(&c) {
            ((*mapped).to_string(), style.to_string())
        } else {
            (c.to_string(), style.to_string())
        };

        let ch = c.chars().next().unwrap_or_default();
        Char {
            char: ch,
            style,
            width: unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0),
        }
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Char({}, {})", self.char, self.style)
    }
}

impl PartialEq for Char {
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char && self.style == other.style
    }
}
impl Eq for Char {}

pub struct Screen {
    pub data_buffer: HashMap<usize, HashMap<usize, Char>>,
    default_char: Char,
    zero_width_escapes: HashMap<usize, HashMap<usize, String>>,
    cursor_positions: HashMap<Window, Point>,
    show_cursor: bool,
    menu_positions: HashMap<Window, Point>,
    pub width: usize,
    pub height: usize,
    visible_windows_to_write_positions: HashMap<Window, WritePosition>,
    draw_float_functions: Vec<(i32, Box<dyn Fn()>)>,
}

impl Screen {
    #[must_use]
    pub fn new(default_char: Option<Char>, initial_width: usize, initial_height: usize) -> Self {
        let default_char = default_char.unwrap_or_else(|| Char::new(' ', "[transparent]"));
        Screen {
            data_buffer: HashMap::new(),
            default_char,
            zero_width_escapes: HashMap::new(),
            cursor_positions: HashMap::new(),
            show_cursor: true,
            menu_positions: HashMap::new(),
            width: initial_width,
            height: initial_height,
            visible_windows_to_write_positions: HashMap::new(),
            draw_float_functions: Vec::new(),
        }
    }

    #[must_use]
    pub fn default_char(&self) -> Char {
        self.default_char.clone()
    }

    #[must_use]
    pub fn show_cursor(&self) -> bool {
        self.show_cursor
    }

    pub fn set_cursor_position(&mut self, window: Window, position: Point) {
        self.cursor_positions.insert(window, position);
    }

    pub fn set_menu_position(&mut self, window: Window, position: Point) {
        self.menu_positions.insert(window, position);
    }

    #[must_use]
    pub fn get_cursor_position(&self, window: &Window) -> Point {
        self.cursor_positions
            .get(window)
            .copied()
            .unwrap_or(Point::new(0, 0))
    }

    #[must_use]
    pub fn get_menu_position(&self, window: &Window) -> Point {
        self.menu_positions
            .get(window)
            .copied()
            .or_else(|| self.cursor_positions.get(window).copied())
            .unwrap_or(Point::new(0, 0))
    }

    pub fn draw_with_z_index<F>(&mut self, z_index: i32, draw_func: F)
    where
        F: Fn() + 'static,
    {
        self.draw_float_functions
            .push((z_index, Box::new(draw_func)));
    }

    pub fn draw_all_floats(&mut self) {
        while !self.draw_float_functions.is_empty() {
            self.draw_float_functions.sort_by_key(|item| item.0);
            let func = self.draw_float_functions.remove(0).1;
            func();
        }
    }

    pub fn append_style_to_content(&mut self, style: &str) {
        for row in self.data_buffer.values_mut() {
            for cell in row.values_mut() {
                cell.style.push(' ');
                cell.style.push_str(style);
            }
        }
    }

    pub fn fill_area(&mut self, write_position: &WritePosition, style: &str, after: bool) {
        if style.trim().is_empty() {
            return;
        }

        let xmin = write_position.xpos;
        let xmax = write_position.xpos + write_position.width;
        let y_range = write_position.ypos..(write_position.ypos + write_position.height);

        let (append_style, prepend_style) = if after {
            (format!(" {style}"), String::new())
        } else {
            (String::new(), format!("{style} "))
        };

        for y in y_range {
            let row = self.data_buffer.entry(y).or_default();
            for x in xmin..xmax {
                let cell = row.entry(x).or_insert_with(|| self.default_char.clone());
                let new_style = format!("{}{}{}", prepend_style, cell.style, append_style);
                cell.style = new_style;
            }
        }
    }

    pub fn direct_draw(&mut self, write_position: &WritePosition, data: &str) {
        let mut x = write_position.xpos;
        let mut y = write_position.ypos;

        for c in data.chars() {
            if c == '\n' {
                y += 1;
                x = write_position.xpos;
                continue;
            }

            if x >= write_position.xpos + write_position.width
                || y >= write_position.ypos + write_position.height
            {
                break;
            }

            let ch = Char::new(c, "");
            self.data_buffer.entry(y).or_default().insert(x, ch);
            x += 1;
        }
    }

    #[must_use]
    pub fn buffer_representation(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let ch = self
                    .data_buffer
                    .get(&y)
                    .and_then(|row| row.get(&x))
                    .unwrap_or(&self.default_char);
                result.push(ch.char);
            }
            result.push('\n');
        }
        result.trim_end().to_string()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

pub struct WritePosition {
    xpos: usize,
    ypos: usize,
    width: usize,
    height: usize,
}

impl WritePosition {
    #[must_use]
    pub fn new(xpos: usize, ypos: usize, width: usize, height: usize) -> Self {
        WritePosition {
            xpos,
            ypos,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Window {
    content: String,
}
