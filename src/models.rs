use crate::schema::{matches, teams};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Match {
    pub id: i32,
    pub team_1: i32,
    pub team_1_score: i32,
    pub team_2: i32,
    pub team_2_score: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub team_1: i32,
    pub team_1_score: i32,
    pub team_2: i32,
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
