use crate::persistence::models::{Item, Character, ItemQuery};
use common::comp::Inventory;
use common::comp::item::{ItemKind};
use std::sync::atomic::{Ordering, AtomicU64};
use std::sync::Arc;
use tracing::info;

pub struct ItemModelPair {
    pub comp: common::comp::item::Item,
    pub model: Item
}

pub fn convert_inventory_to_database_items(inventory: Inventory, character_container_id: i32) -> Vec<ItemModelPair> {
    inventory.slots.into_iter().filter_map(|x| x).map(|item| {
        let pair = ItemModelPair {
            model: Item {
                item_definition_id: item.item_definition_id().to_owned(),
                position: None, // TODO
                parent_container_item_id: character_container_id,
                item_id: match item.item_id.load(Ordering::Relaxed) {
                    x if x > 0 => Some(x as i32),
                    _ => None
                }, //item.item_id().map(|x| x as i32), // TODO: Remove this downcast, change database type to BigInteger
                stack_size: match item.kind {
                    ItemKind::Consumable { kind: _, effect: _, amount } => Some(amount as i32),
                    ItemKind::Throwable { kind: _, amount } => Some(amount as i32),
                    ItemKind::Utility { kind: _, amount } => Some(amount as i32),
                    ItemKind::Ingredient { kind: _, amount } => Some(amount as i32),
                    _ => None
                }
            },
            comp: item
        };
        pair
    }).collect()
}
pub fn convert_inventory_from_database_items(database_items: &Vec<ItemQuery>) -> Inventory {
    let mut inventory = Inventory::new_empty();
    let item_iter = database_items.iter().map(|db_item| {
        let mut item = common::comp::Item::new_from_asset_expect(db_item.item_definition_id.as_str());
        item.item_id = Arc::new(AtomicU64::new(db_item.item_id as u64));
        if let Some(amount) = db_item.stack_size {
            info!("Setting item amount for {} to {}", db_item.item_definition_id, amount);
            item.set_amount(amount as u32);
        }
        item
    });
    inventory.push_all(item_iter);
    inventory
}

pub fn convert_character_from_database(character: &Character) -> common::character::Character {
    common::character::Character {
        id: Some(character.id),
        tool: character.tool.clone(),
        alias: String::from(&character.alias)
    }
}