use crate::infrastructure::telemetry::setup_telemetry;
use crate::interface::http::server::create_server;
use actix_web::dev::Server;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(host: &str, port: u16) -> anyhow::Result<Application> {
        let server = create_server(host, port)?;
        Ok(Self { server })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        setup_telemetry()?;
        self.server.await?;
        Ok(())
    }
}
