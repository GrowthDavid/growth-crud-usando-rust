use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions};
use std::env;

mod handlers;
mod models;
mod routes;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut client_options = ClientOptions::parse(&database_url).await.expect("Failed to parse options");
    client_options.app_name = Some("RustMongoCRUD".to_string());
    let client = Client::with_options(client_options).expect("Failed to initialize client");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/", web::get().to(hello))
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
