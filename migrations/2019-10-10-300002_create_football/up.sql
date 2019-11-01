CREATE TABLE football (
    match_id
        INTEGER NOT NULL REFERENCES matches(id),
    team_id
        INTEGER NOT NULL REFERENCES teams(id),

    PRIMARY KEY (match_id, team_id),

    -- per-quarter scores
    q1
        INTEGER,
    q2
        INTEGER,
    q3
        INTEGER,
    q4
        INTEGER,
    -- touchdowns
    td
        INTEGER,
    -- field goals
    fg
        INTEGER,
    -- passes attempted
    p_att
        INTEGER,
    -- passes completed
    p_comp
        INTEGER,
    -- yards passing
    yds_pass
        INTEGER,
    -- yards rushing
    yds_rush
        INTEGER
);
