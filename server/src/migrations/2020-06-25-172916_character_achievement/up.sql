CREATE TABLE IF NOT EXISTS "character_achievement" (
    character_id INTEGER PRIMARY KEY NOT NULL,
    achievement_uuid TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    progress INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(character_id) REFERENCES "character"(id) ON DELETE CASCADE,
    FOREIGN KEY(achievement_uuid) REFERENCES "achievement"(uuid) ON DELETE CASCADE
);