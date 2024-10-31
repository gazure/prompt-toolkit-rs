use crate::styles::{AnsiColor, Attrs, Color};

pub struct Size {
    pub rows: usize,
    pub columns: usize,
}
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Size(rows={}, columns={})", self.rows, self.columns)
    }
}

#[derive(Debug, Default)]
pub enum CursorShape {
    #[default]
    NeverChange,
    Block,
    Beam,
    Underline,
    BlinkingBlock,
    BlinkingBeam,
    BlinkingUnderline,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorDepth {
    Monochrome,
    Ansi,
    Default,
    True,
}

impl ColorDepth {
    #[must_use]
    pub fn bit_depth(&self) -> usize {
        match self {
            ColorDepth::Monochrome => 1,
            ColorDepth::Ansi => 4,
            ColorDepth::Default => 8,
            ColorDepth::True => 24,
        }
    }

    fn depth_aware_escape_code(self, color: Color, foreground_info: Option<Color>) -> Vec<String> {
        let (r, g, b) = color.rgb();
        let is_background = foreground_info.is_some();
        let first = if is_background { "48" } else { "38" };
        match self {
            Self::Monochrome => vec![],
            Self::Ansi => {
                let exclude = if let Some(foreground_color) = foreground_info {
                    if color == foreground_color {
                        vec![foreground_color.closest_ansi()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };
                vec![AnsiColor::closest_from_rgb(r, g, b, &exclude)
                    .code()
                    .to_string()]
            }
            Self::Default => {
                vec![
                    first.to_string(),
                    "5".to_string(),
                    format!(
                        "{}",
                        16 + ((u16::from(r) * 6 / 256) * 36
                            + (u16::from(g) * 6 / 256) * 6
                            + (u16::from(b) * 6 / 256))
                    ),
                ]
            }
            Self::True => {
                vec![
                    first.to_string(),
                    "2".to_string(),
                    r.to_string(),
                    b.to_string(),
                    g.to_string(),
                ]
            }
        }
    }

    fn colors_to_escape_code(self, color: Color, foreground_info: Option<Color>) -> Vec<String> {
        let is_background = foreground_info.is_some();
        match (self, color) {
            (ColorDepth::Monochrome, _) | (_, Color::Default) => vec![],
            (_, Color::Ansi(ansi_color)) => {
                vec![(if is_background {
                    ansi_color.to_background_code()
                } else {
                    ansi_color.code()
                })
                .to_string()]
            }
            // TODO: refactor this, matching self to match again is kind of goofy
            (_, Color::Hex(_, _, _)) => self.depth_aware_escape_code(color, foreground_info),
        }
    }

    #[must_use]
    pub fn escape_code(self, attrs: Attrs) -> String {
        let mut parts: Vec<String> = Vec::new();
        let fg = attrs.color.unwrap_or_default();
        parts.extend(self.colors_to_escape_code(fg, None));
        parts.extend(
            self.colors_to_escape_code(attrs.background_color.unwrap_or_default(), Some(fg)),
        );
        if attrs.bold.is_on() {
            parts.push("1".to_string());
        }
        if attrs.italic.is_on() {
            parts.push("3".to_string());
        }
        if attrs.blink.is_on() {
            parts.push("5".to_string());
        }
        if attrs.underline.is_on() {
            parts.push("4".to_string());
        }
        if attrs.reverse.is_on() {
            parts.push("7".to_string());
        }
        if attrs.hidden.is_on() {
            parts.push("8".to_string());
        }
        if attrs.strike.is_on() {
            parts.push("9".to_string());
        }

        if parts.is_empty() {
            "\x1b[0m".to_string()
        } else {
            format!("\x1b[0;{}m", parts.join(";"))
        }
    }
}

pub trait Output {
    fn fileno(&self) -> i32;
    fn encoding(&self) -> &'static str;
    fn write(&mut self, data: &str);
    fn write_raw(&mut self, data: &str);
    fn set_title(&mut self, title: &str);
    fn clear_title(&mut self);
    fn flush(&mut self);
    fn erase_screen(&mut self);
    fn enter_alternate_screen(&mut self);
    fn quit_alternate_screen(&mut self);
    fn enable_mouse_support(&mut self);
    fn disable_mouse_support(&mut self);
    fn erase_end_of_line(&mut self);
    fn erase_down(&mut self);
    fn reset_attributes(&mut self);
    fn set_attributes(&mut self, attrs: Attrs, color_depth: ColorDepth);
    fn disable_autowrap(&mut self);
    fn enable_autowrap(&mut self);
    fn cursor_goto(&mut self, row: usize, column: usize);
    fn cursor_up(&mut self, amount: usize);
    fn cursor_down(&mut self, amount: usize);
    fn cursor_forward(&mut self, amount: usize);
    fn cursor_back(&mut self, amount: usize);
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
    fn set_cursor_shape(&mut self, shape: CursorShape);
    fn reset_cursor_shape(&mut self);
    fn supports_cursor_position_requests(&self) -> bool;
    fn request_cursor_position(&mut self);
    fn get_size(&self) -> Size;
    fn get_default_color_depth() -> ColorDepth;
}

pub struct DummyOutput;

impl Output for DummyOutput {
    fn fileno(&self) -> i32 {
        panic!("no sensible default for dummy output fileno")
    }

    fn encoding(&self) -> &'static str {
        "utf-8"
    }

    fn write(&mut self, _data: &str) {}

    fn write_raw(&mut self, _data: &str) {}

    fn set_title(&mut self, _title: &str) {}

    fn clear_title(&mut self) {}

    fn flush(&mut self) {}

    fn erase_screen(&mut self) {}

    fn enter_alternate_screen(&mut self) {}

    fn quit_alternate_screen(&mut self) {}

    fn enable_mouse_support(&mut self) {}

    fn disable_mouse_support(&mut self) {}

    fn erase_end_of_line(&mut self) {}

    fn erase_down(&mut self) {}

    fn reset_attributes(&mut self) {}

    fn set_attributes(&mut self, _attrs: Attrs, _color_depth: ColorDepth) {}

    fn disable_autowrap(&mut self) {}

    fn enable_autowrap(&mut self) {}

    fn cursor_goto(&mut self, _row: usize, _column: usize) {}

    fn cursor_up(&mut self, _amount: usize) {}

    fn cursor_down(&mut self, _amount: usize) {}

    fn cursor_forward(&mut self, _amount: usize) {}

    fn cursor_back(&mut self, _amount: usize) {}

    fn hide_cursor(&mut self) {}

    fn show_cursor(&mut self) {}

    fn set_cursor_shape(&mut self, _shape: CursorShape) {}

    fn reset_cursor_shape(&mut self) {}

    fn supports_cursor_position_requests(&self) -> bool {
        false
    }

    fn request_cursor_position(&mut self) {}

    fn get_size(&self) -> Size {
        Size {
            rows: 40,
            columns: 40,
        }
    }

    fn get_default_color_depth() -> ColorDepth {
        ColorDepth::Monochrome
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cursor_shapes() {
        let mut out = DummyOutput;

        // NeverChange should not affect the cursor shape
        out.set_cursor_shape(CursorShape::NeverChange);

        // Test all cursor shape variants
        out.set_cursor_shape(CursorShape::Block);
        out.set_cursor_shape(CursorShape::Beam);
        out.set_cursor_shape(CursorShape::Underline);
        out.set_cursor_shape(CursorShape::BlinkingBlock);
        out.set_cursor_shape(CursorShape::BlinkingBeam);
        out.set_cursor_shape(CursorShape::BlinkingUnderline);

        // Reset should restore default cursor shape
        out.reset_cursor_shape();
    }
}
