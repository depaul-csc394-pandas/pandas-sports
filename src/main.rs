#![recursion_limit = "128"]
#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use lazy_static::lazy_static;
use log::{error, info};
use ring::rand::SystemRandom;
use serde::Serialize;
use std::env;

mod error;
mod hash;
mod models;
mod resource;
mod schema;
mod secret;
mod user;

static BIND_ADDR: &'static str = "BIND_ADDR";
static DATABASE_URL: &'static str = "DATABASE_URL";
static DOMAIN: &'static str = "DOMAIN";
static PORT: &'static str = "PORT";

lazy_static! {
    static ref RNG: SystemRandom = SystemRandom::new();
}

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct IndexResponse {
    version: String,
}

fn index() -> actix_web::Result<web::Json<IndexResponse>> {
    Ok(web::Json(IndexResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

fn is_first_run(pool: Pool) -> bool {
    let conn = pool
        .get()
        .expect("is_first_run: Failed to get connection from pool");
    let admin: Option<models::sql::User> = schema::users::table
        .find(1)
        .get_result(&conn)
        .optional()
        .unwrap();
    admin.is_none()
}

fn create_admin(pool: Pool) {
    let username = "admin";
    let password = env::var("ADMIN_PASS").expect("ADMIN_PASS not set");

    let hash::SaltedHash { salt, hash } = hash::SaltedHash::from_password(&password);
    let new_user = models::sql::NewUser {
        privileged: true,
        username: username.to_string(),
        salt_base64: base64::encode(&salt),
        argon2_hash: String::from_utf8(hash).expect("Hash parse failed"),
    };

    let conn = pool
        .get()
        .expect("create_admin: Failed to get connection from pool");
    let user: models::sql::User = diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(&conn)
        .expect("create_admin: Failed to insert admin user");

    if user.id != 1 {
        error!(
            "Admin user id is not 1; original admin account was likely deleted. \
             Please reset the database."
        );
        std::process::exit(1);
    }
}

fn main() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "pandas_sports=info,actix_web=info,diesel=info");
    env_logger::init();

    let db_url = match env::var(DATABASE_URL) {
        Ok(v) => v,
        Err(e) => {
            error!("{}: {}", DATABASE_URL, e);
            std::process::exit(1);
        }
    };

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Pool creation failed.");

    if is_first_run(pool.clone()) {
        create_admin(pool.clone());
    }

    let bind_addr = env::var(BIND_ADDR).unwrap_or("0.0.0.0".to_string());
    let domain = env::var(DOMAIN).unwrap_or("localhost".to_string());
    let port = env::var(PORT).unwrap_or("8080".to_string());

    HttpServer::new(move || {
        info!("Starting HTTP server");
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(Cors::new().allowed_methods(vec!["GET", "POST", "DELETE"]))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&*secret::COOKIE_KEY)
                    .name("pandas-auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false),
            ))
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/api").route(web::get().to(index)))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/register")
                            .route(web::post().to_async(resource::register::register)),
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to_async(resource::login::login)),
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::post().to_async(resource::logout::logout)),
                    )
                    .service(
                        web::resource("/matches")
                            .route(web::get().to_async(resource::matches::list_matches))
                            .route(web::post().to_async(resource::matches::create_match)),
                    )
                    .service(
                        web::resource("/matches/{id}")
                            .route(web::get().to_async(resource::matches::get_match))
                            .route(web::delete().to_async(resource::matches::delete_match))
                    )
                    .service(
                        web::resource("/teams")
                            .route(web::get().to_async(resource::teams::list_teams))
                            .route(web::post().to_async(resource::teams::create_team)),
                    )
                    .service(
                        web::resource("/teams/{id}")
                            .route(web::get().to_async(resource::teams::get_team))
                            .route(web::delete().to_async(resource::teams::delete_team))
                    )
                    .service(
                        web::resource("/players")
                            // .route(web::delete().to_async(resource::teams::delete))
                            // .route(web::get().to_async(resource::teams::get))
                            // .route(web::post().to_async(resource::teams::post)),
                    ),
            )
    })
    .bind(&format!("{}:{}", bind_addr, port))
    .unwrap()
    .run()
    .unwrap();
}
