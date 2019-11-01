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
                    inning_runs: team_1.inning_runs.clone(),
                    hits: team_1.hits,
                    errors: team_1.errors,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::baseball)
                .values(sql::Baseball {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    inning_runs: team_2.inning_runs.clone(),
                    hits: team_2.hits,
                    errors: team_2.errors,
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
                    q1: team_1.q1,
                    q2: team_1.q2,
                    q3: team_1.q3,
                    q4: team_1.q4,
                    td: team_1.td,
                    fg: team_1.fg,
                    p_att: team_1.p_att,
                    p_comp: team_1.p_comp,
                    yds_pass: team_1.yds_pass,
                    yds_rush: team_1.yds_rush,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::football)
                .values(sql::Football {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    q1: team_2.q1,
                    q2: team_2.q2,
                    q3: team_2.q3,
                    q4: team_2.q4,
                    td: team_2.td,
                    fg: team_2.fg,
                    p_att: team_2.p_att,
                    p_comp: team_2.p_comp,
                    yds_pass: team_2.yds_pass,
                    yds_rush: team_2.yds_rush,
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
                    sog: team_1.sog,
                    sm: team_1.sm,
                    fw: team_1.fw,
                    fl: team_1.fl,
                    sv: team_1.sv,
                    sa: team_1.sa,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::hockey)
                .values(sql::Hockey {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    sog: team_2.sog,
                    sm: team_2.sm,
                    fw: team_2.fw,
                    fl: team_2.fl,
                    sv: team_2.sv,
                    sa: team_2.sa,
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
                    shots: team_1.shots,
                    sog: team_1.sog,
                    poss: team_1.poss,
                    passes: team_1.passes,
                    fouls: team_1.fouls,
                    yellow: team_1.yellow,
                    red: team_1.red,
                    offsides: team_1.offsides,
                    corners: team_1.corners,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::soccer)
                .values(sql::Soccer {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    shots: team_2.shots,
                    sog: team_2.sog,
                    poss: team_2.poss,
                    passes: team_2.passes,
                    fouls: team_2.fouls,
                    yellow: team_2.yellow,
                    red: team_2.red,
                    offsides: team_2.offsides,
                    corners: team_2.corners,
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
                    set_scores: team_1.set_scores.clone(),
                    sv_ace: team_1.sv_ace,
                    sv_err: team_1.sv_err,
                    sv_att: team_1.sv_att,
                    at_kill: team_1.at_kill,
                    at_err: team_1.at_err,
                    at_att: team_1.at_att,
                    rc_err: team_1.rc_err,
                    rc_att: team_1.rc_att,
                })
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let t2 = diesel::insert_into(dsl::volleyball)
                .values(sql::Volleyball {
                    match_id: match_.id,
                    team_id: match_.team_2_id,
                    set_scores: team_2.set_scores.clone(),
                    sv_ace: team_2.sv_ace,
                    sv_err: team_2.sv_err,
                    sv_att: team_2.sv_att,
                    at_kill: team_2.at_kill,
                    at_err: team_2.at_err,
                    at_att: team_2.at_att,
                    rc_err: team_2.rc_err,
                    rc_att: team_2.rc_att,
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
