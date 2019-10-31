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

impl<M> From<M> for MatchCommon
where
    M: std::borrow::Borrow<sql::Match>,
{
    fn from(match_: M) -> Self {
        let m = match_.borrow();

        MatchCommon {
            start_time: m.start_time,
            duration_seconds: m.duration_seconds,
            location: m.location.clone(),
            team_1_id: m.team_1_id,
            team_2_id: m.team_2_id,
            team_1_score: m.team_1_score,
            team_2_score: m.team_2_score,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "sport")]
pub enum GetMatchDetails {
    Baseball {
        team_1: sql::Baseball,
        team_2: sql::Baseball,
    },
    Basketball {
        team_1: sql::Basketball,
        team_2: sql::Basketball,
    },
    Football {
        team_1: sql::Football,
        team_2: sql::Football,
    },
    Hockey {
        team_1: sql::Hockey,
        team_2: sql::Hockey,
    },
    Soccer {
        team_1: sql::Soccer,
        team_2: sql::Soccer,
    },
    Volleyball {
        team_1: sql::Volleyball,
        team_2: sql::Volleyball,
    },
}

#[derive(Clone, Serialize)]
pub struct GetMatch {
    pub id: i32,
    #[serde(flatten)]
    pub match_common: MatchCommon,
    pub details: GetMatchDetails,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PostMatchDetailsBaseball {
    pub inning_runs: Option<Vec<i32>>,
    pub hits: Option<i32>,
    pub errors: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PostMatchDetailsBasketball {
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

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PostMatchDetailsFootball {
    pub dummy: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PostMatchDetailsHockey {
    pub dummy: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PostMatchDetailsSoccer {
    pub dummy: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PostMatchDetailsVolleyball {
    pub dummy: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case", tag = "sport")]
pub enum PostMatchDetails {
    Baseball {
        team_1: PostMatchDetailsBaseball,
        team_2: PostMatchDetailsBaseball,
    },
    Basketball {
        team_1: PostMatchDetailsBasketball,
        team_2: PostMatchDetailsBasketball,
    },
    Football {
        team_1: PostMatchDetailsFootball,
        team_2: PostMatchDetailsFootball,
    },
    Hockey {
        team_1: PostMatchDetailsHockey,
        team_2: PostMatchDetailsHockey,
    },
    Soccer {
        team_1: PostMatchDetailsSoccer,
        team_2: PostMatchDetailsSoccer,
    },
    Volleyball {
        team_1: PostMatchDetailsVolleyball,
        team_2: PostMatchDetailsVolleyball,
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
        let src = GetMatchDetails::Volleyball {
            team_1: sql::Volleyball {
                match_id: 1,
                team_id: 5,
                dummy: Some(7),
            },
            team_2: sql::Volleyball {
                match_id: 1,
                team_id: 7,
                dummy: Some(12),
            },
        };

        assert_eq!(
            serde_json::to_value(src).expect("serialization failed"),
            json!({
                "sport": "volleyball",
                "team_1": {
                    "match_id": 1,
                    "team_id": 5,
                    "dummy": 7,
                },
                "team_2": {
                    "match_id": 1,
                    "team_id": 7,
                    "dummy": 12,
                },
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
            details: GetMatchDetails::Football {
                team_1: sql::Football {
                    match_id: 42,
                    team_id: 6,
                    dummy: Some(21),
                },
                team_2: sql::Football {
                    match_id: 42,
                    team_id: 17,
                    dummy: Some(40),
                },
            },
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
                    "team_1": {
                        "match_id": 42,
                        "team_id": 6,
                        "dummy": 21,
                    },
                    "team_2": {
                        "match_id": 42,
                        "team_id": 17,
                        "dummy": 40,
                    },
                }
            })
        );
    }

    #[test]
    fn test_post_match_details_deserialize() {
        let src = json!({
            "sport": "hockey",
            "team_1": {
                "match_id": 57,
                "team_id": 24,
                "dummy": 7,
            },
            "team_2": {
                "match_id": 57,
                "team_id": 72,
                "dummy": 2,
            },
        });

        assert_eq!(
            PostMatchDetails::deserialize(src).expect("deserialization failed"),
            PostMatchDetails::Hockey {
                team_1: PostMatchDetailsHockey { dummy: Some(7) },
                team_2: PostMatchDetailsHockey { dummy: Some(2) },
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
                "sport": "basketball",
                "team_1": {
                    "q1": 21,
                    "q2": 17,
                    "q3": 30,
                    "q4": 19,
                    "fgm": 17,
                    "fga": 44,
                    "tpm": 6,
                    "tpa": 14,
                    "ftm": 7,
                    "fta": 9
                },
                "team_2": {
                    "q1": 27,
                    "q2": 14,
                    "q3": 19,
                    "q4": 29,
                    "fgm": 13,
                    "fga": 39,
                    "tpm": 7,
                    "tpa": 12,
                    "ftm": 6,
                    "fta": 6
                }
            }
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
                details: PostMatchDetails::Basketball {
                    team_1: PostMatchDetailsBasketball {
                        q1: Some(21),
                        q2: Some(17),
                        q3: Some(30),
                        q4: Some(19),
                        fgm: Some(17),
                        fga: Some(44),
                        tpm: Some(6),
                        tpa: Some(14),
                        ftm: Some(7),
                        fta: Some(9),
                    },
                    team_2: PostMatchDetailsBasketball {
                        q1: Some(27),
                        q2: Some(14),
                        q3: Some(19),
                        q4: Some(29),
                        fgm: Some(13),
                        fga: Some(39),
                        tpm: Some(7),
                        tpa: Some(12),
                        ftm: Some(6),
                        fta: Some(6),
                    }
                }
            }
        );
    }
}
