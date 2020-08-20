table! {
    body (body_id) {
        body_id -> Integer,
        variant -> Text,
        body_data -> Text,
    }
}

table! {
    character (character_id) {
        character_id -> BigInt,
        body_id -> Integer,
        player_uuid -> Text,
        alias -> Text,
    }
}

table! {
    entity (entity_id) {
        entity_id -> BigInt,
    }
}

table! {
    item (item_id) {
        item_id -> BigInt,
        parent_container_item_id -> BigInt,
        item_definition_id -> Text,
        stack_size -> Nullable<Integer>,
        position -> Nullable<Text>,
    }
}

table! {
    stats (character_id) {
        character_id -> BigInt,
        level -> Integer,
        exp -> Integer,
        endurance -> Integer,
        fitness -> Integer,
        willpower -> Integer,
        skills -> Nullable<Text>,
    }
}

joinable!(character -> body (body_id));
joinable!(stats -> character (character_id));

allow_tables_to_appear_in_same_query!(body, character, entity, item, stats,);
