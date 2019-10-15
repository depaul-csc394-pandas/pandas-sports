table! {
    matches (id) {
        id -> Int4,
        team_1_id -> Int4,
        team_1_score -> Int4,
        team_2_id -> Int4,
        team_2_score -> Int4,
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
    teams (id) {
        id -> Int4,
        team_name -> Varchar,
    }
}

joinable!(rosters -> players (player_id));
joinable!(rosters -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    matches,
    players,
    rosters,
    teams,
);
