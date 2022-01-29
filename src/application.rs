use crate::infrastructure::telemetry::setup_telemetry;
use crate::interface::http::server::create_server;
use actix_web::dev::Server;
use std::net::TcpListener;

pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    pub fn new(host: &str, port: u16) -> anyhow::Result<Application> {
        let listener = TcpListener::bind((host, port))?;
        let port = listener.local_addr()?.port();
        let server = create_server(listener)?;
        Ok(Self { server, port })
    }

    pub async fn start(self, enable_tracing: bool) -> anyhow::Result<()> {
        if enable_tracing {
            setup_telemetry()?;
        }
        self.server.await?;
        Ok(())
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
