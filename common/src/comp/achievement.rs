use crate::comp::item::{Consumable, Item, ItemKind};
use specs::{Component, FlaggedStorage};
use specs_idvs::IDVStorage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementEvent {
    None,
    CollectedItem(Item),
    LevelUp(u32),
}

/// The types of achievements available in game
///
/// Some potential additions in the future:
/// - ReachCoordinate
/// - CollectCurrency
/// - KillPlayers
/// - OpenChests
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementAction {
    None,
    CollectConsumable(Consumable),
    ReachLevel,
}

/// Information about an achievement. This differs from a complete
/// [`Achievement`](struct.Achievement.html) in that it describes the
/// achievement without any information about progress
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AchievementItem {
    pub title: String,
    pub action: AchievementAction,
    pub target: usize,
}

impl AchievementItem {
    pub fn matches_event(&self, event: AchievementEvent) -> bool {
        match event {
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
            _ => {
                tracing::warn!(
                    ?event,
                    "An AchievementEvent was processed but the event was not handled"
                );

                false
            },
        }
    }
}

/// The complete representation of an achievement that has been
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Achievement {
    pub id: i32,
    pub item: AchievementItem,
    pub completed: bool,
    pub progress: usize,
}

impl Achievement {
    /// Increment the progress of this Achievement based on its type
    ///
    /// By default, when an achievement is incremented, its `progress` value is
    /// incremented by 1. This covers many cases, but using this method allows
    /// handling of unique types of achievements which are not simple
    /// counters for events
    pub fn increment_progress(&mut self, event: AchievementEvent) -> bool {
        match event {
            AchievementEvent::LevelUp(level) => {
                self.progress = level as usize;
            },
            _ => self.progress += 1,
        };

        self.completed = self.progress >= self.item.target;
        self.completed
    }
}

/// Each character is assigned an achievement list, which holds information
/// about which achievements that the player has made some progress on, or
/// completed.
///
/// This minimises storage of data per-character, and can be merged with a full
/// achievement list
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AchievementList(Vec<Achievement>);

impl Default for AchievementList {
    fn default() -> AchievementList { AchievementList(Vec::new()) }
}

impl AchievementList {
    pub fn from(data: Vec<Achievement>) -> Self { Self(data) }
}

impl Component for AchievementList {
    type Storage = FlaggedStorage<Self, IDVStorage<Self>>;
}

impl AchievementList {
    pub fn item_by_id(&mut self, id: i32) -> Option<&mut Achievement> {
        self.0.iter_mut().find(|a| a.id == id)
    }

    /// Process a single achievement item, inrementing the progress of the
    /// achievement. This is called as part of server/sys/Achievements.
    pub fn process_achievement(
        &mut self,
        achievement: Achievement,
        event: AchievementEvent,
    ) -> bool {
        let id = achievement.id;

        if !self.0.contains(&achievement) {
            self.0.push(achievement);
        }

        return if let Some(char_achievement) = self.item_by_id(id) {
            if char_achievement.completed {
                return false;
            }

            char_achievement.increment_progress(event)
        } else {
            tracing::warn!("Failed to find achievement after inserting");

            false
        };
    }
}

/// Used as a container for in-game events that contribute towards player
/// achievements.
///
/// For example, when an `InventoryManip` is detected, we record that event in
/// order to process achievements which depend on collecting items.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementUpdate {
    event: AchievementEvent,
}

impl AchievementUpdate {
    pub fn new(event: AchievementEvent) -> Self { Self { event } }

    pub fn event(&self) -> AchievementEvent { self.event.clone() }
}

impl Component for AchievementUpdate {
    type Storage = FlaggedStorage<Self, IDVStorage<Self>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assets, comp::item::Consumable};

    #[test]
    fn inv_collect_event_matches_consumable_achievement_item() {
        let item = AchievementItem {
            title: String::from("Test"),
            action: AchievementAction::CollectConsumable(Consumable::Apple),
            target: 10,
        };

        let event =
            AchievementEvent::CollectedItem(assets::load_expect_cloned("common.items.apple"));

        assert!(item.matches_event(event));
    }

    #[test]
    fn inv_collect_event_not_matches_consumable_achievement_item() {
        let item = AchievementItem {
            title: String::from("Test"),
            action: AchievementAction::CollectConsumable(Consumable::Cheese),
            target: 10,
        };

        let event =
            AchievementEvent::CollectedItem(assets::load_expect_cloned("common.items.apple"));

        assert_eq!(item.matches_event(event), false);
    }

    #[test]
    fn levelup_event_matches_reach_level_achievement_item() {
        let item = AchievementItem {
            title: String::from("Test"),
            action: AchievementAction::ReachLevel,
            target: 100,
        };

        let event = AchievementEvent::LevelUp(3);

        assert_eq!(item.matches_event(event), true);
    }

    #[test]
    fn process_collect_achievement_increments_progress() {
        let item = AchievementItem {
            title: String::from("Collect 3 Mushrooms"),
            action: AchievementAction::CollectConsumable(Consumable::Mushroom),
            target: 3,
        };

        let achievement = Achievement {
            id: 1,
            item,
            completed: false,
            progress: 0,
        };

        let event =
            AchievementEvent::CollectedItem(assets::load_expect_cloned("common.items.mushroom"));

        let mut achievement_list = AchievementList::default();

        // The first two increments should not indicate that it is complete
        assert_eq!(
            achievement_list.process_achievement(achievement.clone(), event.clone()),
            false
        );

        assert_eq!(
            achievement_list.process_achievement(achievement.clone(), event.clone()),
            false
        );

        assert_eq!(achievement_list.0.get(0).unwrap().progress, 2);

        // It should return true when completed
        assert_eq!(
            achievement_list.process_achievement(achievement, event),
            true
        );

        assert_eq!(achievement_list.0.get(0).unwrap().progress, 3);

        // The achievement `completed` field should be true
        assert_eq!(achievement_list.0.get(0).unwrap().completed, true);
    }

    #[test]
    fn process_levelup_achievement_increments_progress() {
        let item = AchievementItem {
            title: String::from("Reach Level 10"),
            action: AchievementAction::ReachLevel,
            target: 10,
        };

        let achievement = Achievement {
            id: 1,
            item,
            completed: false,
            progress: 1,
        };

        let mut achievement_list = AchievementList::default();

        assert_eq!(
            achievement_list.process_achievement(achievement.clone(), AchievementEvent::LevelUp(6)),
            false
        );

        // The achievement progress should be the new level value, and be incomplete
        assert_eq!(achievement_list.0.get(0).unwrap().progress, 6);
        assert_eq!(achievement_list.0.get(0).unwrap().completed, false);

        assert_eq!(
            achievement_list.process_achievement(achievement, AchievementEvent::LevelUp(10)),
            true
        );

        // The achievement progress should be the new level value, and be completed
        assert_eq!(achievement_list.0.get(0).unwrap().progress, 10);
        assert_eq!(achievement_list.0.get(0).unwrap().completed, true);
    }

    #[test]
    fn process_completed_achievement_doesnt_increment_progress() {
        let item = AchievementItem {
            title: String::from("Collect 3 Mushrooms"),
            action: AchievementAction::CollectConsumable(Consumable::Mushroom),
            target: 3,
        };

        let achievement = Achievement {
            id: 1,
            item,
            completed: true,
            progress: 3,
        };

        let mut achievement_list = AchievementList(vec![achievement.clone()]);

        let event =
            AchievementEvent::CollectedItem(assets::load_expect_cloned("common.items.mushroom"));

        assert_eq!(
            achievement_list.process_achievement(achievement, event),
            false
        );

        // The achievement progress should not have incremented
        assert_eq!(achievement_list.0.get(0).unwrap().progress, 3);
    }
}
