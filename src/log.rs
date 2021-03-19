use lazy_static::lazy_static;
use std::sync::Once;

lazy_static! {
    static ref INIT: Once = Once::new();
}

pub fn start_logger() {
    INIT.call_once(|| {
        // install global collector configured based on RUST_LOG env var.
        tracing_subscriber::fmt::init();
    });
}
