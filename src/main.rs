use actix_web::{get, middleware, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder  {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default().log_target("http_log"))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
