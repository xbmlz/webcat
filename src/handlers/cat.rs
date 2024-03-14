use actix_web::{get, web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;
use mime_guess::mime;
use urlencoding::decode;

use crate::utils::{download_file, extract_filename_from_url};

#[derive(serde::Deserialize)]
struct CatQuery {
    url: String,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    msg: &'a str,
}

#[derive(Template)]
#[template(path = "image.html")]
struct ImageTemplate<'a> {
    url: &'a str,
}

#[get("/cat")]
pub async fn cat_handler(query: web::Query<CatQuery>) -> Result<impl Responder> {
    let decode_url = decode(&query.url).expect("Could not decode URL");
    let filename = extract_filename_from_url(&decode_url).expect("Could not extract filename");

    let file_type = mime_guess::from_path(filename.clone()).first_or_octet_stream();

    let html = match file_type.type_() {
            mime::IMAGE => ImageTemplate { url: &decode_url }.render(),
            _ => ErrorTemplate { msg: "File type not supported" }.render()
    }
    .map_err(|_| actix_web::error::ErrorInternalServerError("Could not render template"))?;
    // download_file(&decode_url, &format!("./data/{}", filename)).await.expect("Could not download file");
    
    Ok(Html(html))
}
