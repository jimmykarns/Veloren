use crate::persistence::models::{Character, Item, NewItem, Stats};
use crate::persistence::character::EntityId;

use common::{character::CharacterId, comp::*, loadout_builder};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tracing::warn;

pub struct ItemModelPair {
    pub comp: common::comp::item::Item,
    pub model: NewItem,
}

pub fn convert_inventory_to_database_items(
    inventory: Inventory,
    inventory_container_id: EntityId,
) -> Vec<ItemModelPair> {
    inventory
        .slots
        .into_iter()
        .filter_map(|x| x)
        .map(|item| {
            ItemModelPair {
                model: NewItem {
                    item_definition_id: item.item_definition_id().to_owned(),
                    position: None, // TODO
                    parent_container_item_id: inventory_container_id,
                    item_id: match item.item_id.load(Ordering::Relaxed) {
                        x if x > 0 => Some(x as EntityId),
                        _ => None,
                    },
                    stack_size: item.kind.stack_size().map(|x| x as i32),
                },
                comp: item,
            }
        })
        .collect()
}

pub fn convert_loadout_to_database_items(
    loadout: Loadout,
    loadout_container_id: EntityId,
) -> Vec<ItemModelPair> {
    vec![
        loadout.active_item.map(|x| ("active_item", x.item)),
        loadout.second_item.map(|x| ("second_item", x.item)),
        loadout.lantern.map(|x| ("lantern", x)),
        loadout.shoulder.map(|x| ("shoulder", x)),
        loadout.chest.map(|x| ("chest", x)),
        loadout.belt.map(|x| ("belt", x)),
        loadout.hand.map(|x| ("hand", x)),
        loadout.pants.map(|x| ("pants", x)),
        loadout.foot.map(|x| ("foot", x)),
        loadout.back.map(|x| ("back", x)),
        loadout.ring.map(|x| ("ring", x)),
        loadout.neck.map(|x| ("neck", x)),
        loadout.head.map(|x| ("head", x)),
        loadout.tabard.map(|x| ("tabard", x)),
    ]
    .iter()
    .filter(|x| x.is_some())
    .map(|x| {
        let (slot, item) = x.as_ref().unwrap();
        ItemModelPair {
            model: NewItem {
                item_definition_id: item.item_definition_id().to_owned(),
                position: Some((*slot).to_owned()),
                parent_container_item_id: loadout_container_id,
                item_id: match item.item_id.load(Ordering::Relaxed) {
                    x if x > 0 => Some(x as EntityId),
                    _ => None,
                },
                stack_size: None, // Armor/weapons cannot have stack sizes
            },
            comp: item.clone(), // TODO don't clone?
        }
    })
    .collect()
}

pub fn convert_stats_to_database(character_id: CharacterId, stats: &common::comp::Stats) -> Stats {
    Stats {
        character_id,
        level: stats.level.level() as i32,
        exp: stats.exp.current() as i32,
        endurance: stats.endurance as i32,
        fitness: stats.fitness as i32,
        willpower: stats.willpower as i32,
        skills: Some("".to_owned()), // TODO: actual skillset
    }
}

pub fn convert_inventory_from_database_items(database_items: &[Item]) -> Inventory {
    let mut inventory = Inventory::new_empty();
    let item_iter = database_items.iter().map(|db_item| {
        let mut item =
            common::comp::Item::new_from_asset_expect(db_item.item_definition_id.as_str());
        item.item_id = Arc::new(AtomicU64::new(db_item.item_id as u64));
        if let Some(amount) = db_item.stack_size {
            if item.set_amount(amount as u32).is_err() {
                warn!(?item, "Error setting amount for item");
            };
        }
        item
    });

    if let Err(e) = inventory.push_all(item_iter) {
        match e {
            common::comp::inventory::Error::Full(_) => {
                warn!("Unable to push items to inventory during database load, inventory full");
            },
        }
    };
    inventory
}

pub fn convert_loadout_from_database_items(database_items: &[Item]) -> Loadout {
    let mut loadout = loadout_builder::LoadoutBuilder::new();
    for db_item in database_items.iter() {
        let mut item =
            common::comp::Item::new_from_asset_expect(db_item.item_definition_id.as_str());
        item.item_id = Arc::new(AtomicU64::new(db_item.item_id as u64));
        let item_opt = Some(item);
        if let Some(position) = &db_item.position {
            match position.as_str() {
                "active_item" => {
                    loadout = loadout.active_item(Some(slot::item_config(item_opt.unwrap())))
                },
                "second_item" => {
                    loadout = loadout.second_item(Some(slot::item_config(item_opt.unwrap())))
                },
                "lantern" => loadout = loadout.lantern(item_opt),
                "shoulder" => loadout = loadout.shoulder(item_opt),
                "chest" => loadout = loadout.chest(item_opt),
                "belt" => loadout = loadout.belt(item_opt),
                "hand" => loadout = loadout.hand(item_opt),
                "pants" => loadout = loadout.pants(item_opt),
                "foot" => loadout = loadout.foot(item_opt),
                "back" => loadout = loadout.back(item_opt),
                "ring" => loadout = loadout.ring(item_opt),
                "neck" => loadout = loadout.neck(item_opt),
                "head" => loadout = loadout.head(item_opt),
                "tabard" => loadout = loadout.tabard(item_opt),
                _ => warn!(?db_item.item_id, ?db_item.position, "Unknown loadout position on item"),
            }
        }
    }

    loadout.build()
}

pub fn convert_character_from_database(character: &Character) -> common::character::Character {
    common::character::Character {
        id: Some(character.id),
        alias: String::from(&character.alias),
    }
}

pub fn convert_stats_from_database(stats: &Stats, alias: String) -> common::comp::Stats {
    let mut new_stats = common::comp::Stats::default();
    new_stats.name = alias;
    new_stats.level.set_level(stats.level as u32);
    new_stats.exp.set_current(stats.exp as u32);
    new_stats.update_max_hp(new_stats.body_type);
    new_stats.health.set_to(
        new_stats.health.maximum(),
        common::comp::HealthSource::Revive,
    );
    new_stats.endurance = stats.endurance as u32;
    new_stats.fitness = stats.fitness as u32;
    new_stats.willpower = stats.willpower as u32;

    // TODO: Skillset

    new_stats
}
