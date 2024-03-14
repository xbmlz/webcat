use actix_web::{get, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
pub async fn index_handler() -> Result<impl Responder> {
    let html = IndexTemplate.render().expect("Could not render template");
    Ok(Html(html))
}
