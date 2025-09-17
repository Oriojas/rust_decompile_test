use actix_web::{web, App, HttpServer};
use log::info;

use rust_decompile_test::handlers::{analysis_handler, decode_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let server_address = "127.0.0.1:8080";

    info!("🚀 Servidor web iniciando en http://{}", server_address);

    HttpServer::new(|| {
        App::new()
            .route("/decode", web::post().to(decode_handler))
            .route("/analysis", web::post().to(analysis_handler))
    })
    .bind(server_address)?
    .run()
    .await
}
