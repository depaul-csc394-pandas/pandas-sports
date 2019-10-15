use crate::{
    error::{self, ServiceError},
    models, Pool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PathParams {
    id: i32,
}

#[derive(Serialize)]
pub struct Response {
    #[serde(flatten, rename = "match")]
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

fn query(path: web::Path<PathParams>, pool: web::Data<Pool>) -> Result<Response, ServiceError> {
    use crate::schema::matches::dsl::*;
    let conn = pool.get().map_err(error::unavailable)?;

    let match_ = matches
        .find(path.id)
        .get_result(&conn)
        .map_err(error::from_diesel)?;

    Ok(Response { match_ })
}

pub fn get_match(
    path: web::Path<PathParams>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(path, pool)).then(move |res| match res {
        Ok(r) => Ok(r),
        Err(e) => Err(error::from_blocking(e)),
    })
}
