-- Your SQL goes here
CREATE TABLE teams (
       id
           SERIAL PRIMARY KEY,
       owner_id
           INTEGER NOT NULL REFERENCES users(id)
           ON DELETE CASCADE,
       team_name
           VARCHAR(128) NOT NULL);
