mod handler;
mod qsegment_constructor;
mod dotenv_tools;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct BasicResponse {
    message: String,
    code: u8,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on port 9912");
    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/upload")
                    .route(web::post().to(handler::Backend::handle_input)),
            )
            .route("/", web::get().to(index))
    })
        .bind("0.0.0.0:9912")?
        .run()
        .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
