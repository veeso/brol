#[macro_use]
extern crate serde;

mod database;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web::start("127.0.0.1", 3000).await
}
