mod dns_controller;
mod server;
mod services;
mod utils;
use server::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}
