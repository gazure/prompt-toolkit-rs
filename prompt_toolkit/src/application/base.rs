#![expect(dead_code)]

use std::{fs::OpenOptions, os::fd::IntoRawFd};

use nix::libc;
use nix::unistd::isatty;
use tracing::info;

use crate::{
    clipboard::Clipboard,
    filters::Filter,
    input::VT100 as VT100Input,
    key_bindings::{KeyBindings, KeyPressEvent},
    layout::Layout,
    output::{ColorDepth, VT100 as VT100Output},
    render, Input, Output, Screen, WritePosition,
};

fn is_tty(fd: i32) -> bool {
    isatty(fd).unwrap_or(false)
}

pub struct Application {
    layout: Layout,
    key_bindings: KeyBindings,
    clipboard: Clipboard,
    color_depth: ColorDepth,
    erase_when_done: bool,
    filter: Filter,
    input: Box<dyn Input>,
    output: Box<dyn Output>,
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
        let tty = OpenOptions::new().read(true).write(true).open("/dev/tty");
        let (in_fd, _is_in_tty, out, _is_out_tty, _close_on_drop) = if let Ok(tty) = tty {
            info!("susing dev/tty");
            let fd = tty.into_raw_fd();
            let is_a_tty = is_tty(fd);
            (fd, is_a_tty, fd, is_a_tty, true)
        } else {
            info!("using stdin/out");
            (
                libc::STDIN_FILENO,
                isatty(libc::STDIN_FILENO).unwrap_or(false),
                libc::STDOUT_FILENO,
                is_tty(libc::STDOUT_FILENO),
                false,
            )
        };
        let input = VT100Input::new(in_fd);
        let output = VT100Output::new(out);
        Self {
            layout,
            key_bindings,
            clipboard,
            color_depth,
            erase_when_done,
            filter,
            input: Box::new(input),
            output: Box::new(output),
        }
    }

    pub fn set_input(&mut self, input: Box<dyn Input>) {
        self.input = input;
    }

    pub fn set_output(&mut self, output: Box<dyn Output>) {
        self.output = output;
    }

    #[must_use]
    pub fn color_depth(&self) -> ColorDepth {
        self.color_depth
    }

    pub fn run(&mut self, wait_for: u64) {
        self.output.set_title("Prompt Toolkit mini-demo");
        std::thread::sleep(std::time::Duration::from_secs(wait_for));

        let mut raw_input = self.input.raw_mode();

        let key_presses = raw_input.read_keys();
        let mut additional = String::new();
        for key_press in key_presses {
            let key_press_event = KeyPressEvent::new(key_press);
            additional.push_str(key_press_event.key_press.text());
            self.key_bindings
                .get_all_keys_bindings_mut()
                .iter_mut()
                .for_each(|binding| {
                    binding.handler(&key_press_event);
                });
        }

        let size = self.output.get_size();
        let mut screen = Screen::new(None, size.columns, 10);
        let mut wp = WritePosition::new(0, 0, size.columns, 1);
        let data = format!("You entered (raw mode not enabled): {additional}");
        screen.direct_draw(&wp, &data, "bold");
        wp.ypos += 1;
        screen.direct_draw(&wp, "italic", "italic");
        wp.ypos += 1;
        screen.direct_draw(
            &wp,
            "Ansi blue background with Ansi red characters",
            "bg:ansiblue fg:ansired",
        );
        wp.ypos += 1;
        screen.direct_draw(
            &wp,
            "greyer text and background using #RRGGBB",
            "bg:#111111 fg:#BBBBBB",
        );
        wp.ypos += 1;
        screen.direct_draw(&wp, "strike", "strike");
        wp.ypos += 1;
        screen.direct_draw(&wp, "underline", "underline");
        wp.ypos += 1;
        screen.direct_draw(&wp, "blink", "blink");
        wp.ypos += 1;
        screen.direct_draw(&wp, "reverse (inverted colors)", "reverse");
        wp.ypos += 1;
        screen.direct_draw(&wp, "hidden", "hidden");
        wp.xpos += 6;
        screen.direct_draw(&wp, "<- there is hidden text there", "nohidden");

        render::output_screen(self.output.as_mut(), &screen, &size);
    }
}

#[cfg(test)]
mod test {
    use std::io::{Read, Seek};

    use super::*;
    use tempfile::tempfile;

    #[test]
    fn test_new_application() {
        let layout = Layout;
        let key_bindings = KeyBindings::new();
        let clipboard = Clipboard;
        let color_depth = ColorDepth::default();

        let mut app = Application::new(
            layout,
            key_bindings,
            clipboard,
            color_depth,
            false,
            Filter::default(),
        );

        let input = tempfile().expect("input temp file");
        let input = VT100Input::new(input.into_raw_fd());
        app.set_input(Box::new(input));

        let output = tempfile().expect("output temp file");
        let mut output_clone = output.try_clone().expect("cloned output temp file");
        let output = VT100Output::new(output.into_raw_fd());
        app.set_output(Box::new(output));

        assert_eq!(app.color_depth(), ColorDepth::default());
        app.run(0);

        output_clone
            .seek(std::io::SeekFrom::Start(0))
            .expect("seek error");
        let mut output_str = String::new();
        output_clone
            .read_to_string(&mut output_str)
            .expect("read error");

        // since output is not a tty, uncertain term size values should swallow the content of the default app
        assert_eq!(output_str, "\u{1b}]2;Prompt Toolkit mini-demo\u{7}\u{1b}[?25l\u{1b}[?7l\u{1b}[0m\u{1b}[?25l\u{1b}[?25h");
    }
}
