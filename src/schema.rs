table! {
    baseball (id) {
        id -> Int4,
        dummy -> Nullable<Int4>,
    }
}

table! {
    basketball (id) {
        id -> Int4,
        dummy -> Nullable<Int4>,
    }
}

table! {
    football (id) {
        id -> Int4,
        dummy -> Nullable<Int4>,
    }
}

table! {
    hockey (id) {
        id -> Int4,
        dummy -> Nullable<Int4>,
    }
}

table! {
    matches (id) {
        id -> Int4,
        start_time -> Nullable<Timestamptz>,
        duration_seconds -> Nullable<Int4>,
        location -> Nullable<Varchar>,
        sport -> Varchar,
        team_1_id -> Int4,
        team_2_id -> Int4,
        team_1_score -> Int4,
        team_2_score -> Int4,
        basketball_id -> Nullable<Int4>,
        baseball_id -> Nullable<Int4>,
        football_id -> Nullable<Int4>,
        hockey_id -> Nullable<Int4>,
        volleyball_id -> Nullable<Int4>,
        soccer_id -> Nullable<Int4>,
    }
}

table! {
    players (id) {
        id -> Int4,
        name_family -> Varchar,
        name_given -> Varchar,
        date_of_birth -> Date,
        height_cm -> Float4,
        weight_kg -> Float4,
    }
}

table! {
    rosters (roster_id) {
        roster_id -> Int4,
        player_id -> Int4,
        team_id -> Int4,
        date_range -> Daterange,
    }
}

table! {
    soccer (id) {
        id -> Int4,
        dummy -> Nullable<Int4>,
    }
}

table! {
    teams (id) {
        id -> Int4,
        team_name -> Varchar,
    }
}

table! {
    volleyball (id) {
        id -> Int4,
        dummy -> Nullable<Int4>,
    }
}

joinable!(matches -> baseball (baseball_id));
joinable!(matches -> basketball (basketball_id));
joinable!(matches -> football (football_id));
joinable!(matches -> hockey (hockey_id));
joinable!(matches -> soccer (soccer_id));
joinable!(matches -> volleyball (volleyball_id));
joinable!(rosters -> players (player_id));
joinable!(rosters -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    baseball,
    basketball,
    football,
    hockey,
    matches,
    players,
    rosters,
    soccer,
    teams,
    volleyball,
);
