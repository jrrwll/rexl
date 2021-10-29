mod arg;
mod assets;
mod handler;

use crate::arg::Config;
use crate::handler::index;
use actix_web::{web, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");

    let args = env::args().skip(1).collect();
    let config = Config::from_args(args);

    let Config {
        base_path,
        bind_address,
    } = config;

    println!("serving {} on {}", base_path, bind_address);
    HttpServer::new(move || App::new().route(&base_path, web::get().to(index)))
        .bind(bind_address)?
        .run()
        .await
}
