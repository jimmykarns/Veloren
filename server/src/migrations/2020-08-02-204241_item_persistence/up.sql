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

-- Create entity ID for World pseudo-container
INSERT INTO entity VALUES (NULL);

-- Create World pseudo-container item
WITH world_entity_id AS (SELECT entity_id
                         FROM entity
                         ORDER BY entity_id DESC
                         LIMIT 1)
INSERT INTO item (item_id,
                  parent_container_item_id,
                  item_definition_id,
                  stack_size,
                  position)
VALUES ((SELECT entity_id FROM world_entity_id), (SELECT entity_id FROM world_entity_id),
        'veloren.core.pseudo_containers.world', NULL, NULL);