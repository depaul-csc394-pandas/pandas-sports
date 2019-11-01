CREATE TABLE soccer (
    match_id
        INTEGER NOT NULL REFERENCES matches(id),
    team_id
        INTEGER NOT NULL REFERENCES teams(id),

    PRIMARY KEY (match_id, team_id),

    -- shots total / shots on goal
    shots
        INTEGER,
    sog
        INTEGER,

    -- possession percentage
    poss
        FLOAT4
        CONSTRAINT poss_between_0_and_1 CHECK (poss >= 0.0 AND poss <= 1.0),

    -- passes
    passes
        INTEGER,

    fouls
        INTEGER,

    -- yellow / red cards
    yellow
        INTEGER,
    red
        INTEGER,

    offsides
        INTEGER,
    corners
        INTEGER
);
