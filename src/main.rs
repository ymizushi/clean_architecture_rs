extern crate clean_architecture_rs;

use std::{env};


use actix_web::{
    error, get, Responder, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};

use clean_architecture_rs::adapter::controller::user_controller::show as user_show;


#[get("/")]
pub async fn home(info: web::Path<()>) -> impl Responder {
    Some("hoge")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| App::new()
                    .wrap(middleware::Logger::default())
                    .service(home)
                    .service(user_show))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
