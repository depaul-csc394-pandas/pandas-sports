CREATE TABLE hockey (
    match_id
        INTEGER NOT NULL REFERENCES matches(id),
    team_id
        INTEGER NOT NULL REFERENCES teams(id),

    PRIMARY KEY (match_id, team_id),

    -- shots on goal
    sog
        INTEGER,

    -- shots missed
    sm
        INTEGER,

    -- faceoffs won/lost
    fw
        INTEGER,
    fl
        INTEGER,

    -- successful saves
    sv
        INTEGER,

    -- saves attempted
    sa
        INTEGER,

    CONSTRAINT saves_leq_attempted CHECK (sv <= sa)
);
