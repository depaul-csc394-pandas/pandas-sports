CREATE TABLE matches (
    id
        SERIAL PRIMARY KEY,
    start_time
        TIMESTAMPTZ,
    duration_seconds
        INTEGER,
    location
        VARCHAR(128),
    sport
        VARCHAR(32) NOT NULL
        CHECK (
            sport = 'baseball' OR
            sport = 'basketball' OR
            sport = 'football' OR
            sport = 'hockey' OR
            sport = 'soccer' OR
            sport = 'volleyball'
        ),
    team_1_id
        INTEGER NOT NULL,
    team_2_id
        INTEGER NOT NULL,
    team_1_score
        INTEGER NOT NULL,
    team_2_score
        INTEGER NOT NULL
);

