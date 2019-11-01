CREATE TABLE baseball (
    match_id
        INTEGER NOT NULL REFERENCES matches(id),
    team_id
        INTEGER NOT NULL REFERENCES teams(id),

    PRIMARY KEY (match_id, team_id),

    inning_runs
        INTEGER[]
        CHECK (array_length(inning_runs, 1) >= 9),

    hits
        INTEGER,
    errors
        INTEGER
);
