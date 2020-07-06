CREATE TABLE IF NOT EXISTS "achievements" (
    "uuid" TEXT PRIMARY KEY NOT NULL,
    "title" TEXT NOT NULL,
    "action" TEXT NOT NULL,
    "target" INT NOT NULL
);