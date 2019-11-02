#![recursion_limit = "128"]
#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use log::{error, info};
use serde::Serialize;
use std::env;

mod error;
mod models;
mod resource;
mod schema;

static DATABASE_URL: &'static str = "DATABASE_URL";
static DOMAIN: &'static str = "DOMAIN";
static PORT: &'static str = "PORT";

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

    let domain = env::var(DOMAIN).unwrap_or("localhost".to_string());
    let port = env::var(PORT).unwrap_or("8080".to_string());

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Pool creation failed.");

    HttpServer::new(move || {
        info!("Starting HTTP server");
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/api").route(web::get().to(index)))
            .service(
                web::scope("/api")
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
    .bind(&format!("{}:{}", domain, port))
    .unwrap()
    .run()
    .unwrap();
}
