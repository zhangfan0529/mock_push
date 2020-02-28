use std::process;

use actix_web::{App, HttpServer, web};
use mock_push::init_rule_data;
use mock_push::init_data;
use mock_push::req::{create_push_rule, accept_push, push_record};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting... pid:{}", process::id());
    let rule_data = web::Data::new(init_rule_data());
    let push_list = web::Data::new(init_data());
    HttpServer::new( move || {
        App::new()
            .app_data(rule_data.clone())
            .app_data(push_list.clone())
            .route("/push", web::post().to(create_push_rule))
            .route("/mock/{id}", web::post().to(accept_push))
            .route("/list/{id}", web::post().to(push_record))
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}



