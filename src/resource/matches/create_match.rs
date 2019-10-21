use crate::{
    error::{self, ServiceError},
    models::{api, sql},
    Pool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Params {
    #[serde(rename(deserialize = "match"))]
    match_: api::PostMatch,
}

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "match")]
    match_: api::GetMatch,
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

fn query(params: web::Json<Params>, pool: web::Data<Pool>) -> Result<sql::Match, ServiceError> {
    let conn = pool.get().map_err(error::unavailable)?;

    let mut new_match = sql::NewMatch::from(params.match_.clone());
    match params.match_.details {
        api::PostMatchDetails::Baseball { ref details } => {
            use crate::schema::baseball::dsl;
            let sql::Baseball { id, .. } = diesel::insert_into(dsl::baseball)
                .values(details)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            new_match.baseball_id = Some(id);
        }
        api::PostMatchDetails::Basketball { ref details } => {
            use crate::schema::basketball::dsl;
            let sql::Basketball { id, .. } = diesel::insert_into(dsl::basketball)
                .values(details)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            new_match.basketball_id = Some(id);
        }
        api::PostMatchDetails::Football { ref details } => {
            use crate::schema::football::dsl;
            let sql::Football { id, .. } = diesel::insert_into(dsl::football)
                .values(details)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            new_match.football_id = Some(id);
        }
        api::PostMatchDetails::Hockey { ref details } => {
            use crate::schema::hockey::dsl;
            let sql::Hockey { id, .. } = diesel::insert_into(dsl::hockey)
                .values(details)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            new_match.hockey_id = Some(id);
        }
        api::PostMatchDetails::Soccer { ref details } => {
            use crate::schema::soccer::dsl;
            let sql::Soccer { id, .. } = diesel::insert_into(dsl::soccer)
                .values(details)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            new_match.soccer_id = Some(id);
        }
        api::PostMatchDetails::Volleyball { ref details } => {
            use crate::schema::volleyball::dsl;
            let sql::Volleyball { id, .. } = diesel::insert_into(dsl::volleyball)
                .values(details)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            new_match.volleyball_id = Some(id);
        }
    }

    // this block is to stop the glob import messing up references to `id`
    let match_ = {
        use crate::schema::matches::dsl::*;

        diesel::insert_into(matches)
            .values(&new_match)
            .get_result(&conn)
            .map_err(error::from_diesel)?
    };

    Ok(match_)
}

pub fn create_match(
    params: web::Json<Params>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(params, pool)).then(move |res| match res {
        Ok(m) => Ok(Response { match_: m.into() }),
        Err(e) => Err(error::from_blocking(e)),
    })
}
