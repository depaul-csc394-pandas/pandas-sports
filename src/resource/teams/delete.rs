use crate::{
    error::{self, ServiceError},
    Pool,
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    id: i32,
}

fn query(params: web::Json<Params>, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    use crate::schema::teams::dsl::*;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};
    let conn = pool.get().map_err(error::unavailable)?;

    diesel::delete(teams.find(&params.id))
        .execute(&conn)
        .map_err(|err| match err {
            DieselError::NotFound => error::not_found(err),
            DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => {
                error::conflict(err)
            }
            e => error::internal(e),
        })?;

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
