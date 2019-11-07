use crate::{
    hash::SaltedHash,
    models::{api, Sport},
    schema::users,
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

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub privileged: bool,
    pub username: String,
    pub salt_base64: String,
    pub argon2_hash: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub privileged: bool,
    pub username: String,
    pub salt_base64: String,
    pub argon2_hash: String,
}

impl User {
    pub fn verify<S>(&self, password: S) -> bool
    where
        S: AsRef<str>,
    {
        let mut salt = [0u8; 32];
        (&mut salt[..])
            .write_all(&base64::decode(&self.salt_base64).expect("base64 decode"))
            .expect("write_all");

        SaltedHash {
            salt,
            hash: self.argon2_hash.clone().into_bytes(),
        }
        .verify(password.as_ref())
    }
}

#[derive(Queryable, Serialize)]
pub struct Match {
    pub id: i32,
    pub owner_id: i32,
    pub start_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub location: Option<String>,
    pub sport: Sport,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub team_1_score: i32,
    pub team_2_score: i32,
}

#[derive(Debug, PartialEq, Insertable, Queryable, Deserialize, Serialize, Clone)]
#[table_name = "baseball"]
pub struct Baseball {
    pub match_id: i32,
    pub team_id: i32,
    pub inning_runs: Option<Vec<i32>>,
    pub hits: Option<i32>,
    pub errors: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Queryable, Deserialize, Serialize, Clone)]
#[table_name = "basketball"]
pub struct Basketball {
    pub match_id: i32,
    pub team_id: i32,
    pub q1: Option<i32>,
    pub q2: Option<i32>,
    pub q3: Option<i32>,
    pub q4: Option<i32>,
    pub fgm: Option<i32>,
    pub fga: Option<i32>,
    pub tpm: Option<i32>,
    pub tpa: Option<i32>,
    pub ftm: Option<i32>,
    pub fta: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Queryable, Deserialize, Serialize, Clone)]
#[table_name = "football"]
pub struct Football {
    pub match_id: i32,
    pub team_id: i32,
    pub q1: Option<i32>,
    pub q2: Option<i32>,
    pub q3: Option<i32>,
    pub q4: Option<i32>,
    pub td: Option<i32>,
    pub fg: Option<i32>,
    pub p_att: Option<i32>,
    pub p_comp: Option<i32>,
    pub yds_pass: Option<i32>,
    pub yds_rush: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Queryable, Deserialize, Serialize, Clone)]
#[table_name = "hockey"]
pub struct Hockey {
    pub match_id: i32,
    pub team_id: i32,
    pub sog: Option<i32>,
    pub sm: Option<i32>,
    pub fw: Option<i32>,
    pub fl: Option<i32>,
    pub sv: Option<i32>,
    pub sa: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Queryable, Deserialize, Serialize, Clone)]
#[table_name = "soccer"]
pub struct Soccer {
    pub match_id: i32,
    pub team_id: i32,
    pub shots: Option<i32>,
    pub sog: Option<i32>,
    pub poss: Option<f32>,
    pub passes: Option<i32>,
    pub fouls: Option<i32>,
    pub yellow: Option<i32>,
    pub red: Option<i32>,
    pub offsides: Option<i32>,
    pub corners: Option<i32>,
}

#[derive(Debug, PartialEq, Insertable, Queryable, Deserialize, Serialize, Clone)]
#[table_name = "volleyball"]
pub struct Volleyball {
    pub match_id: i32,
    pub team_id: i32,
    pub set_scores: Option<Vec<i32>>,
    pub sv_ace: Option<i32>,
    pub sv_err: Option<i32>,
    pub sv_att: Option<i32>,
    pub at_kill: Option<i32>,
    pub at_err: Option<i32>,
    pub at_att: Option<i32>,
    pub rc_err: Option<i32>,
    pub rc_att: Option<i32>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub owner_id: i32,
    pub start_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub location: Option<String>,
    pub sport: Sport,
    pub team_1_id: i32,
    pub team_1_score: i32,
    pub team_2_id: i32,
    pub team_2_score: i32,
}

impl NewMatch {
    pub fn with_owner(from: api::PostMatch, owner_id: i32) -> Self {
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
            owner_id,
            start_time,
            duration_seconds,
            location,
            sport,
            team_1_id,
            team_2_id,
            team_1_score,
            team_2_score,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Queryable, Serialize)]
pub struct Team {
    pub id: i32,
    pub owner_id: i32,
    pub team_name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Insertable)]
#[table_name = "teams"]
pub struct NewTeam {
    pub owner_id: i32,
    pub team_name: String,
}

impl NewTeam {
    pub fn with_owner(team: api::PostTeam, owner_id: i32) -> Self {
        NewTeam {
            owner_id,
            team_name: team.team_name.clone(),
        }
    }
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
