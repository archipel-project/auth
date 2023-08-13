use actix_web::{App, HttpServer, web::Data};

use crate::{service::routes::routing, app::{database, config::env::Env}};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Env::load()?;
    let server = config.get_server();

    let database = database::builder().await?;
    database::migrate(&database).await?;

    log::debug!("Serving at http://{}", server);

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(config.clone()))
        .app_data(Data::new(database.clone()))
        .configure(routing::register)
    })
    .bind(server)?
    .run()
    .await?;

    Ok(())
}