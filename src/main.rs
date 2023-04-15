use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Result};

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("ui/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("../", "ui").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}