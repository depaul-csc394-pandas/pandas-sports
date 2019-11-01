mod create_match;
mod delete_match;
mod get_match;
mod list_matches;

pub use create_match::create_match;
pub use delete_match::delete_match;
pub use get_match::get_match;
pub use list_matches::list_matches;

use crate::{
    error::{self, ServiceError},
    models::{api, sql, Sport},
    Pool,
};
use actix_web::web;
use diesel::prelude::*;

pub fn get_match_details(
    pool: web::Data<Pool>,
    match_: &crate::models::sql::Match,
) -> Result<api::GetMatchDetails, ServiceError> {
    let conn = pool.get().map_err(error::unavailable)?;

    let t1_key = (match_.id, match_.team_1_id);
    let t2_key = (match_.id, match_.team_2_id);

    let details = match match_.sport {
        Sport::Baseball => {
            use crate::schema::baseball::dsl::*;
            let team_1 = baseball
                .find(t1_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let team_2 = baseball
                .find((match_.id, match_.team_2_id))
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Baseball { team_1, team_2 }
        }
        Sport::Basketball => {
            use crate::schema::basketball::dsl::*;
            let team_1 = basketball
                .find(t1_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let team_2 = basketball
                .find(t2_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Basketball { team_1, team_2 }
        }
        Sport::Football => {
            use crate::schema::football::dsl::*;
            let team_1 = football
                .find(t1_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let team_2 = football
                .find(t2_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Football { team_1, team_2 }
        }
        Sport::Hockey => {
            use crate::schema::hockey::dsl::*;
            let team_1 = hockey
                .find(t1_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let team_2 = hockey
                .find(t2_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Hockey { team_1, team_2 }
        }
        Sport::Soccer => {
            use crate::schema::soccer::dsl::*;
            let team_1 = soccer
                .find(t1_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let team_2 = soccer
                .find(t2_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Soccer { team_1, team_2 }
        }
        Sport::Volleyball => {
            use crate::schema::volleyball::dsl::*;
            let team_1 = volleyball
                .find(t1_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            let team_2 = volleyball
                .find(t2_key)
                .get_result(&conn)
                .map_err(error::from_diesel)?;
            api::GetMatchDetails::Volleyball { team_1, team_2 }
        }
    };

    Ok(details)
}
