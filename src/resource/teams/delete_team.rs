use crate::{
    error::{self, ServiceError},
    Pool,
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathParams {
    id: i32,
}

fn query(path_params: web::Path<PathParams>, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    use crate::schema::teams::dsl::*;
    let conn = pool.get().map_err(error::unavailable)?;

    diesel::delete(teams.find(path_params.id))
        .execute(&conn)
        .map_err(error::from_diesel)?;

    Ok(())
}

pub fn delete_team(
    path_params: web::Path<PathParams>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || query(path_params, pool)).then(move |res| match res {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Err(error::from_blocking(e)),
    })
}
