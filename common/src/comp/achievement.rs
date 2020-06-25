use crate::{
    comp::{self, inventory::item::Consumable, InventoryUpdateEvent},
    event::ServerEvent,
};
use specs::{Component, FlaggedStorage};
use specs_idvs::IDVStorage;

pub enum AchievementCategory {
    CollectConsumable,
    ReachLevel,
    KillHumanoidSpecies,
    KillBodyType,
}

// Potential additions
// - ReachCoordinate
// - CollectCurrency(amount)
// - KillPlayers(amount)
// - OpenChests
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementType {
    CollectConsumable(Consumable, i32),
    ReachLevel(i32),
    KillHumanoidSpecies(comp::body::humanoid::Species, i32),
    KillBodyType(comp::Body, i32),
}

impl From<AchievementType> for AchievementCategory {
    fn from(achievement_type: AchievementType) -> AchievementCategory {
        match achievement_type {
            AchievementType::CollectConsumable(_, _) => AchievementCategory::CollectConsumable,
            AchievementType::ReachLevel(_) => AchievementCategory::ReachLevel,
            AchievementType::KillHumanoidSpecies(_, _) => AchievementCategory::KillHumanoidSpecies,
            AchievementType::KillBodyType(_, _) => AchievementCategory::KillBodyType,
        }
    }
}

/// The representation of an achievement that is declared in .ron config.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AchievementItem {
    pub title: String,
    pub achievement_type: AchievementType,
}

/// TODO remove this, it's a confusing state
impl Default for AchievementItem {
    fn default() -> Self {
        Self {
            title: String::new(),
            achievement_type: AchievementType::ReachLevel(0),
        }
    }
}

impl AchievementItem {
    pub fn matches_event(&self, event: InventoryUpdateEvent) -> bool {
        match event {
            InventoryUpdateEvent::Collected(_item) => true,
            _ => false,
        }
    }
}

/// The complete representation of an achievement that has been
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Achievement {
    pub id: i32,
    pub item: AchievementItem,
    pub completed: bool,
    pub progress: i32,
}

// impl Achievement {
//     pub fn incr(&self, event: InventoryUpdateEvent) -> Option<bool> {
//
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AchievementList(Vec<Achievement>);

impl Default for AchievementList {
    fn default() -> AchievementList { AchievementList(Vec::new()) }
}

impl AchievementList {
    /// Process a single achievement item, inrementing or doing whataver it does
    /// to indicate it's one step closer to cmpletion
    pub fn process(&self, item: &AchievementItem) -> Option<bool> {
        // if self.0.iter().

        None
    }
}

impl Component for AchievementList {
    type Storage = FlaggedStorage<Self, IDVStorage<Self>>;
}
