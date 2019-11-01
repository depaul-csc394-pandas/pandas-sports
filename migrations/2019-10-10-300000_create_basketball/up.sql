CREATE TABLE basketball (
    match_id
        INTEGER NOT NULL REFERENCES matches(id),
    team_id
        INTEGER NOT NULL REFERENCES teams(id),

    PRIMARY KEY (match_id, team_id),

    -- per-quarter points scored
    q1
        INTEGER,
    q2
        INTEGER,
    q3
        INTEGER,
    q4
        INTEGER,

    -- shots made
    fgm
        INTEGER,
    -- shots attempted
    fga
        INTEGER,

    -- three-point shots made
    tpm
        INTEGER,
    -- three-point shots attempted
    tpa
        INTEGER,

    -- free-throws made
    ftm
        INTEGER,
    -- free-throws attempted
    fta
        INTEGER
);
