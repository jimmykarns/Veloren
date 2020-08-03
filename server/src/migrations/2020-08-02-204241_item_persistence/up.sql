CREATE TABLE entity
(
    entity_id INTEGER NOT NULL
        CONSTRAINT entity_pk PRIMARY KEY AUTOINCREMENT
        CONSTRAINT entity_pk_2 UNIQUE
);

CREATE TABLE item
(
    item_id                  INTEGER NOT NULL
        CONSTRAINT item_pk PRIMARY KEY
        CONSTRAINT item_pk_2 UNIQUE,
    parent_container_item_id INTEGER NOT NULL
        REFERENCES item,
    item_definition_id       TEXT    NOT NULL,
    stack_size               INTEGER,
    position                 TEXT
);

CREATE INDEX idx_parent_container_item_id
    ON item(parent_container_item_id);

-- Create entity_id for world pseudo-container
INSERT
INTO    entity
VALUES  (NULL);

-- Create world pseudo-container
INSERT
INTO    item
VALUES  ((SELECT MAX(entity_id) FROM entity),
         (SELECT MAX(entity_id) FROM entity),
         'veloren.core.pseudo_containers.world',
         NULL,
         NULL);

-- Create a temporary table for mapping between the existing character ID and the new entity ID
PRAGMA temp_store = MEMORY;
CREATE /*TEMP*/ TABLE _new_character_ids
(
    character_id INT NOT NULL
        PRIMARY KEY,
    entity_id INT NOT NULL
);

-- Create an entity_id for each character
INSERT
INTO    entity
SELECT  NULL
FROM    character;

-- Populate the table with all existing character IDs and a new entity ID
INSERT INTO _new_character_ids
WITH entity_ids AS (
    SELECT  entity_id,
            ROW_NUMBER () OVER ( ORDER BY ROWID) AS rownum
    FROM    (
                SELECT  entity_id
                FROM    entity
                ORDER BY entity_id DESC
            )
),
     character_rownums AS (
         SELECT  *,
                 ROW_NUMBER () OVER ( ORDER BY id DESC) AS rownum
         FROM    character
     )
SELECT  c.id AS character_id,
        e.entity_id
FROM    character_rownums c
            JOIN    entity_ids e ON (e.rownum = c.rownum);

-- Body
CREATE TEMP TABLE _body_temp
(
    character_id INT NOT NULL
        PRIMARY KEY,
    species SMALLINT NOT NULL,
    body_type SMALLINT NOT NULL,
    hair_style SMALLINT NOT NULL,
    beard SMALLINT NOT NULL,
    eyes SMALLINT NOT NULL,
    accessory SMALLINT NOT NULL,
    hair_color SMALLINT NOT NULL,
    skin SMALLINT NOT NULL,
    eye_color SMALLINT NOT NULL
);

INSERT INTO _body_temp
SELECT  character_id, species, body_type, hair_style, beard, eyes, accessory, hair_color, skin, eye_color
FROM    body;

DROP TABLE body;

-- Loadout
CREATE TEMP TABLE _loadout_temp
(
    id INTEGER not null
        primary key,
    character_id INT not null
        references character,
    items TEXT NOT NULL
);

INSERT
INTO    _loadout_temp
SELECT  id,
        character_id,
        items
FROM    loadout;

DROP TABLE loadout;

-- Inventory
CREATE TEMP TABLE _inventory_temp
(
    character_id INTEGER NOT NULL
        PRIMARY KEY,
    items TEXT NOT NULL
);

INSERT
INTO    _inventory_temp
SELECT  character_id,
        items
FROM    inventory;

DROP TABLE inventory;

-- Stats
CREATE TEMP TABLE _stats_temp
(
    character_id INT NOT NULL
        PRIMARY KEY,
    level INT DEFAULT 1 NOT NULL,
    exp INT DEFAULT 0 NOT NULL,
    endurance INT DEFAULT 0 NOT NULL,
    fitness INT DEFAULT 0 NOT NULL,
    willpower INT default 0 NOT NULL,
    skills TEXT
);

INSERT
INTO    _stats_temp
SELECT  character_id, level, exp, endurance, fitness, willpower, skills
FROM    stats;

DROP TABLE stats;

-- Update characters to use new entity IDs
-- Add 1000000 to each character id since SQLite verifies unique constraints
-- on every individual row in an UPDATE statement. Remove it in the subsequent
-- UPDATE statement.
UPDATE  character
SET     id = (  SELECT  entity_id + 1000000
                FROM    _new_character_ids
                WHERE   character_id = character.id);

UPDATE  character
SET     id = id - 1000000;

-- Re-create character FK tables with new character IDs
CREATE TABLE body
(
    character_id INT NOT NULL
        PRIMARY KEY REFERENCES character,
    species SMALLINT NOT NULL,
    body_type SMALLINT NOT NULL,
    hair_style SMALLINT NOT NULL,
    beard SMALLINT NOT NULL,
    eyes SMALLINT NOT NULL,
    accessory SMALLINT NOT NULL,
    hair_color SMALLINT NOT NULL,
    skin SMALLINT NOT NULL,
    eye_color SMALLINT NOT NULL
);

INSERT
INTO    body
SELECT  nci.entity_id,
        species,
        body_type,
        hair_style,
        beard,
        eyes,
        accessory,
        hair_color,
        skin,
        eye_color
FROM    _body_temp b
            JOIN    _new_character_ids nci ON b.character_id = nci.character_id;

CREATE TABLE loadout
(
    id INTEGER not null
        primary key,
    character_id INT not null
        references character,
    items TEXT NOT NULL
);

INSERT
INTO    loadout
SELECT  l.id,
        nci.entity_id,
        l.items
FROM    _loadout_temp l
            JOIN    _new_character_ids nci ON l.character_id = nci.character_id;

CREATE TABLE inventory
(
    character_id INTEGER NOT NULL
        PRIMARY KEY,
    items TEXT NOT NULL
);

INSERT
INTO    inventory
SELECT  nci.entity_id,
        i.items
FROM    _inventory_temp i
            JOIN    _new_character_ids nci ON i.character_id = nci.character_id;

CREATE TABLE stats
(
    character_id INT NOT NULL
        PRIMARY KEY,
    level INT DEFAULT 1 NOT NULL,
    exp INT DEFAULT 0 NOT NULL,
    endurance INT DEFAULT 0 NOT NULL,
    fitness INT DEFAULT 0 NOT NULL,
    willpower INT DEFAULT 0 NOT NULL,
    skills TEXT
);

INSERT
INTO    stats
SELECT  nci.entity_id,
        s.level,
        s.exp,
        s.endurance,
        s.fitness,
        s.willpower,
        s.skills
FROM    _stats_temp s
            JOIN    _new_character_ids nci ON s.character_id = nci.character_id;

-- Character containers for existing characters
INSERT
INTO    item
SELECT  c.id,
        1, -- Parent container as World pseudo-container
        'veloren.core.pseudo_containers.character',
        NULL,
        NULL
FROM    character c;

CREATE TEMP TABLE _inventory_entity_ids
(
    character_id INT NOT NULL
        PRIMARY KEY,
    entity_id INT NOT NULL
);

-- Create an entity_id for each character's inventory pseudo-container
INSERT
INTO    entity
SELECT  NULL
FROM    character;

-- Populate the table with all existing character IDs and a new entity ID
INSERT INTO _inventory_entity_ids
WITH entity_ids AS (
    SELECT  entity_id,
            ROW_NUMBER () OVER ( ORDER BY ROWID) AS rownum
    FROM    (
                SELECT  entity_id
                FROM    entity
                ORDER BY entity_id DESC
            )
),
     character_rownums AS (
         SELECT  *,
                 ROW_NUMBER () OVER ( ORDER BY id DESC) AS rownum
         FROM    character
     )
SELECT  c.id AS character_id,
        e.entity_id
FROM    character_rownums c
            JOIN    entity_ids e ON (e.rownum = c.rownum);

-- Inventory containers for existing characters
INSERT
INTO    item
SELECT  i.entity_id,
        i.character_id, -- Inventory pseudo-container has character's Player item pseudo-container as its parent
        'veloren.core.pseudo_containers.inventory',
        NULL,
        NULL
FROM    _inventory_entity_ids i;