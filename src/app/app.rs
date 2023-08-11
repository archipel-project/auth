use actix_web::{App, HttpServer, web::Data};

use crate::service::config::env::Env;
use crate::service::routes::routes;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Env::load()?;
    let server = config.get_server();

    log::debug!("Serving at http://{}", server);

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(config.clone()))
        .configure(routes::config)
    })
    .bind(server)?
    .run()
    .await?;

    Ok(())
}