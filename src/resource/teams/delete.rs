use crate::{
    error::{self, ServiceError},
    Pool,
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Params {
    Team { id: i32 },
}

fn query(params: web::Json<Params>, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    use crate::schema::teams::dsl::*;
    let conn = pool.get().map_err(error::unavailable)?;

    match params.into_inner() {
        Params::Team { id: r_id } => {
            diesel::delete(teams.find(r_id))
                .execute(&conn)
                .map_err(error::from_diesel)?;
        }
    }

    Ok(())
}

pub fn delete(
    params: web::Json<Params>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || query(params, pool)).then(move |res| match res {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Err(error::from_blocking(e)),
    })
}
