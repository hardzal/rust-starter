use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

type Db = Mutex<Vec<User>>;

#[get("/users")]
async fn get_users(db: web::Data<Db>) -> impl Responder {
    let users = db.lock().unwrap();
    HttpResponse::Ok().json(users.clone())
}

#[get("/users/{id}")]
async fn get_user(db: web::Data<Db>, id: web::Path<Uuid>) -> impl Responder {
    let users = db.lock().unwrap();
    match users.iter().find(|user| user.id == *id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json("User not found"),
    }
}

#[post("/users")]
async fn create_user(db: web::Data<Db>, body: web::Json<CreateUserRequest>) -> impl Responder {
    let mut users = db.lock().unwrap();

    let user = User {
        id: Uuid::new_v4(),
        name: body.name.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
    };

    users.push(user.clone());
    HttpResponse::Created().json(user)
}

#[put("/users/{id}")]
async fn update_user(
    db: web::Data<Db>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let mut users = db.lock().unwrap();

    match users.iter_mut().find(|user| user.id == *id) {
        Some(user) => {
            if let Some(name) = &body.name {
                user.name = name.clone();
            }

            if let Some(email) = &body.email {
                user.email = email.clone();
            }

            HttpResponse::Ok().json(user)
        }
        None => HttpResponse::NotFound().json("User not found"),
    }
}

#[delete("/users/{id}")]
async fn delete_user(db: web::Data<Db>, id: web::Path<Uuid>) -> impl Responder {
    let mut users = db.lock().unwrap();
    let user_len = users.len();

    users.retain(|user| user.id != *id);
    if users.len() == user_len {
        HttpResponse::NotFound().json("User not found")
    } else {
        HttpResponse::NoContent().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: web::Data<Db> = web::Data::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(get_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8008))?
    .run()
    .await
}
