pub mod armor;
pub mod tool;

// Reexports
pub use tool::{Hands, Tool, ToolCategory, ToolKind};

use crate::{
    assets::{self, Asset, Error},
    effect::Effect,
    lottery::Lottery,
    terrain::{Block, BlockKind},
};
use serde::{Deserialize, Serialize};
use specs::{Component, FlaggedStorage};
use specs_idvs::IdvStorage;
use std::{
    fs::File,
    io::BufReader,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tracing::warn;
use vek::Rgb;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Throwable {
    Bomb,
    TrainingDummy,
    Firework(Reagent),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Reagent {
    Blue,
    Green,
    Purple,
    Red,
    Yellow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Utility {
    Collar,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lantern {
    pub kind: String,
    color: Rgb<u32>,
    strength_thousandths: u32,
    flicker_thousandths: u32,
}

impl Lantern {
    pub fn strength(&self) -> f32 { self.strength_thousandths as f32 / 1000_f32 }

    pub fn color(&self) -> Rgb<f32> { self.color.map(|c| c as f32 / 255.0) }
}

fn default_amount() -> u32 { 1 }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemKind {
    /// Something wieldable
    Tool(tool::Tool),
    Lantern(Lantern),
    Armor(armor::Armor),
    Consumable {
        kind: String,
        effect: Effect,
        #[serde(default = "default_amount")]
        amount: u32,
    },
    Throwable {
        kind: Throwable,
        #[serde(default = "default_amount")]
        amount: u32,
    },
    Utility {
        kind: Utility,
        #[serde(default = "default_amount")]
        amount: u32,
    },
    Ingredient {
        kind: String,
        #[serde(default = "default_amount")]
        amount: u32,
    },
}

impl ItemKind {
    pub fn stack_size(&self) -> Option<u32> {
        match self {
            ItemKind::Consumable {
                kind: _,
                effect: _,
                amount,
            } => Some(*amount),
            ItemKind::Throwable { kind: _, amount } => Some(*amount),
            ItemKind::Utility { kind: _, amount } => Some(*amount),
            ItemKind::Ingredient { kind: _, amount } => Some(*amount),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ItemId(AtomicU64);

impl Default for ItemId {
    fn default() -> Self { ItemId(AtomicU64::new(0)) }
}

impl ItemId {
    fn get(&self) -> Option<u64> { Some(self.0.load(Ordering::Relaxed)).filter(|x| *x != 0) }

    pub fn set(&mut self, item_id: u64) {
        if self.0.load(Ordering::Relaxed) > 0 {
            panic!("Cannot set an item_id twice");
        }
        self.0.store(item_id, Ordering::Relaxed);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq)]
pub struct Item {
    #[serde(skip)]
    #[derivative(PartialEq = "ignore")]
    pub item_id: Arc<AtomicU64>,
    item_definition_id: Option<String>, //TODO: Intern these strings?
    name: String,
    description: String,
    pub kind: ItemKind,
}

impl Asset for Item {
    const ENDINGS: &'static [&'static str] = &["ron"];

    fn parse(buf_reader: BufReader<File>) -> Result<Self, assets::Error> {
        ron::de::from_reader(buf_reader).map_err(assets::Error::parse_error)
    }
}

impl Item {
    // TODO: consider alternatives such as default abilities that can be added to a
    // loadout when no weapon is present
    pub fn empty() -> Self {
        info!("Created empty item");
        Self {
            item_id: Arc::new(AtomicU64::new(0)),
            item_definition_id: None,
            name: "Empty Item".to_owned(),
            description: "This item may grant abilities, but is invisible".to_owned(),
            kind: ItemKind::Tool(Tool::empty()),
        }
    }

    /// Creates a new instance of an `Item` from the provided asset identifier
    /// Panics if the asset does not exist.
    pub fn new_from_asset_expect(asset: &str) -> Self {
        let mut item = assets::load_expect_cloned::<Item>(asset);
        item.item_definition_id = Some(asset.to_owned());
        item.item_id = Arc::new(AtomicU64::new(0));
        item
    }

    /// Creates a Vec containing one of each item that matches the provided
    /// asset glob pattern
    pub fn new_from_asset_glob(asset_glob: &str) -> Result<Vec<Self>, Error> {
        let items = assets::load_glob_cloned::<Item>(asset_glob)?;

        Ok(items
            .into_iter()
            .map(|(mut item, identifier)| {
                item.item_definition_id = Some(identifier.to_string());
                item.item_id = Arc::new(AtomicU64::new(0));
                item
            })
            .collect::<Vec<_>>())
    }

    /// Creates a new instance of an `Item from the provided asset identifier if
    /// it exists
    pub fn new_from_asset(asset: &str) -> Result<Self, Error> {
        // Some commands like /give_item provide the asset specifier separated with \
        // instead of .
        let asset_specifier = asset.replace('\\', ".");

        let mut item = assets::load_cloned::<Item>(&asset_specifier)?;
        item.item_definition_id = Some(asset_specifier);
        item.item_id = Arc::new(AtomicU64::new(0));
        Ok(item)
    }

    pub fn duplicate(&self) -> Self {
        let mut item = self.clone();

        // Duplicated item has a 0 item ID as it is a new item instance
        item.item_id = Arc::new(AtomicU64::new(0));
        item
    }

    pub fn set_amount(&mut self, give_amount: u32) -> Result<(), assets::Error> {
        use ItemKind::*;
        match self.kind {
            Consumable { ref mut amount, .. }
            | Throwable { ref mut amount, .. }
            | Utility { ref mut amount, .. }
            | Ingredient { ref mut amount, .. } => {
                *amount = give_amount;
                Ok(())
            },
            Tool { .. } | Lantern { .. } | Armor { .. } => {
                // Tools and armor don't stack
                Err(assets::Error::InvalidType)
            },
        }
    }

    //pub fn item_id(&self) -> Option<u64> { self.item_id.get() }

    pub fn item_definition_id(&self) -> &str {
        match &self.item_definition_id {
            Some(x) => x.as_str(),
            _ => {
                warn!("Tried to get item_definition_id from item without one set.");
                "null_item_definition"
            },
        }
    }

    pub fn is_same_item_def_as(&self, other: &Item) -> bool {
        self.item_definition_id() == other.item_definition_id()
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn description(&self) -> &str { &self.description }

    pub fn amount(&self) -> u32 {
        match &self.kind {
            ItemKind::Tool(_) => 1,
            ItemKind::Lantern(_) => 1,
            ItemKind::Armor { .. } => 1,
            ItemKind::Consumable { amount, .. } => *amount,
            ItemKind::Throwable { amount, .. } => *amount,
            ItemKind::Utility { amount, .. } => *amount,
            ItemKind::Ingredient { amount, .. } => *amount,
        }
    }

    pub fn try_reclaim_from_block(block: Block) -> Option<Self> {
        match block.kind() {
            BlockKind::Apple => Some(Item::new_from_asset_expect("common.items.food.apple")),
            BlockKind::Mushroom => Some(Item::new_from_asset_expect("common.items.food.mushroom")),
            BlockKind::Velorite => Some(Item::new_from_asset_expect("common.items.ore.velorite")),
            BlockKind::VeloriteFrag => {
                Some(Item::new_from_asset_expect("common.items.ore.veloritefrag"))
            },
            BlockKind::BlueFlower => Some(Item::new_from_asset_expect("common.items.flowers.blue")),
            BlockKind::PinkFlower => Some(Item::new_from_asset_expect("common.items.flowers.pink")),
            BlockKind::PurpleFlower => {
                Some(Item::new_from_asset_expect("common.items.flowers.purple"))
            },
            BlockKind::RedFlower => Some(Item::new_from_asset_expect("common.items.flowers.red")),
            BlockKind::WhiteFlower => {
                Some(Item::new_from_asset_expect("common.items.flowers.white"))
            },
            BlockKind::YellowFlower => {
                Some(Item::new_from_asset_expect("common.items.flowers.yellow"))
            },
            BlockKind::Sunflower => Some(Item::new_from_asset_expect("common.items.flowers.sun")),
            BlockKind::LongGrass => Some(Item::new_from_asset_expect("common.items.grasses.long")),
            BlockKind::MediumGrass => {
                Some(Item::new_from_asset_expect("common.items.grasses.medium"))
            },
            BlockKind::ShortGrass => {
                Some(Item::new_from_asset_expect("common.items.grasses.short"))
            },
            BlockKind::Coconut => Some(Item::new_from_asset_expect("common.items.food.coconut")),
            BlockKind::Chest => {
                let chosen = assets::load_expect::<Lottery<String>>("common.loot_table");
                let chosen = chosen.choose();

                Some(Item::new_from_asset_expect(chosen))
            },
            BlockKind::Stones => Some(Item::new_from_asset_expect(
                "common.items.crafting_ing.stones",
            )),
            BlockKind::Twigs => Some(Item::new_from_asset_expect(
                "common.items.crafting_ing.twigs",
            )),
            BlockKind::ShinyGem => Some(Item::new_from_asset_expect(
                "common.items.crafting_ing.shiny_gem",
            )),
            _ => None,
        }
    }

    /// Determines whether two items are superficially equivalent to one another
    /// (i.e: one may be substituted for the other in crafting recipes or
    /// item possession checks).
    pub fn superficially_eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (ItemKind::Tool(a), ItemKind::Tool(b)) => a.superficially_eq(b),
            // TODO: Differentiate between lantern colors?
            (ItemKind::Lantern(_), ItemKind::Lantern(_)) => true,
            (ItemKind::Armor(a), ItemKind::Armor(b)) => a.superficially_eq(b),
            (ItemKind::Consumable { kind: a, .. }, ItemKind::Consumable { kind: b, .. }) => a == b,
            (ItemKind::Throwable { kind: a, .. }, ItemKind::Throwable { kind: b, .. }) => a == b,
            (ItemKind::Utility { kind: a, .. }, ItemKind::Utility { kind: b, .. }) => a == b,
            (ItemKind::Ingredient { kind: a, .. }, ItemKind::Ingredient { kind: b, .. }) => a == b,
            _ => false,
        }
    }
}

impl Component for Item {
    type Storage = FlaggedStorage<Self, IdvStorage<Self>>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemDrop(pub Item);

impl Component for ItemDrop {
    type Storage = FlaggedStorage<Self, IdvStorage<Self>>;
}
