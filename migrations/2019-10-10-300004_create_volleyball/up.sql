CREATE TABLE volleyball (
    match_id
        INTEGER NOT NULL REFERENCES matches(id),
    team_id
        INTEGER NOT NULL REFERENCES teams(id),

    PRIMARY KEY (match_id, team_id),

    set_scores
        INTEGER[]
        CONSTRAINT set_scores_length_3_to_5 CHECK (array_length(set_scores, 1) >= 3 AND array_length(set_scores, 1) <= 5),
    sv_ace
        INTEGER,
    sv_err
        INTEGER,
    sv_att
        INTEGER,
    at_kill
        INTEGER,
    at_err
        INTEGER,
    at_att
        INTEGER,
    rc_err
        INTEGER,
    rc_att
        INTEGER
);
