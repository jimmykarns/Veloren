use crate::comp::item::{Consumable, Item, ItemKind};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use specs::{Component, Entity, FlaggedStorage};
use specs_idvs::IdvStorage;

/// Used for in-game events that contribute towards player achievements.
///
/// For example, when an `InventoryManip` is detected, we record that event in
/// order to process achievements which depend on collecting items.
pub struct AchievementTrigger {
    pub entity: Entity,
    pub event: AchievementEvent,
}

/// Used to indicate an in-game event that can contribute towards the completion
/// of an achievement.
///
/// These are paired with `AchievementAction` items - for
/// example an event of type `LevelUp` will trigger a check for `ReachLevel`
/// achievements.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementEvent {
    None,
    CollectedItem(Item),
    LevelUp(u32),
    KilledPlayer,
    KilledNpc,
}

/// The types of achievements available in game
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementAction {
    None,
    CollectConsumable(Consumable),
    ReachLevel,
    KillPlayers,
    KillNpcs,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Achievement {
    pub uuid: String,
    pub title: String,
    pub action: AchievementAction,
    pub target: usize,
}

impl Achievement {
    pub fn matches_event(&self, event: &AchievementEvent) -> bool {
        match event {
            AchievementEvent::KilledNpc => self.action == AchievementAction::KillNpcs,
            AchievementEvent::KilledPlayer => self.action == AchievementAction::KillPlayers,
            AchievementEvent::LevelUp(_) => self.action == AchievementAction::ReachLevel,
            AchievementEvent::CollectedItem(item) => match self.action {
                AchievementAction::CollectConsumable(consumable) => {
                    if let ItemKind::Consumable { kind, .. } = item.kind {
                        kind == consumable
                    } else {
                        false
                    }
                },
                _ => false,
            },
            AchievementEvent::None => false,
        }
    }
}

/// The complete representation of an achievement that has been
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CharacterAchievement {
    pub achievement: Achievement,
    pub completed: bool,
    pub progress: usize,
}

impl CharacterAchievement {
    /// Increment the progress of this Achievement based on its type
    ///
    /// By default, when an achievement is incremented, its `progress` value is
    /// incremented by 1. This covers many cases, but using this method allows
    /// handling of unique types of achievements which are not simple
    /// counters for events
    pub fn increment_progress(&mut self, event: &AchievementEvent) -> Option<&mut Self> {
        if self.completed {
            return None;
        }

        match event {
            AchievementEvent::LevelUp(level) => {
                self.progress = *level as usize;
            },
            _ => self.progress += 1,
        };

        self.completed = self.progress >= self.achievement.target;

        if self.completed == true {
            Some(self)
        } else {
            None
        }
    }
}

/// For initialisation of a new CharacterAchievement item when a new achievement
/// is progressed
impl From<Achievement> for CharacterAchievement {
    fn from(achievement: Achievement) -> Self {
        Self {
            achievement,
            completed: false,
            progress: 0,
        }
    }
}

/// Each character is assigned an achievement list, which holds information
/// about which achievements that the player has made some progress on, or
/// completed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AchievementList(HashMap<String, CharacterAchievement>);

impl Default for AchievementList {
    fn default() -> AchievementList { AchievementList(HashMap::new()) }
}

impl AchievementList {
    pub fn from(data: HashMap<String, CharacterAchievement>) -> Self { Self(data) }
}

impl Component for AchievementList {
    type Storage = FlaggedStorage<Self, IdvStorage<Self>>; // TODO check
}

impl AchievementList {
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Process a single CharacterAchievement item based on the occurance of an
    /// `AchievementEvent`.
    ///
    /// When the character has existing progress on the achievement it is
    /// updated, otherwise an insert-then-update is performed.
    ///
    /// Returns the `CharacterAchievement` item that was processed in the event
    /// that prcessing resulted in its completion.
    pub fn process_achievement(
        &mut self,
        achievement: &Achievement,
        event: &AchievementEvent,
    ) -> Option<CharacterAchievement> {
        let uuid = achievement.uuid.clone();

        self.0
            .entry(uuid)
            .or_insert(CharacterAchievement::from(achievement.clone()))
            .increment_progress(event)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assets, comp::item::Consumable};

    #[test]
    fn inv_collect_event_matches_consumable_achievement_item() {
        let item = Achievement {
            uuid: String::new(),
            title: String::new(),
            action: AchievementAction::CollectConsumable(Consumable::Apple),
            target: 10,
        };

        let event =
            AchievementEvent::CollectedItem(assets::load_expect_cloned("common.items.apple"));

        assert!(item.matches_event(&event));
    }

    #[test]
    fn inv_collect_event_not_matches_consumable_achievement_item() {
        let item = Achievement {
            uuid: String::new(),
            title: String::new(),
            action: AchievementAction::CollectConsumable(Consumable::Cheese),
            target: 10,
        };

        let event =
            AchievementEvent::CollectedItem(assets::load_expect_cloned("common.items.apple"));

        assert_eq!(item.matches_event(&event), false);
    }

    #[test]
    fn levelup_event_matches_reach_level_achievement_item() {
        let item = Achievement {
            uuid: String::new(),
            title: String::new(),
            action: AchievementAction::ReachLevel,
            target: 100,
        };

        let event = AchievementEvent::LevelUp(3);

        assert_eq!(item.matches_event(&event), true);
    }

    #[test]
    fn process_achievement_increments_progress() {
        let uuid = String::from("2ef30659-5884-40aa-ba4d-8f5af32ff9ac");

        let achievement = Achievement {
            uuid: uuid.clone(),
            title: String::from("Collect 3 Mushrooms"),
            action: AchievementAction::CollectConsumable(Consumable::Mushroom),
            target: 3,
        };

        let event =
            AchievementEvent::CollectedItem(assets::load_expect_cloned("common.items.mushroom"));

        let mut achievement_list = AchievementList::default();

        // The first two increments should not indicate that it is complete
        assert_eq!(
            achievement_list.process_achievement(&achievement, &event),
            None
        );

        assert_eq!(
            achievement_list.process_achievement(&achievement, &event),
            None
        );

        assert_eq!(achievement_list.0.get(&uuid).unwrap().progress, 2);
    }

    #[test]
    fn process_achievement_returns_achievement_when_complete() {
        let uuid = String::from("Test");

        let achievement = Achievement {
            uuid: uuid.clone(),
            title: String::from("Reach Level 10"),
            action: AchievementAction::ReachLevel,
            target: 10,
        };

        let mut achievement_list = AchievementList::default();

        achievement_list.process_achievement(&achievement, &AchievementEvent::LevelUp(6));

        // The achievement progress should be the new level value, but be incomplete
        let incomplete_result = achievement_list.0.get(&uuid).unwrap();

        assert_eq!(incomplete_result.progress, 6);
        assert_eq!(incomplete_result.completed, false);

        achievement_list.process_achievement(&achievement, &AchievementEvent::LevelUp(10));

        // The achievement progress should be the new level value, and be completed
        let complete_result = achievement_list.0.get(&uuid).unwrap();

        assert_eq!(complete_result.progress, 10);
        assert_eq!(complete_result.completed, true);
    }

    #[test]
    fn process_completed_achievement_doesnt_increment_progress() {
        let uuid = String::from("Test");

        let achievement = Achievement {
            uuid: uuid.clone(),
            title: String::from("Collect 3 Mushrooms"),
            action: AchievementAction::KillNpcs,
            target: 3,
        };

        // Initialise with the already completed event
        let mut achievement_list = AchievementList::default();

        achievement_list
            .0
            .insert(uuid.clone(), CharacterAchievement {
                achievement: achievement.clone(),
                completed: true,
                progress: 3,
            });

        achievement_list.process_achievement(&achievement, &AchievementEvent::KilledNpc);

        let result = achievement_list.0.get(&uuid).unwrap();

        // The achievement progress should not have incremented
        assert_eq!(result.progress, 3);
    }
}
