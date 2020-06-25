CREATE TABLE IF NOT EXISTS "character_achievement" (
    character_id INTEGER PRIMARY KEY NOT NULL,
    achievement_id INTEGER NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    progress INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(character_id) REFERENCES "character"(id) ON DELETE CASCADE,
    FOREIGN KEY(achievement_id) REFERENCES "achievement"(id) ON DELETE CASCADE
);