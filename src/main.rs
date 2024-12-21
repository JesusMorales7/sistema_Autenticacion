use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use actix_session::{Session};
use actix_session::storage::CookieSession; // Asegúrate de usar el import correcto
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use actix_files::Files;

#[macro_use]
extern crate diesel;

mod models;
mod schema;

#[derive(Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(CookieSession::private(&[0; 32]))  // Usa CookieSession correctamente
            .route("/", web::get().to(hello))  // Reemplaza el closure por una función
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/dashboard", web::get().to(dashboard))
            .service(Files::new("/static", "./static").show_files_listing()) // Archivos estáticos
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Define la función para la ruta "/"
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Sistema de Autenticación")
}

async fn register_user(pool: web::Data<DbPool>, item: web::Json<NewUser>) -> impl Responder {
    use schema::users;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let hashed_password = hash(&item.password, DEFAULT_COST).unwrap();

    let new_user = models::NewUser {
        username: &item.username,
        password_hash: &hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn)
        .expect("Error inserting new user");

    HttpResponse::Created().finish()
}

async fn login_user(pool: web::Data<DbPool>, item: web::Json<LoginCredentials>, session: Session) -> impl Responder {
    use schema::users::dsl::*;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let user = users.filter(username.eq(&item.username))
        .first::<models::User>(&conn)
        .ok();

    match user {
        Some(user) if verify(&item.password, &user.password_hash).unwrap() => {
            session.insert("user_id", user.id).unwrap();
            HttpResponse::Ok().body("Login successful")
        }
        _ => HttpResponse::Unauthorized().body("Invalid username or password"),
    }
}

async fn dashboard(session: Session) -> impl Responder {
    match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => HttpResponse::Ok().body(format!("Welcome, user {}", user_id)),
        _ => HttpResponse::Unauthorized().body("Please log in"),
    }
}
