use crate::{
    error::{self, ServiceError},
    filter_query,
    models::{api, sql},
    order_query,
    resource::matches::get_match_details,
    Pool,
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
    pub order: Option<String>,
    pub filter: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    matches: Vec<api::GetMatch>,
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
    query: web::Query<QueryParams>,
    pool: web::Data<Pool>,
) -> Result<Response, ServiceError> {
    use crate::schema::matches::dsl;

    let limit = match query.limit {
        Some(l) if l <= MAX_LIMIT => l,
        _ => MAX_LIMIT,
    };

    let offset = match query.offset {
        Some(o) => o,
        _ => 0,
    };

    let orders = match query.order {
        Some(ref order) => match crate::resource::order::parse_orders(order) {
            Ok(o) => o,
            Err(e) => return Err(error::bad_request(failure::err_msg(e))),
        },

        None => Vec::new(),
    };

    let filters = match query.filter {
        Some(ref filter) => match crate::resource::filter::parse_filters(filter) {
            Ok(f) => f,
            Err(e) => return Err(error::bad_request(failure::err_msg(e))),
        },

        None => Vec::new(),
    };

    let mut query = dsl::matches
        .into_boxed()
        .limit(limit as i64)
        .offset(offset as i64);

    for order in orders {
        // TODO: weird type bounds in Diesel make it apparently impossible to remove this
        // boilerplate. Maybe there's a shorter way to write this?
        query = match order.field.as_str() {
            "id" => order_query!(query, dsl::id, order.ordering),
            "team_1_id" => order_query!(query, dsl::team_1_id, order.ordering),
            "team_1_score" => order_query!(query, dsl::team_1_score, order.ordering),
            "team_2_id" => order_query!(query, dsl::team_2_id, order.ordering),
            "team_2_score" => order_query!(query, dsl::team_2_score, order.ordering),
            other => {
                return Err(error::bad_request(failure::err_msg(format!(
                    "{} is not a valid column",
                    other
                ))))
            }
        };
    }

    for filter in filters {
        // TODO: don't unwrap the parse result of filter.val
        query = match filter.field.as_ref() {
            "id" => filter_query!(
                query,
                dsl::id,
                filter.op,
                filter.val.parse::<i32>().map_err(error::bad_request)?
            ),
            "team_1_id" => filter_query!(
                query,
                dsl::team_1_id,
                filter.op,
                filter.val.parse::<i32>().map_err(error::bad_request)?
            ),
            "team_1_score" => filter_query!(
                query,
                dsl::team_1_score,
                filter.op,
                filter.val.parse::<i32>().map_err(error::bad_request)?
            ),
            "team_2_id" => filter_query!(
                query,
                dsl::team_2_id,
                filter.op,
                filter.val.parse::<i32>().map_err(error::bad_request)?
            ),
            "team_2_score" => filter_query!(
                query,
                dsl::team_2_score,
                filter.op,
                filter.val.parse::<i32>().map_err(error::bad_request)?
            ),
            other => {
                return Err(error::bad_request(failure::err_msg(format!(
                    "{} is not a valid column",
                    other
                ))))
            }
        };
    }

    let conn = pool.get().map_err(error::unavailable)?;
    let matches = query
        .load::<sql::Match>(&conn)
        .map_err(error::from_diesel)?
        .into_iter()
        .map(|ref m| api::GetMatch {
            id: m.id,
            match_common: api::MatchCommon::from(m),
            details: get_match_details(pool.clone(), m).unwrap(),
        })
        .collect();

    Ok(Response { matches })
}

pub fn list_matches(
    query: web::Query<QueryParams>,
    pool: web::Data<Pool>,
) -> impl Future<Item = Response, Error = ServiceError> {
    web::block(move || list_query(query, pool)).then(move |res| match res {
        Ok(r) => Ok(r),
        Err(e) => Err(error::from_blocking(e)),
    })
}
