use nix::errno::Errno;
use nix::libc::{ioctl, winsize, TIOCGWINSZ};
use nix::unistd::write;
use tracing::{error, warn};

use crate::output::Output;
use crate::styles::Attrs;
use std::os::{fd::BorrowedFd, unix::io::RawFd};

use super::{ColorDepth, CursorShape};

#[derive(Debug)]
pub struct VT100 {
    out: RawFd,
    buffer: String,
}

impl VT100 {
    #[must_use]
    pub fn new(out: RawFd) -> Self {
        Self {
            out,
            buffer: String::with_capacity(1024),
        }
    }
}

impl Output for VT100 {
    fn fileno(&self) -> i32 {
        self.out
    }

    fn encoding(&self) -> &'static str {
        "utf-8"
    }

    fn write(&mut self, data: &str) {
        let sanitized = data.replace('\x1b', "?");
        self.write_raw(&sanitized);
    }

    fn write_raw(&mut self, data: &str) {
        self.buffer.push_str(data);
        self.flush();
    }

    fn set_title(&mut self, title: &str) {
        self.write_raw(&format!(
            "\x1b]2;{}\x07",
            title.replace(['\x1b', '\x07'], "")
        ));
    }

    fn clear_title(&mut self) {
        self.set_title("");
    }

    fn flush(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let mut bytes = self.buffer.as_bytes();
        while !bytes.is_empty() {
            match write(unsafe { BorrowedFd::borrow_raw(self.out) }, bytes) {
                Ok(0) => {
                    warn!("wrote 0 bytes!");
                    return;
                }
                Ok(n) => bytes = &bytes[n..],
                Err(Errno::EINTR) => {}
                Err(r) => {
                    error!("unspecified error writing to output: {}", r);
                    return;
                }
            }
        }
        self.buffer.clear();
    }

    fn erase_screen(&mut self) {
        self.write_raw("\x1b[2J");
    }

    fn enter_alternate_screen(&mut self) {
        self.write_raw("\x1b[?1049h\x1b[H");
    }

    fn quit_alternate_screen(&mut self) {
        self.write_raw("\x1b[?1049l");
    }

    fn enable_mouse_support(&mut self) {
        self.write_raw("\x1b[?1000h");

        // Enable mouse-drag support.
        self.write_raw("\x1b[?1003h");

        // Enable urxvt Mouse mode. (For terminals that understand this.)
        self.write_raw("\x1b[?1015h");

        // Also enable Xterm SGR mouse mode. (For terminals that understand this.)
        self.write_raw("\x1b[?1006h");
    }

    fn disable_mouse_support(&mut self) {
        self.write_raw("\x1b[?1000l");
        self.write_raw("\x1b[?1015l");
        self.write_raw("\x1b[?1006l");
        self.write_raw("\x1b[?1003l");
    }

    fn erase_end_of_line(&mut self) {
        self.write_raw("\x1b[K");
    }

    fn erase_down(&mut self) {
        self.write_raw("\x1b[J");
    }

    fn reset_attributes(&mut self) {
        self.write_raw("\x1b[0m");
    }

    fn set_attributes(&mut self, attrs: Attrs, color_depth: ColorDepth) {
        let escape_code = color_depth.escape_code(attrs);
        self.write_raw(&escape_code);
    }

    fn disable_autowrap(&mut self) {
        self.write_raw("\x1b[?7l");
    }

    fn enable_autowrap(&mut self) {
        self.write_raw("\x1b[?7h");
    }

    fn cursor_goto(&mut self, row: usize, column: usize) {
        self.write_raw(&format!("\x1b[{row};{column}H"));
    }

    fn cursor_up(&mut self, count: usize) {
        self.write_raw(&format!("\x1b[{count}A"));
    }

    fn cursor_down(&mut self, count: usize) {
        self.write_raw(&format!("\x1b[{count}B"));
    }

    fn cursor_forward(&mut self, count: usize) {
        self.write_raw(&format!("\x1b[{count}C"));
    }

    fn cursor_back(&mut self, count: usize) {
        self.write_raw(&format!("\x1b[{count}D"));
    }

    fn hide_cursor(&mut self) {
        self.write_raw("\x1b[?25l");
    }

    fn show_cursor(&mut self) {
        self.write_raw("\x1b[?25l\x1b[?25h");
    }

    fn set_cursor_shape(&mut self, cursor_shape: super::CursorShape) {
        if matches!(cursor_shape, CursorShape::NeverChange) {
            return;
        }

        let shape_code = match cursor_shape {
            CursorShape::Block => "\x1b[2 q",
            CursorShape::Beam => "\x1b[6 q",
            CursorShape::Underline => "\x1b[4 q",
            CursorShape::BlinkingBlock => "\x1b[1 q",
            CursorShape::BlinkingBeam => "\x1b[5 q",
            CursorShape::BlinkingUnderline => "\x1b[3 q",
            CursorShape::NeverChange => "",
        };
        self.write_raw(shape_code);
    }

    fn reset_cursor_shape(&mut self) {
        self.write_raw("\x1b[0 q");
    }

    fn supports_cursor_position_requests(&self) -> bool {
        true
    }

    fn request_cursor_position(&mut self) {
        // Request cursor position
        self.write_raw("\x1b[6n");
        self.flush();
    }

    fn get_size(&self) -> super::base::Size {
        let mut ws = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        unsafe {
            if ioctl(self.out, TIOCGWINSZ, &mut ws) == 0 {
                super::base::Size {
                    rows: ws.ws_row as usize,
                    columns: ws.ws_col as usize,
                }
            } else {
                super::base::Size {
                    rows: 0,
                    columns: 0,
                }
            }
        }
    }

    fn get_default_color_depth() -> super::ColorDepth {
        todo!()
    }
}
