-- Add migration script here
CREATE TABLE discordUsers (
       id INTEGER PRIMARY KEY,
       requested_server_id INTEGER NOT NULL,
       watching_prs TEXT NOT NULL
)
