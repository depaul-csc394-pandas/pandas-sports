use crate::{
    error::{self, ServiceError},
    models, Pool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Params {
    Team {
        #[serde(flatten)]
        team: models::NewTeam
    },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Team { team: models::Team },
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

fn query(params: web::Json<Params>, pool: web::Data<Pool>) -> Result<Response, ServiceError> {
    use crate::schema::teams::dsl::*;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    let conn = pool.get().map_err(error::unavailable)?;
    let response = match params.into_inner() {
        Params::Team { team: r_team } => {
            let team = diesel::insert_into(teams)
                .values(&r_team)
                .get_result(&conn)
                .map_err(|e| match e {
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => error::conflict(e),
                    e => error::internal(e),
                })?;

            Response::Team { team }
        }
    };

    Ok(response)
}

pub fn post(
    params: web::Json<Params>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(params, pool)).then(move |res| match res {
        Ok(r) => Ok(r),
        Err(e) => Err(error::from_blocking(e)),
    })
}
