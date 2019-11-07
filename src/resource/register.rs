use crate::{
    error::{self, ServiceError},
    hash::SaltedHash,
    models::sql,
    Pool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterParams {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    id: i32,
    username: String,
}

impl Responder for RegisterResponse {
    type Error = actix_web::Error;
    type Future = Result<HttpResponse, actix_web::Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Created()
            .content_type("application/json")
            .body(body))
    }
}

fn query(
    params: web::Json<RegisterParams>,
    pool: web::Data<Pool>,
) -> Result<sql::User, ServiceError> {
    use crate::schema::users::dsl::*;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    let SaltedHash { salt, hash } = SaltedHash::from_password(&params.password);
    let new_user = sql::NewUser {
        privileged: false,
        username: params.username.clone(),
        salt_base64: base64::encode(&salt),
        argon2_hash: String::from_utf8(hash).map_err(error::internal)?,
    };

    let conn = pool.get().map_err(error::unavailable)?;
    let user = diesel::insert_into(users)
        .values(new_user)
        .get_result(&conn)
        .map_err(|e| match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => error::conflict(e),
            e => error::internal(e),
        })?;

    Ok(user)
}

pub fn register(
    params: web::Json<RegisterParams>,
    pool: web::Data<Pool>,
) -> impl Future<Item = RegisterResponse, Error = ServiceError> {
    web::block(move || query(params, pool)).then(move |res| match res {
        Ok(sql::User { id, username, .. }) => Ok(RegisterResponse { id, username }),
        Err(e) => Err(error::from_blocking(e)),
    })
}
