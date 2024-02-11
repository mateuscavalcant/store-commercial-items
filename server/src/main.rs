use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs;
use futures::StreamExt;

async fn index() -> impl Responder {
    // Carrega o conteúdo do arquivo HTML
    let html_content = fs::read_to_string("templates/index.html")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read HTML file"));

    HttpResponse::Ok().content_type("text/html").body(html_content.unwrap())
}

async fn upload_image(mut payload: web::Payload) -> impl Responder {
    // Salva o payload recebido em um vetor de bytes
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
    }

    // Converte os bytes para base64
    let base64_image = base64::encode(&bytes);


    // Retorna a imagem no formato base64 na resposta HTTP
    HttpResponse::Ok().body(base64_image)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
