use atto_chat_server::application::Application;
use tracing::error;

#[tokio::main]
async fn main() {
    let _ = run().await.map_err(|err| error!("{:?}", err));
}

async fn run() -> anyhow::Result<()> {
    let application = Application::new("127.0.0.1", 5000)?;
    application.start(true).await?;
    Ok(())
}
