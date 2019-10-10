#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use log::{error, info};
use serde::Serialize;

mod error;
mod models;
mod resource;
mod schema;

static DATABASE_URL: &'static str = "DATABASE_URL";

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Queryable, Serialize)]
pub struct Match {
    pub id: i32,
    pub team_1: i32,
    pub team_1_score: i32,
    pub team_2: i32,
    pub team_2_score: i32,
}

fn main() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "pandas_sports=info,actix_web=info,diesel=info");
    env_logger::init();

    let db_url = match std::env::var(DATABASE_URL) {
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

    HttpServer::new(move || {
        info!("Starting HTTP server");
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/matches")
                            .route(web::delete().to_async(resource::matches::delete))
                            .route(web::get().to_async(resource::matches::get))
                            .route(web::post().to_async(resource::matches::post)),
                    )
                    .service(
                        web::resource("/teams")
                            .route(web::delete().to_async(resource::teams::delete))
                            .route(web::get().to_async(resource::teams::get))
                            .route(web::post().to_async(resource::teams::post)),
                    ),
            )
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .unwrap();
}
