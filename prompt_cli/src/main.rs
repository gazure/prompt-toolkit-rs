use std::{fs::OpenOptions, os::fd::IntoRawFd, path::PathBuf};

use nix::unistd::isatty;
use prompt_toolkit::{render, Output, WritePosition, VT100};
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
    let (_in_fd, _is_in_tty, out, _is_out_tty, _close_on_drop) = if let Ok(tty) = tty {
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
    let mut output = VT100::new(out);

    let size = output.get_size();
    let mut screen = prompt_toolkit::Screen::new(None, size.columns, 1);
    let wp = WritePosition::new(0, 0, size.columns, 1);
    let data = "Hello, terminal";
    output.set_title("Prompt Toolkit RS");
    screen.direct_draw(&wp, data);
    render::output_screen(&mut output, &screen, &size);
}
