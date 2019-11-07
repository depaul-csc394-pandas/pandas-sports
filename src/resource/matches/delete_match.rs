use crate::{
    error::{self, ServiceError},
    models::{api, sql},
    user, Pool,
};
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathParams {
    id: i32,
}

fn query(
    path: web::Path<PathParams>,
    pool: web::Data<Pool>,
    user_id: i32,
) -> Result<(), ServiceError> {
    use crate::schema::matches::dsl::*;

    if !user::user_owns_match_(pool.clone(), user_id, path.id)? {
        return Err(error::forbidden(failure::err_msg(
            "You do not own this match.",
        )));
    }

    let conn = pool.get().map_err(error::unavailable)?;

    let match_: sql::Match = {
        use crate::schema::matches::dsl::*;
        matches
            .find(path.id)
            .get_result(&conn)
            .map_err(error::from_diesel)?
    };

    let details = crate::resource::matches::get_match_details(pool, &match_)?;

    let t1_key = (match_.id, match_.team_1_id);
    let t2_key = (match_.id, match_.team_2_id);

    match details {
        api::GetMatchDetails::Baseball { .. } => {
            use crate::schema::baseball::dsl::*;
            diesel::delete(baseball.find(t1_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
            diesel::delete(baseball.find(t2_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
        }
        api::GetMatchDetails::Basketball { .. } => {
            use crate::schema::basketball::dsl::*;
            diesel::delete(basketball.find(t1_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
            diesel::delete(basketball.find(t2_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
        }
        api::GetMatchDetails::Football { .. } => {
            use crate::schema::football::dsl::*;
            diesel::delete(football.find(t1_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
            diesel::delete(football.find(t2_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
        }
        api::GetMatchDetails::Hockey { .. } => {
            use crate::schema::hockey::dsl::*;
            diesel::delete(hockey.find(t1_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
            diesel::delete(hockey.find(t2_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
        }
        api::GetMatchDetails::Soccer { .. } => {
            use crate::schema::soccer::dsl::*;
            diesel::delete(soccer.find(t1_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
            diesel::delete(soccer.find(t2_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
        }
        api::GetMatchDetails::Volleyball { .. } => {
            use crate::schema::volleyball::dsl::*;
            diesel::delete(volleyball.find(t1_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
            diesel::delete(volleyball.find(t2_key))
                .execute(&conn)
                .map_err(error::from_diesel)?;
        }
    }

    diesel::delete(matches.find(&path.id))
        .execute(&conn)
        .map_err(error::from_diesel)?;

    Ok(())
}

pub fn delete_match(
    path: web::Path<PathParams>,
    pool: web::Data<Pool>,
    identity: Identity,
) -> Result<HttpResponse, ServiceError> {
    let user_id = crate::user::id_for_identity(pool.clone(), identity.clone())?;

    match query(path, pool, user_id) {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Err(e),
    }
}
