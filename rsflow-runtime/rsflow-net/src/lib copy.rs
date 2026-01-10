use axum::{routing::get, Router};
use std::net::SocketAddr;

pub struct HttpServerManager {
    port: u16,
}

impl HttpServerManager {
    pub fn new(port: u16) -> Self {
        Self {
            port,
        }
    }

    pub async fn start(&self) {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        
        let app = Router::new()
            .route("/", get(|| async { "RSFlow HTTP Service" }))
            .route("/health", get(|| async { "OK" }));

        println!("HTTP server starting on port {}", self.port);
        
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
