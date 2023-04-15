use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Result};

// importing our API route
// mod api {
//     pub mod routes {
//         pub mod users;
//     }
// }

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("ui/dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("../", "ui/dist").index_file("index.html"))
            // .service(api::routes::users::get_users) // Register users
            // .service(Files::new("../ui/dist/assets", ".").show_files_listing()) // this did not work..
            .service(Files::new("/assets", "ui/dist/assets").show_files_listing())// static file handling
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}