use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Context;
use tracing::{error, subscriber::set_global_default};
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() {
    let _ = run().await.map_err(|err| error!("{:?}", err));
}

async fn run() -> anyhow::Result<()> {
    let formatting_layer = BunyanFormattingLayer::new("atto-chat".into(), std::io::stdout);
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).with_context(|| "Failed to set tracing subscriber")?;

    LogTracer::init()?;

    let _server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await;

    Ok(())
}
