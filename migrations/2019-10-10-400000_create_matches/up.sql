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
        INTEGER NOT NULL,

    -- Exactly one of these sport-specific IDs must be set.
    basketball_id
        INTEGER REFERENCES basketball(id),
    baseball_id
        INTEGER REFERENCES baseball(id),
    football_id
        INTEGER REFERENCES football(id),
    hockey_id
        INTEGER REFERENCES hockey(id),
    volleyball_id
        INTEGER REFERENCES volleyball(id),
    soccer_id
        INTEGER REFERENCES soccer(id),

    CONSTRAINT one_sport_specific_id CHECK (
        (basketball_id IS NOT NULL)::INTEGER +
        (baseball_id   IS NOT NULL)::INTEGER +
        (football_id   IS NOT NULL)::INTEGER +
        (hockey_id     IS NOT NULL)::INTEGER +
        (volleyball_id IS NOT NULL)::INTEGER +
        (soccer_id     IS NOT NULL)::INTEGER = 1
    ),

    CONSTRAINT sport_id_matches_sport CHECK (
        (basketball_id IS NOT NULL AND sport = 'basketball') OR
        (baseball_id   IS NOT NULL AND sport = 'baseball') OR
        (football_id   IS NOT NULL AND sport = 'football') OR
        (hockey_id     IS NOT NULL AND sport = 'hockey') OR
        (volleyball_id IS NOT NULL AND sport = 'volleyball') OR
        (soccer_id     IS NOT NULL AND sport = 'soccer')
    )
);

CREATE FUNCTION delete_details() RETURNS trigger AS $delete_details$
    BEGIN
        IF OLD.baseball_id IS NOT NULL THEN
            DELETE FROM baseball WHERE id = OLD.baseball_id;
        ELSIF OLD.basketball_id IS NOT NULL THEN
            DELETE FROM basketball WHERE id = OLD.basketball_id;
        ELSIF OLD.football_id IS NOT NULL THEN
            DELETE FROM football WHERE id = OLD.football_id;
        ELSIF OLD.hockey_id IS NOT NULL THEN
            DELETE FROM hockey WHERE id = OLD.hockey_id;
        ELSIF OLD.soccer_id IS NOT NULL THEN
            DELETE FROM soccer WHERE id = OLD.soccer_id;
        ELSIF OLD.volleyball_id IS NOT NULL THEN
            DELETE FROM volleyball WHERE id = OLD.volleyball_id;
        ELSE
            RAISE EXCEPTION 'match % has no sport-specific ID set', OLD.id;
        END IF;
    END;
$delete_details$ LANGUAGE plpgsql;

CREATE TRIGGER before_delete_match_delete_details BEFORE DELETE ON matches
    FOR EACH ROW EXECUTE PROCEDURE delete_details();
