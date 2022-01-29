use crate::infrastructure::telemetry::setup_telemetry;
use crate::interface::http::server::create_server;
use actix_web::dev::Server;
use std::net::TcpListener;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(host: &str, port: u16) -> anyhow::Result<Application> {
        let listener = TcpListener::bind((host, port))?;
        let server = create_server(listener)?;
        Ok(Self { server })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        setup_telemetry()?;
        self.server.await?;
        Ok(())
    }
}
