use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;

mod domain;
mod application;
mod infrastructure;

use application::user_service::UserUseCase;
use infrastructure::{
    http::user_handler,
    persistence::user_repository_pg::UserRepositoryPg,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:@localhost:5432/user_api")
        .await
        .unwrap();

    let user_repo = UserRepositoryPg::new(pool.clone());
    let user_usecase = web::Data::new(UserUseCase::new(user_repo));

    HttpServer::new(move || {
        App::new()
            .app_data(user_usecase.clone())
            .route("/users", web::post().to(user_handler::create_user))
            .route("/users", web::get().to(user_handler::list_users))
    })
    .bind(("127.0.0.1", 8008))?
    .run()
    .await
}