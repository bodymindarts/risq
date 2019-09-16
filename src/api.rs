use actix_web::{web, App, HttpServer};
use std::io;

pub fn listen(port: u16) -> io::Result<()> {
    HttpServer::new(|| App::new().route("/ping", web::get().to(|| "hello")))
        .bind(("127.0.0.1", port))?
        .run()
}
