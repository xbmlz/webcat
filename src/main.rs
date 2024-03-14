use anyhow::Result;
use webcat::handlers::{cat_handler, index_handler};
use actix_web::{middleware, App, HttpServer};


#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default().log_target("http_log"))
            .service(actix_files::Files::new("/public", "./public"))
            .service(index_handler)
            .service(cat_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
