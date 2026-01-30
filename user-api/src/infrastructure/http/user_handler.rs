use crate::application::user_service::UserUseCase;
use crate::infrastructure::persistence::user_repository_pg::UserRepositoryPg;
use crate::domain::entity::user::User;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
}

pub async fn create_user(
    uc: web::Data<UserUseCase<UserRepositoryPg>>,
    payload: web::Json<CreateUserRequest>,
) -> impl Responder {
    match uc
        .create_user(payload.name.clone(), payload.email.clone())
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_users(uc: web::Data<UserUseCase<UserRepositoryPg>>) -> impl Responder {
    match uc.list_users().await {
        Ok(users) => {
            let response: Vec<User> = users
                .into_iter()
                .map(|user| User {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                })
                .collect();

            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
