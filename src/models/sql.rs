use crate::{
    models::{api, Sport},
    schema::*,
};
use chrono::{DateTime, NaiveDate, Utc};
use diesel::{
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};
use std::{convert::From, io::Write};

impl<Db> ToSql<Text, Db> for crate::models::Sport
where
    Db: diesel::backend::Backend,
{
    fn to_sql<W>(&self, out: &mut Output<W, Db>) -> diesel::serialize::Result
    where
        W: Write,
    {
        match *self {
            Sport::Baseball => out.write_all(b"baseball")?,
            Sport::Basketball => out.write_all(b"basketball")?,
            Sport::Football => out.write_all(b"football")?,
            Sport::Hockey => out.write_all(b"hockey")?,
            Sport::Soccer => out.write_all(b"soccer")?,
            Sport::Volleyball => out.write_all(b"volleyball")?,
        }

        Ok(diesel::serialize::IsNull::No)
    }
}

impl<Db> FromSql<Text, Db> for crate::models::Sport
where
    Db: diesel::backend::Backend,
    String: FromSql<Text, Db>,
{
    fn from_sql(
        bytes: Option<&<Db as diesel::backend::Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        let sport = match String::from_sql(bytes)?.as_str() {
            "baseball" => Sport::Baseball,
            "basketball" => Sport::Basketball,
            "football" => Sport::Football,
            "hockey" => Sport::Hockey,
            "soccer" => Sport::Soccer,
            "volleyball" => Sport::Volleyball,
            s => {
                return Err(format!("Invalid data in database: {} is not a valid Sport", s).into())
            }
        };

        Ok(sport)
    }
}

#[derive(Queryable, Serialize)]
pub struct Match {
    pub id: i32,
    pub start_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub location: Option<String>,
    pub sport: Sport,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub team_1_score: i32,
    pub team_2_score: i32,
    pub basketball_id: Option<i32>,
    pub baseball_id: Option<i32>,
    pub football_id: Option<i32>,
    pub hockey_id: Option<i32>,
    pub volleyball_id: Option<i32>,
    pub soccer_id: Option<i32>,
}

#[derive(Debug, PartialEq, Queryable, Deserialize, Serialize, Clone)]
pub struct Baseball {
    pub id: i32,
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Deserialize, Clone)]
#[table_name = "baseball"]
pub struct NewBaseball {
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Queryable, Deserialize, Serialize, Clone)]
pub struct Basketball {
    pub id: i32,
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Deserialize, Clone)]
#[table_name = "basketball"]
pub struct NewBasketball {
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Queryable, Deserialize, Serialize, Clone)]
pub struct Football {
    pub id: i32,
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Deserialize, Clone)]
#[table_name = "football"]
pub struct NewFootball {
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Queryable, Deserialize, Serialize, Clone)]
pub struct Hockey {
    pub id: i32,
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Deserialize, Clone)]
#[table_name = "hockey"]
pub struct NewHockey {
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Queryable, Deserialize, Serialize, Clone)]
pub struct Soccer {
    pub id: i32,
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Deserialize, Clone)]
#[table_name = "soccer"]
pub struct NewSoccer {
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Queryable, Deserialize, Serialize, Clone)]
pub struct Volleyball {
    pub id: i32,
    pub dummy: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Deserialize, Clone)]
#[table_name = "volleyball"]
pub struct NewVolleyball {
    pub dummy: Option<i32>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub start_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub location: Option<String>,
    pub sport: Sport,
    pub team_1_id: i32,
    pub team_1_score: i32,
    pub team_2_id: i32,
    pub team_2_score: i32,
    pub basketball_id: Option<i32>,
    pub baseball_id: Option<i32>,
    pub football_id: Option<i32>,
    pub hockey_id: Option<i32>,
    pub volleyball_id: Option<i32>,
    pub soccer_id: Option<i32>,
}

impl From<api::PostMatch> for NewMatch {
    fn from(from: api::PostMatch) -> Self {
        let api::MatchCommon {
            start_time,
            duration_seconds,
            location,
            team_1_id,
            team_2_id,
            team_1_score,
            team_2_score,
        } = from.match_common;

        let sport = match from.details {
            api::PostMatchDetails::Baseball { .. } => Sport::Baseball,
            api::PostMatchDetails::Basketball { .. } => Sport::Basketball,
            api::PostMatchDetails::Football { .. } => Sport::Football,
            api::PostMatchDetails::Hockey { .. } => Sport::Hockey,
            api::PostMatchDetails::Soccer { .. } => Sport::Soccer,
            api::PostMatchDetails::Volleyball { .. } => Sport::Volleyball,
        };

        NewMatch {
            start_time,
            duration_seconds,
            location,
            sport,
            team_1_id,
            team_2_id,
            team_1_score,
            team_2_score,
            basketball_id: None,
            baseball_id: None,
            football_id: None,
            hockey_id: None,
            volleyball_id: None,
            soccer_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Queryable, Serialize)]
pub struct Team {
    pub id: i32,
    pub team_name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "teams"]
pub struct NewTeam {
    pub team_name: String,
}

#[derive(Queryable, Serialize)]
pub struct Player {
    pub id: i32,
    pub name_family: String,
    pub name_given: String,
    pub date_of_birth: NaiveDate,
    pub height_cm: f32,
    pub weight_kg: f32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub name_family: String,
    pub name_given: String,
    pub date_of_birth: NaiveDate,
    pub height_cm: f32,
    pub weight_kg: f32,
}
