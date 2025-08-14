use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    let filter = std::env::var("MONKEY_TYPER_LOG")
        .or_else(|_| std::env::var("RUST_LOG"))
        .unwrap_or_else(|_| "info".to_string());

    let env_filter = EnvFilter::new(filter);

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .without_time()
        .init();
}