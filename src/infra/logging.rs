use anyhow::Result;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_logging() -> Result<WorkerGuard> {
    // 1) File appender (rolling daily)
    let file_appender = rolling::daily("logs", "centichain.log");
    let (file_nb, guard) = tracing_appender::non_blocking(file_appender);

    // 2) File layer
    let file_layer = fmt::layer()
        .with_writer(file_nb)
        .with_ansi(false)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .compact();

    // 3) Console layer (stdout یا stderr هرکدوم خواستی)
    let console_layer = fmt::layer()
        .with_writer(std::io::stderr) // یا std::io::stdout
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_file(false)
        .with_line_number(false)
        .compact();

    // 4) EnvFilter: مثل RUST_LOG=info,centichain=debug,libp2p=warn
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    Ok(guard) // توی main نگهش دار تا بافرها Flush بشن
}
