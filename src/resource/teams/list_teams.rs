use crate::{
    error::{self, ServiceError},
    models::{api, sql},
    order_query, Pool,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

const MAX_LIMIT: i32 = 100;

#[derive(Deserialize)]
pub struct QueryParams {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub q: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    teams: Vec<sql::Team>,
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

fn list_query(
    query_params: web::Query<QueryParams>,
    pool: web::Data<Pool>,
) -> Result<Response, ServiceError> {
    use crate::schema::teams::dsl;

    let limit = match query_params.limit {
        Some(l) if l <= MAX_LIMIT => l,
        _ => MAX_LIMIT,
    };

    let offset = match query_params.offset {
        Some(o) => o,
        _ => 0,
    };

    let mut query = dsl::teams
        .into_boxed()
        .limit(limit as i64)
        .offset(offset as i64);

    if let Some(ref q) = query_params.q {
        query = query.filter(dsl::team_name.ilike(format!("{}%", q)));
    }

    let conn = pool.get().map_err(error::unavailable)?;
    let teams = query.load::<sql::Team>(&conn).map_err(error::from_diesel)?;

    Ok(Response { teams })
}

pub fn list_teams(
    query: web::Query<QueryParams>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || list_query(query, pool)).then(move |res| match res {
        Ok(r) => Ok(r),
        Err(e) => Err(error::from_blocking(e)),
    })
}
