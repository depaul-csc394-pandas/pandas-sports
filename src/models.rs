use crate::schema::{matches, players, teams};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Match {
    pub id: i32,
    pub team_1_id: i32,
    pub team_1_score: i32,
    pub team_2_id: i32,
    pub team_2_score: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub team_1_id: i32,
    pub team_1_score: i32,
    pub team_2_id: i32,
    pub team_2_score: i32,
}

#[derive(Queryable, Serialize)]
pub struct Team {
    pub id: i32,
    pub team_name: String,
}

#[derive(Deserialize, Insertable)]
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
