use actix_web::{get, middleware, web, App, HttpServer, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(serde::Deserialize)]
struct CatQuery {
    url: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
async fn index() -> Result<impl Responder> {
    let html = Index.render().expect("Could not render template");
    Ok(Html(html))
}

#[get("/cat")]
async fn cat(query: web::Query<CatQuery>) -> impl Responder {
    format!("{}", query.url)
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
            .service(cat)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
