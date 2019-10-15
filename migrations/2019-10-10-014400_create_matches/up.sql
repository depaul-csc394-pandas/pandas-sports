CREATE TABLE matches (
       id           SERIAL    PRIMARY KEY,
       -- start_time   TIMESTAMP,
       -- duration     INTERVAL,
       -- location     VARCHAR(128),
       team_1_id    INTEGER   REFERENCES teams(id) NOT NULL,
       team_1_score INTEGER                        NOT NULL,
       team_2_id    INTEGER   REFERENCES teams(id) NOT NULL,
       team_2_score INTEGER                        NOT NULL);
