use anyhow::Context;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn setup_telemetry() -> anyhow::Result<()> {
    let formatting_layer = BunyanFormattingLayer::new("atto-chat".into(), std::io::stdout);
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).with_context(|| "Failed to set tracing subscriber")?;

    LogTracer::init()?;
    Ok(())
}
