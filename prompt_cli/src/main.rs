use std::{fs::OpenOptions, os::fd::IntoRawFd, path::PathBuf};

use nix::unistd::isatty;
use prompt_toolkit::{render, Output, Input, WritePosition, output::VT100 as VT100Output, input::VT100 as VT100Input};
use tracing::{info, Level};
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
};

fn is_tty(fd: i32) -> bool {
    isatty(fd).unwrap_or(false)
}

fn main() {
    let log_dir = PathBuf::from("./log");
    let registry = tracing_subscriber::registry();
    let appender = tracing_appender::rolling::RollingFileAppender::builder()
        .filename_prefix("ptrs")
        .rotation(Rotation::DAILY)
        .max_log_files(50)
        .build(log_dir)
        .expect("valid appender")
        .with_max_level(Level::INFO);
    registry
        .with(tracing_subscriber::fmt::layer().with_writer(appender))
        .init();

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
    let mut input = VT100Input::new(in_fd);
    let mut output = VT100Output::new(out);

    output.set_title("Prompt Toolkit mini-demo");
    std::thread::sleep(std::time::Duration::from_secs(10));

    let key_presses = input.read_keys();
    let mut additional = String::new();
    for key_press in key_presses {
        additional.push_str(key_press.text());
    }

    let size = output.get_size();
    let mut screen = prompt_toolkit::Screen::new(None, size.columns, 1);
    let wp = WritePosition::new(0, 0, size.columns, 1);
    let data = format!("Hello, terminal, you entered: {}", additional);
    output.set_title("Prompt Toolkit RS");
    screen.direct_draw(&wp, &data);
    render::output_screen(&mut output, &screen, &size);
}
