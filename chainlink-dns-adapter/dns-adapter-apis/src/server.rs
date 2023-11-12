use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

use crate::dns_controller::dns_resolver::DnsResolver;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!!!")
}

pub async fn run_server() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "9001".to_string());
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/domain", web::post().to(DnsResolver::resolve))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run();

    println!("Server running on port {}", port);

    server.await
}
