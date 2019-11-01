table! {
    baseball (match_id, team_id) {
        match_id -> Int4,
        team_id -> Int4,
        inning_runs -> Nullable<Array<Int4>>,
        hits -> Nullable<Int4>,
        errors -> Nullable<Int4>,
    }
}

table! {
    basketball (match_id, team_id) {
        match_id -> Int4,
        team_id -> Int4,
        q1 -> Nullable<Int4>,
        q2 -> Nullable<Int4>,
        q3 -> Nullable<Int4>,
        q4 -> Nullable<Int4>,
        fgm -> Nullable<Int4>,
        fga -> Nullable<Int4>,
        tpm -> Nullable<Int4>,
        tpa -> Nullable<Int4>,
        ftm -> Nullable<Int4>,
        fta -> Nullable<Int4>,
    }
}

table! {
    football (match_id, team_id) {
        match_id -> Int4,
        team_id -> Int4,
        q1 -> Nullable<Int4>,
        q2 -> Nullable<Int4>,
        q3 -> Nullable<Int4>,
        q4 -> Nullable<Int4>,
        td -> Nullable<Int4>,
        fg -> Nullable<Int4>,
        p_att -> Nullable<Int4>,
        p_comp -> Nullable<Int4>,
        yds_pass -> Nullable<Int4>,
        yds_rush -> Nullable<Int4>,
    }
}

table! {
    hockey (match_id, team_id) {
        match_id -> Int4,
        team_id -> Int4,
        sog -> Nullable<Int4>,
        sm -> Nullable<Int4>,
        fw -> Nullable<Int4>,
        fl -> Nullable<Int4>,
        sv -> Nullable<Int4>,
        sa -> Nullable<Int4>,
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
    soccer (match_id, team_id) {
        match_id -> Int4,
        team_id -> Int4,
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
    volleyball (match_id, team_id) {
        match_id -> Int4,
        team_id -> Int4,
        dummy -> Nullable<Int4>,
    }
}

joinable!(baseball -> matches (match_id));
joinable!(baseball -> teams (team_id));
joinable!(basketball -> matches (match_id));
joinable!(basketball -> teams (team_id));
joinable!(football -> matches (match_id));
joinable!(football -> teams (team_id));
joinable!(hockey -> matches (match_id));
joinable!(hockey -> teams (team_id));
joinable!(rosters -> players (player_id));
joinable!(rosters -> teams (team_id));
joinable!(soccer -> matches (match_id));
joinable!(soccer -> teams (team_id));
joinable!(volleyball -> matches (match_id));
joinable!(volleyball -> teams (team_id));

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
