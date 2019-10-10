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
    id: i32,
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

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn query(params: web::Json<Params>, pool: web::Data<Pool>) -> Result<models::Match, ServiceError> {
    use crate::schema::matches::dsl::*;
    let conn = pool.get().map_err(error::unavailable)?;
    let match_ = matches
        .find(&params.id)
        .get_result(&conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => error::not_found(e),
            e => error::internal(e),
        })?;

    Ok(match_)
}

pub fn get(
    params: web::Json<Params>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(params, pool)).then(move |res| match res {
        Ok(m) => Ok(Response { match_: m }),
        Err(e) => Err(error::from_blocking(e)),
    })
}
