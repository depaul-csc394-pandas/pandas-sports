use crate::models::{self, sql};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MatchCommon {
    pub start_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub location: Option<String>,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub team_1_score: i32,
    pub team_2_score: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "sport")]
pub enum GetMatchDetails {
    Baseball { details_id: i32 },
    Basketball { details_id: i32 },
    Football { details_id: i32 },
    Hockey { details_id: i32 },
    Soccer { details_id: i32 },
    Volleyball { details_id: i32 },
}

#[derive(Clone, Serialize)]
pub struct GetMatch {
    pub id: i32,
    #[serde(flatten)]
    pub match_common: MatchCommon,
    pub details: GetMatchDetails,
}

impl From<sql::Match> for GetMatch {
    fn from(m: sql::Match) -> Self {
        let details = match m.sport {
            models::Sport::Baseball => GetMatchDetails::Baseball {
                details_id: m.baseball_id.unwrap(),
            },
            models::Sport::Basketball => GetMatchDetails::Basketball {
                details_id: m.basketball_id.unwrap(),
            },
            models::Sport::Football => GetMatchDetails::Football {
                details_id: m.football_id.unwrap(),
            },
            models::Sport::Hockey => GetMatchDetails::Hockey {
                details_id: m.hockey_id.unwrap(),
            },
            models::Sport::Soccer => GetMatchDetails::Soccer {
                details_id: m.soccer_id.unwrap(),
            },
            models::Sport::Volleyball => GetMatchDetails::Volleyball {
                details_id: m.volleyball_id.unwrap(),
            },
        };

        GetMatch {
            id: m.id,
            match_common: MatchCommon {
                start_time: m.start_time.clone(),
                duration_seconds: m.duration_seconds.clone(),
                location: m.location.clone(),
                team_1_id: m.team_1_id,
                team_2_id: m.team_2_id,
                team_1_score: m.team_1_score,
                team_2_score: m.team_2_score,
            },
            details,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case", tag = "sport")]
pub enum PostMatchDetails {
    Baseball {
        #[serde(flatten)]
        details: sql::NewBaseball,
    },
    Basketball {
        #[serde(flatten)]
        details: sql::NewBasketball,
    },
    Football {
        #[serde(flatten)]
        details: sql::NewFootball,
    },
    Hockey {
        #[serde(flatten)]
        details: sql::NewHockey,
    },
    Soccer {
        #[serde(flatten)]
        details: sql::NewSoccer,
    },
    Volleyball {
        #[serde(flatten)]
        details: sql::NewVolleyball,
    },
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct PostMatch {
    #[serde(flatten)]
    pub match_common: MatchCommon,
    pub details: PostMatchDetails,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use serde_json::json;

    #[test]
    fn test_match_common_deserialize() {
        let src = json!({
            "start_time": "2019-07-09T12:00:00Z",
            "team_1_id": 1,
            "team_2_id": 2,
            "team_1_score": 3,
            "team_2_score": 0,
        });

        assert_eq!(
            MatchCommon::deserialize(src).unwrap(),
            MatchCommon {
                start_time: Some(Utc.ymd(2019, 07, 09).and_hms(12, 00, 00)),
                duration_seconds: None,
                location: None,
                team_1_id: 1,
                team_2_id: 2,
                team_1_score: 3,
                team_2_score: 0,
            }
        );
    }

    #[test]
    fn test_get_match_details_serialize() {
        let src = GetMatchDetails::Volleyball { details_id: 42 };

        assert_eq!(
            serde_json::to_value(src).expect("serialization failed"),
            json!({
                "sport": "volleyball",
                "details_id": 42,
            })
        );
    }

    #[test]
    fn test_get_match_serialize() {
        let src = GetMatch {
            id: 17,
            match_common: MatchCommon {
                start_time: Some(Utc.ymd(1917, 11, 08).and_hms(00, 45, 00)),
                duration_seconds: Some(15900),
                location: Some("Petrograd, Russia".to_string()),
                team_1_id: 3,
                team_2_id: 2,
                team_1_score: 21,
                team_2_score: 7,
            },
            details: GetMatchDetails::Football { details_id: 42 },
        };

        assert_eq!(
            serde_json::to_value(src).expect("serialization failed"),
            json!({
                "id": 17,
                "start_time": "1917-11-08T00:45:00Z",
                "duration_seconds": 15900,
                "location": "Petrograd, Russia",
                "team_1_id": 3,
                "team_2_id": 2,
                "team_1_score": 21,
                "team_2_score": 7,
                "details": {
                    "sport": "football",
                    "details_id": 42,
                }
            })
        );
    }

    #[test]
    fn test_post_match_details_deserialize() {
        let src = json!({
            "sport": "hockey",
            "dummy": 1,
        });

        assert_eq!(
            PostMatchDetails::deserialize(src).expect("deserialization failed"),
            PostMatchDetails::Hockey {
                details: models::sql::NewHockey { dummy: Some(1) }
            }
        );
    }

    #[test]
    fn test_post_match_deserialize() {
        let src = json!({
            "start_time": "1996-07-09T03:00:00Z",
            "duration_seconds": 725300000,
            "location": "Chicago, Illinois, United States",
            "team_1_id": 42,
            "team_2_id": 18,
            "team_1_score": 9001,
            "team_2_score": 26,
            "details": {
                "sport": "baseball",
                "dummy": 99,
            },
        });

        assert_eq!(
            PostMatch::deserialize(src).expect("deserialization failed"),
            PostMatch {
                match_common: MatchCommon {
                    start_time: Some(Utc.ymd(1996, 07, 09).and_hms(03, 00, 00)),
                    duration_seconds: Some(725300000),
                    location: Some("Chicago, Illinois, United States".to_string()),
                    team_1_id: 42,
                    team_2_id: 18,
                    team_1_score: 9001,
                    team_2_score: 26,
                },
                details: PostMatchDetails::Baseball {
                    details: models::sql::NewBaseball { dummy: Some(99) }
                }
            }
        );
    }
}
