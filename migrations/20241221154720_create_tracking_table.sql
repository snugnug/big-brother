-- Add migration script here
CREATE TABLE pullRequests (
       id INTEGER PRIMARY KEY,
       merged INTEGER NOT NULL,
       merged_into TEXT NOT NULL,
       unmerged_into TEXT NOT NULL
)
