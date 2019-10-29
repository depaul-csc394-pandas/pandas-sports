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

fn query(params: web::Json<Params>, pool: web::Data<Pool>) -> Result<api::GetMatch, ServiceError> {
    let conn = pool.get().map_err(error::unavailable)?;

    let mut new_match = sql::NewMatch::from(params.match_.clone());
    let match_: sql::Match = {
        use crate::schema::matches::dsl::*;

        diesel::insert_into(matches)
            .values(&new_match)
            .get_result(&conn)
            .map_err(error::from_diesel)?
    };

    let details = match params.match_.details {
        api::PostMatchDetails::Baseball {
            ref team_1,
            ref team_2,
        } => {
            use crate::schema::baseball::dsl;
            let t1 = diesel::insert_into(dsl::baseball)
                .values(sql::Baseball {
                    match_id: match_.id,
                    team_id: match_.team_1_id,
                    dummy: team_1.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::baseball)
                .values(sql::Baseball {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    dummy: team_2.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Baseball {
                team_1: t1,
                team_2: t2,
            }
        }

        api::PostMatchDetails::Basketball {
            ref team_1,
            ref team_2,
        } => {
            use crate::schema::basketball::dsl;
            let t1 = diesel::insert_into(dsl::basketball)
                .values(sql::Basketball {
                    match_id: match_.id,
                    team_id: match_.team_1_id,
                    q1: team_1.q1,
                    q2: team_1.q2,
                    q3: team_1.q3,
                    q4: team_1.q4,
                    fgm: team_1.fgm,
                    fga: team_1.fga,
                    tpm: team_1.tpm,
                    tpa: team_1.tpa,
                    ftm: team_1.ftm,
                    fta: team_1.fta,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::basketball)
                .values(sql::Basketball {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    q1: team_2.q1,
                    q2: team_2.q2,
                    q3: team_2.q3,
                    q4: team_2.q4,
                    fgm: team_2.fgm,
                    fga: team_2.fga,
                    tpm: team_2.tpm,
                    tpa: team_2.tpa,
                    ftm: team_2.ftm,
                    fta: team_2.fta,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Basketball {
                team_1: t1,
                team_2: t2,
            }
        }
        api::PostMatchDetails::Football {
            ref team_1,
            ref team_2,
        } => {
            use crate::schema::football::dsl;
            let t1 = diesel::insert_into(dsl::football)
                .values(sql::Football {
                    match_id: match_.id,
                    team_id: match_.team_1_id,
                    dummy: team_1.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::football)
                .values(sql::Football {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    dummy: team_2.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Football {
                team_1: t1,
                team_2: t2,
            }
        }
        api::PostMatchDetails::Hockey {
            ref team_1,
            ref team_2,
        } => {
            use crate::schema::hockey::dsl;
            let t1 = diesel::insert_into(dsl::hockey)
                .values(sql::Hockey {
                    match_id: match_.id,
                    team_id: match_.team_1_id,
                    dummy: team_1.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::hockey)
                .values(sql::Hockey {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    dummy: team_2.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Hockey {
                team_1: t1,
                team_2: t2,
            }
        }
        api::PostMatchDetails::Soccer {
            ref team_1,
            ref team_2,
        } => {
            use crate::schema::soccer::dsl;
            let t1 = diesel::insert_into(dsl::soccer)
                .values(sql::Soccer {
                    match_id: match_.id,
                    team_id: match_.team_1_id,
                    dummy: team_1.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::soccer)
                .values(sql::Soccer {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    dummy: team_2.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Soccer {
                team_1: t1,
                team_2: t2,
            }
        }
        api::PostMatchDetails::Volleyball {
            ref team_1,
            ref team_2,
        } => {
            use crate::schema::volleyball::dsl;
            let t1 = diesel::insert_into(dsl::volleyball)
                .values(sql::Volleyball {
                    match_id: match_.id,
                    team_id: match_.team_1_id,
                    dummy: team_1.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::volleyball)
                .values(sql::Volleyball {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    dummy: team_2.dummy,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Volleyball {
                team_1: t1,
                team_2: t2,
            }
        }
    };

    Ok(api::GetMatch {
        id: match_.id,
        match_common: api::MatchCommon::from(match_),
        details,
    })
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
