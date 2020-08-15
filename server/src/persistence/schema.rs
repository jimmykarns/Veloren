table! {
    body (character_id) {
        character_id -> Integer,
        species -> SmallInt,
        body_type -> SmallInt,
        hair_style -> SmallInt,
        beard -> SmallInt,
        eyes -> SmallInt,
        accessory -> SmallInt,
        hair_color -> SmallInt,
        skin -> SmallInt,
        eye_color -> SmallInt,
    }
}

table! {
    character (id) {
        id -> Integer,
        player_uuid -> Text,
        alias -> Text,
    }
}

table! {
    entity (entity_id) {
        entity_id -> Integer,
    }
}

table! {
    item (item_id) {
        item_id -> Integer,
        parent_container_item_id -> Integer,
        item_definition_id -> Text,
        stack_size -> Nullable<Integer>,
        position -> Nullable<Text>,
    }
}

table! {
    stats (character_id) {
        character_id -> Integer,
        level -> Integer,
        exp -> Integer,
        endurance -> Integer,
        fitness -> Integer,
        willpower -> Integer,
        skills -> Nullable<Text>,
    }
}

joinable!(body -> character (character_id));
joinable!(stats -> character (character_id));

allow_tables_to_appear_in_same_query!(body, character, entity, item, stats,);
