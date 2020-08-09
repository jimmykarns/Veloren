use crate::persistence::models::Item;
use common::comp::Inventory;
use common::comp::item::{ItemKind};
use std::sync::atomic::Ordering;

pub struct ItemModelPair {
    pub comp: common::comp::item::Item,
    pub model: Item
}

pub fn convert_inventory(inventory: Inventory, character_container_id: i32) -> Vec<ItemModelPair> {
    inventory.slots.into_iter().filter_map(|x| x).map(|item| {
        info!("comp item_definition_id: {} item_id: {} unique_item_id: {}", item.item_definition_id(), item.item_id.load(Ordering::Relaxed), item.item_unique_id);

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

        info!("modl item_definition_id: {} item_id: {}", pair.model.item_definition_id, pair.model.item_id.map_or(0, |x| x));

        pair
    }).collect()
}