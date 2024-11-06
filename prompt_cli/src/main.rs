use std::path::PathBuf;

use prompt_toolkit::{
    application::Application, clipboard::Clipboard, key_bindings::{EchoBinding, KeyBindings}, output::ColorDepth
};
use tracing::Level;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
};

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
    use_application();
}


fn use_application() {
    let mut key_bindings = KeyBindings::new();
    key_bindings.add_for_all_keys(Box::new(EchoBinding));
    let clipboard = Clipboard;
    let mut app = Application::new(prompt_toolkit::layout::Layout,
        key_bindings, clipboard, ColorDepth::True, false, prompt_toolkit::filters::Filter::Always);
    app.run();
}
