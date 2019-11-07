use crate::{
    error::{self, ServiceError},
    models::sql,
};
use actix_identity::Identity;
use actix_web::web;
use diesel::prelude::*;
use failure::err_msg;

pub fn id_for_name<S>(pool: web::Data<crate::Pool>, username: S) -> Result<i32, ServiceError>
where
    S: AsRef<str>,
{
    let conn = pool.get().map_err(error::unavailable)?;

    let username = username.as_ref().to_string();

    use crate::schema::users::dsl;

    let user: sql::User = dsl::users
        .filter(dsl::username.eq(&username))
        .get_result(&conn)
        .map_err(error::unauthorized)?;

    Ok(user.id)
}

pub fn id_for_identity(
    pool: web::Data<crate::Pool>,
    identity: Identity,
) -> Result<i32, ServiceError> {
    let id_str = match identity.identity().map(|s| s.to_string()) {
        Some(s) => s,
        None => {
            return Err(error::unauthorized(err_msg(
                "You must be logged in to create matches.",
            )))
        }
    };

    id_for_name(pool, &id_str)
}

pub fn user_owns_team(
    pool: web::Data<crate::Pool>,
    user_id: i32,
    team_id: i32,
) -> Result<bool, ServiceError> {
    let conn = pool.get().map_err(error::unavailable)?;
    use crate::schema::teams::dsl;
    let team: sql::Team = dsl::teams
        .find(team_id)
        .get_result(&conn)
        .map_err(error::not_found)?;

    Ok(team.owner_id == user_id)
}

pub fn user_owns_match_(
    pool: web::Data<crate::Pool>,
    user_id: i32,
    match_id: i32,
) -> Result<bool, ServiceError> {
    let conn = pool.get().map_err(error::unavailable)?;
    use crate::schema::matches::dsl;
    let match_: sql::Match = dsl::matches
        .find(match_id)
        .get_result(&conn)
        .map_err(error::not_found)?;

    Ok(match_.owner_id == user_id)
}
