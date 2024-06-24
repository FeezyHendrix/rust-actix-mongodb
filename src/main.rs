use actix_web::{App, HttpServer};
use MongoDB::{options::ClientOptions, Client};
use std::sync::*;

mod controller;
use crate::controller::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let mut client_options = ClientOptions::parse("MongoDB://127.0.0.1:27017/todolist").await.unwrap();
    client_options.app_name = Some("Todolist".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));
    
    HttpServer::new(move || {
        App::new()
            .route("/todos", web::get().to(controller::get_todos))
            .route("/todos", web::post().to(controller::create_todo))
            .route("/todos/{id}", web::get().to(controller::fetch_one))
            .route("/todos/{id}", web::patch().to(controller::update_todo))
            .route("/todos/{id}", web::delete().to(controller::delete_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
