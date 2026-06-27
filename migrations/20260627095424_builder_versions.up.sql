-- Add up migration script here
CREATE TABLE builder_versions (
    language TEXT NOT NULL,
    version TEXT NOT NULL,

    PRIMARY KEY (language, version)
);
