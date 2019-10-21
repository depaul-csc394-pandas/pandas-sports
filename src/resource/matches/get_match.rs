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
pub struct PathParams {
    id: i32,
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

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn query(path: web::Path<PathParams>, pool: web::Data<Pool>) -> Result<Response, ServiceError> {
    use crate::schema::matches::dsl::*;
    let conn = pool.get().map_err(error::unavailable)?;

    let match_: sql::Match = matches
        .find(path.id)
        .get_result(&conn)
        .map_err(error::from_diesel)?;

    Ok(Response {
        match_: match_.into(),
    })
}

pub fn get_match(
    path: web::Path<PathParams>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || query(path, pool)).then(move |res| match res {
        Ok(r) => Ok(r),
        Err(e) => Err(error::from_blocking(e)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{offset::TimeZone, Utc};
    use serde_json::json;

    #[test]
    fn test_serialize_response() {
        let src = Response {
            match_: api::GetMatch {
                id: 1,
                match_common: api::MatchCommon {
                    start_time: Some(Utc.ymd(2019, 07, 09).and_hms(12, 34, 56)),
                    duration_seconds: Some(99),
                    location: Some("DePaul University".to_string()),
                    team_1_id: 1,
                    team_2_id: 2,
                    team_1_score: 21,
                    team_2_score: 17,
                },
                details: api::GetMatchDetails::Football { details_id: 24 },
            },
        };

        assert_eq!(
            serde_json::to_value(src).expect("serialization failed"),
            json!({
                "match": {
                    "id": 1,
                    "start_time": "2019-07-09T12:34:56Z",
                    "duration_seconds": 99,
                    "location": "DePaul University",
                    "team_1_id": 1,
                    "team_2_id": 2,
                    "team_1_score": 21,
                    "team_2_score": 17,
                    "details": {
                        "sport": "football",
                        "details_id": 24,
                    }
                }
            })
        );
    }
}
