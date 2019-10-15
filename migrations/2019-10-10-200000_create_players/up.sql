-- Your SQL goes here
CREATE TABLE players (
       id            SERIAL       PRIMARY KEY,
       name_family   VARCHAR(32)  NOT NULL,
       name_given    VARCHAR(32)  NOT NULL,
       date_of_birth DATE         NOT NULL,
       height_cm     REAL         NOT NULL,
       weight_kg     REAL         NOT NULL);
