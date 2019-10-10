-- Your SQL goes here
CREATE TABLE rosters (
       roster_id  SERIAL PRIMARY KEY,
       player_id  INT NOT NULL REFERENCES players(id),
       team_id    INT NOT NULL REFERENCES teams(id),
       date_range DATERANGE NOT NULL,

       CONSTRAINT unique_player_date_range
       EXCLUDE USING gist (player_id WITH =, date_range WITH &&));
