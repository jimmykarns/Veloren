CREATE TABLE IF NOT EXISTS "achievements" (
    "uuid" TEXT PRIMARY KEY NOT NULL,
    "checksum" TEXT NOT NULL UNIQUE,
    "title" TEXT NOT NULL,
    "action" INT NOT NULL,
    "target" INT NOT NULL
);