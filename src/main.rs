mod handler;
mod model;
mod response;

use actix_web::{
    web,
    App,
    HttpServer
};

use model::AppState;

#[actix_web::main]
async fn main () -> std::io::Result<()> {
    let app_state = AppState::init();
    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
    })
    .bind("127.0.0.1:7777")?
    .run()
    .await
}