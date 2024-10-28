#![expect(dead_code)]

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

#[derive(Debug, PartialEq, Eq)]
pub enum AnsiColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl AnsiColor {
    pub fn to_code(&self) -> i32 {
        match self {
            AnsiColor::Default => 0,
            AnsiColor::Black => 30,
            AnsiColor::Red => 31,
            AnsiColor::Green => 32,
            AnsiColor::Yellow => 33,
            AnsiColor::Blue => 34,
            AnsiColor::Magenta => 35,
            AnsiColor::Cyan => 36,
            AnsiColor::White => 37,
            AnsiColor::BrightBlack => 90,
            AnsiColor::BrightRed => 91,
            AnsiColor::BrightGreen => 92,
            AnsiColor::BrightYellow => 93,
            AnsiColor::BrightBlue => 94,
            AnsiColor::BrightMagenta => 95,
            AnsiColor::BrightCyan => 96,
            AnsiColor::BrightWhite => 97,
        }
    }
    pub fn to_background_code(&self) -> i32 {
        match self {
            AnsiColor::Default => 0,
            AnsiColor::Black => 40,
            AnsiColor::Red => 41,
            AnsiColor::Green => 42,
            AnsiColor::Yellow => 43,
            AnsiColor::Blue => 44,
            AnsiColor::Magenta => 45,
            AnsiColor::Cyan => 46,
            AnsiColor::White => 47,
            AnsiColor::BrightBlack => 100,
            AnsiColor::BrightRed => 101,
            AnsiColor::BrightGreen => 102,
            AnsiColor::BrightYellow => 103,
            AnsiColor::BrightBlue => 104,
            AnsiColor::BrightMagenta => 105,
            AnsiColor::BrightCyan => 106,
            AnsiColor::BrightWhite => 107,
        }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            AnsiColor::Default => (0, 0, 0), // Default to black
            AnsiColor::Black => (0, 0, 0),
            AnsiColor::Red => (205, 0, 0),
            AnsiColor::Green => (0, 205, 0),
            AnsiColor::Yellow => (205, 205, 0),
            AnsiColor::Blue => (0, 0, 238),
            AnsiColor::Magenta => (205, 0, 205),
            AnsiColor::Cyan => (0, 205, 205),
            AnsiColor::White => (229, 229, 229),
            AnsiColor::BrightBlack => (127, 127, 127),
            AnsiColor::BrightRed => (255, 0, 0),
            AnsiColor::BrightGreen => (0, 255, 0),
            AnsiColor::BrightYellow => (255, 255, 0),
            AnsiColor::BrightBlue => (92, 92, 255),
            AnsiColor::BrightMagenta => (255, 0, 255),
            AnsiColor::BrightCyan => (0, 255, 255),
            AnsiColor::BrightWhite => (255, 255, 255),
        }
    }

    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "ansidefault" => Some(Self::Default),
            "ansiblack" => Some(Self::Black),
            "ansired" => Some(Self::Red),
            "ansigreen" => Some(Self::Green),
            "ansiyellow" => Some(Self::Yellow),
            "ansiblue" => Some(Self::Blue),
            "ansimagenta" => Some(Self::Magenta),
            "ansicyan" => Some(Self::Cyan),
            "ansiwhite" => Some(Self::White),
            "ansibrightblack" => Some(Self::BrightBlack),
            "ansibrightred" => Some(Self::BrightRed),
            "ansibrightgreen" => Some(Self::BrightGreen),
            "ansibrightyellow" => Some(Self::BrightYellow),
            "ansibrightblue" => Some(Self::BrightBlue),
            "ansibrightmagenta" => Some(Self::BrightMagenta),
            "ansibrightcyan" => Some(Self::BrightCyan),
            "ansibrightwhite" => Some(Self::BrightWhite),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ColorDepth {
    Monochrome,
    Ansi,
    Default,
    True,
}

impl ColorDepth {
    pub fn bit_depth(&self) -> usize {
        match self {
            ColorDepth::Monochrome => 1,
            ColorDepth::Ansi => 4,
            ColorDepth::Default => 8,
            ColorDepth::True => 24,
        }
    }

    fn rgb(color: &str) -> Option<(u8, u8, u8)> {
        if let Some(digits) = color
            .chars()
            .map(|c| c.to_digit(16))
            .collect::<Option<Vec<u32>>>()
        {
            if digits.len() == 6 {
                let r = (
                    (digits[0] * 16 + digits[1]) as u8,
                    (digits[2] * 16 + digits[3]) as u8,
                    (digits[4] * 16 + digits[5]) as u8,
                );
                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn depth_code_from_rgb(&self, color: &str) -> Vec<String> {
        let Some((r, g, b)) = Self::rgb(color) else {
            return Vec::new();
        };

        match self {
            Self::Monochrome => vec![],
            Self::Ansi => {
                vec![AnsiColor::Black.to_code().to_string()]
            }
            Self::Default => {
                vec![
                    "38".to_string(),
                    "5".to_string(),
                    format!(
                        "{}",
                        16 + ((r as u16 * 6 / 256) * 36
                            + (g as u16 * 6 / 256) * 6
                            + (b as u16 * 6 / 256))
                    ),
                ]
            }
            Self::True => {
                vec![
                    "38".to_string(),
                    "2".to_string(),
                    r.to_string(),
                    b.to_string(),
                    g.to_string(),
                ]
            }
        }
    }

    fn colors_to_code(&self, color: &str, is_background: bool) -> Vec<String> {
        if color.is_empty() || matches!(self, Self::Monochrome) {
            return Vec::new();
        }

        if let Some(ansi_color) = AnsiColor::try_from_str(color) {
            let color = if is_background {
                ansi_color.to_background_code()
            } else {
                ansi_color.to_code()
            };
            return vec![color.to_string()];
        }

        self.depth_code_from_rgb(color)
    }

    pub fn escape_code(&self, attrs: Attrs) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.extend(self.colors_to_code(&attrs.color.unwrap_or_default(), false));
        parts.extend(self.colors_to_code(&attrs.background_color.unwrap_or_default(), true));
        if attrs.bold {
            parts.push("1".to_string());
        }
        if attrs.italic {
            parts.push("3".to_string());
        }
        if attrs.blink {
            parts.push("5".to_string());
        }
        if attrs.underline {
            parts.push("4".to_string());
        }
        if attrs.reverse {
            parts.push("7".to_string());
        }
        if attrs.hidden {
            parts.push("8".to_string());
        }
        if attrs.strike {
            parts.push("9".to_string());
        }

        if !parts.is_empty() {
            format!("\x1b[0;{}m", parts.join(";"))
        } else {
            "\x1b[0m".to_string()
        }
    }
}

#[derive(Debug, Default)]
pub struct Attrs {
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub bold: bool,
    pub underline: bool,
    pub strike: bool,
    pub italic: bool,
    pub blink: bool,
    pub reverse: bool,
    pub hidden: bool,
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

    #[test]
    fn test_str_to_ansi_color() {
        // Test valid colors
        assert_eq!(
            AnsiColor::try_from_str("ansidefault"),
            Some(AnsiColor::Default)
        );
        assert_eq!(AnsiColor::try_from_str("ansiblack"), Some(AnsiColor::Black));
        assert_eq!(AnsiColor::try_from_str("ansired"), Some(AnsiColor::Red));
        assert_eq!(AnsiColor::try_from_str("ansiblue"), Some(AnsiColor::Blue));
        assert_eq!(
            AnsiColor::try_from_str("ansibrightred"),
            Some(AnsiColor::BrightRed)
        );
        assert_eq!(
            AnsiColor::try_from_str("ansibrightblue"),
            Some(AnsiColor::BrightBlue)
        );

        // Test invalid colors
        assert_eq!(AnsiColor::try_from_str(""), None);
        assert_eq!(AnsiColor::try_from_str("red"), None);
        assert_eq!(AnsiColor::try_from_str("invalid"), None);
        assert_eq!(AnsiColor::try_from_str("bright_red"), None);
    }
}
