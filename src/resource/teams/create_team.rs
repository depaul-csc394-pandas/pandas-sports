use crate::{
    error::{self, ServiceError},
    models::{api, sql},
    Pool,
};
use actix_identity::Identity;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Body {
    team: api::PostTeam,
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

fn query(
    body: web::Json<Body>,
    pool: web::Data<Pool>,
    user_id: i32,
) -> Result<Response, ServiceError> {
    use crate::schema::teams::dsl::*;
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    let conn = pool.get().map_err(error::unavailable)?;
    let response = match body.into_inner() {
        Body { team: r_team } => {
            let new_team = sql::NewTeam::with_owner(r_team, user_id);
            let team = diesel::insert_into(teams)
                .values(new_team)
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
    identity: Identity,
) -> Result<Response, ServiceError> {
    let user_id = crate::user::id_for_identity(pool.clone(), identity.clone())?;

    match query(body, pool, user_id) {
        Ok(r) => Ok(r),
        Err(e) => Err(e),
    }
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
                team: api::PostTeam {
                    team_name: "Pandas".to_string(),
                }
            }
        );
    }
}
