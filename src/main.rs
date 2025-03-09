use tokio::io;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;
use sse::App;

pub mod broker;
pub mod system;
pub mod sse;
pub mod clap;
#[tokio::main]
async fn main() -> io::Result<()> {
    // Set up file appender for logging
    // let file_appender = RollingFileAppender::new(Rotation::DAILY, "/var/log/llmidium", "server.log");

    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        // .with_writer(file_appender)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
    let args = clap::args();
    let listener = tokio::net::TcpListener::bind(args.bind()).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, App::new().router()).await
}

#[macro_export]
macro_rules! embed {
    (
        $path: literal
    ) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/embed/",
            $path
        ))
    };
}