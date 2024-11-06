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

    #[must_use]
    pub fn color_depth(&self) -> ColorDepth {
        self.color_depth
    }

    pub fn run(&mut self) {
        self.output.set_title("Prompt Toolkit mini-demo");
        std::thread::sleep(std::time::Duration::from_secs(5));

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
        let data = format!("You entered (raw mode not enabled): {}", additional);
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
