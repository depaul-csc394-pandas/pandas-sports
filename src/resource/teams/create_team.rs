use crate::{
    error::{self, ServiceError},
    models::sql,
    Pool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Body {
    team: sql::NewTeam,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Response {
    team: sql::Team,
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

fn query(body: web::Json<Body>, pool: web::Data<Pool>) -> Result<Response, ServiceError> {
    use crate::schema::teams::dsl::*;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    let conn = pool.get().map_err(error::unavailable)?;
    let response = match body.into_inner() {
        Body { team: r_team } => {
            let team = diesel::insert_into(teams)
                .values(&r_team)
                .get_result(&conn)
                .map_err(|e| match e {
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        error::conflict(e)
                    }
                    e => error::internal(e),
                })?;

            Response { team }
        }
    };

    Ok(response)
}

pub fn create_team(
    body: web::Json<Body>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(body, pool)).then(move |res| match res {
        Ok(r) => Ok(r),
        Err(e) => Err(error::from_blocking(e)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_body() {
        let src = json!({
            "team": {
                "team_name": "Pandas",
            }
        });

        assert_eq!(
            Body::deserialize(src).expect("deserialization failed"),
            Body {
                team: sql::NewTeam {
                    team_name: "Pandas".to_string(),
                }
            }
        );
    }
}
