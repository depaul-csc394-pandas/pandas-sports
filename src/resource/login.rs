use crate::{
    error::{self, ServiceError},
    models::sql,
    Pool,
};
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use failure::err_msg;
use futures::Future;
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginParams {
    username: String,
    password: String,
}

fn login_query(
    params: web::Json<LoginParams>,
    pool: web::Data<Pool>,
) -> Result<(sql::User, web::Json<LoginParams>), ServiceError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get().map_err(error::unavailable)?;
    let user = users
        .filter(username.eq(&params.username))
        .get_result(&conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => error::unauthorized(e),
            e => error::internal(e),
        })?;
    Ok((user, params))
}

pub fn login(
    params: web::Json<LoginParams>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(|| login_query(params, pool)).then(move |res| match res {
        Ok((user, p)) => {
            if user.verify(&p.password) {
                identity.remember(user.username.clone());
                info!("Logged in user {}", user.username);
                Ok(HttpResponse::Ok().finish())
            } else {
                Err(error::unauthorized(err_msg("User verification failed")))
            }
        }

        Err(e) => Err(error::from_blocking(e)),
    })
}
