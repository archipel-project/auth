extern crate log;

mod app;
mod service;
mod utils;

use app::app as application;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().unwrap();
    pretty_env_logger::init();

    match application::run().await {
        Err(err) => {
            log::error!("Error: {}", err);
            std::process::exit(1);
        },
        Ok(_) => {},
    }
}