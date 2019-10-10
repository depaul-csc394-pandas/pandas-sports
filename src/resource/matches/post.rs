use crate::{
    error::{self, ServiceError},
    models, Pool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Params {
    #[serde(rename(deserialize = "match"))]
    match_: models::NewMatch,
}

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "match")]
    match_: models::Match,
}

impl Responder for Response {
    type Error = actix_web::Error;
    type Future = Result<HttpResponse, actix_web::Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Created()
            .content_type("application/json")
            .body(body))
    }
}

fn query(params: web::Json<Params>, pool: web::Data<Pool>) -> Result<models::Match, ServiceError> {
    use crate::schema::matches::dsl::*;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    let conn = pool.get().map_err(error::unavailable)?;
    let match_ = diesel::insert_into(matches)
        .values(&params.match_)
        .get_result(&conn)
        .map_err(|e| match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => error::conflict(e),
            e => error::internal(e),
        })?;

    Ok(match_)
}

pub fn post(
    params: web::Json<Params>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(params, pool)).then(move |res| match res {
        Ok(m) => Ok(Response { match_: m }),
        Err(e) => Err(error::from_blocking(e)),
    })
}
