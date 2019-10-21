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
    team: models::sql::Team,
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

fn query(
    path_params: web::Path<PathParams>,
    pool: web::Data<Pool>,
) -> Result<Response, ServiceError> {
    use crate::schema::teams::dsl::*;
    let conn = pool.get().map_err(error::unavailable)?;

    let team = teams
        .find(path_params.id)
        .get_result(&conn)
        .map_err(error::from_diesel)?;

    Ok(Response { team })
}

pub fn get_team(
    path_params: web::Path<PathParams>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(path_params, pool)).then(move |res| match res {
        Ok(r) => Ok(r),
        Err(e) => Err(error::from_blocking(e)),
    })
}
