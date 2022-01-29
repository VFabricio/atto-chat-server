use super::routes::health_check::health_check;
use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub fn create_server(host: &str, port: u16) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
    })
    .bind((host, port))?
    .run();

    Ok(server)
}
