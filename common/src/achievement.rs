use crate::comp::item::{Item, ItemKind};

// TODO: Kill(Race, amount)
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementType {
    Collect(String, i32),
    ReachLevel(i32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AchievementItem {
    pub title: String,
    pub achievement_type: AchievementType,
}

impl Default for AchievementItem {
    fn default() -> Self {
        Self {
            title: String::new(),
            achievement_type: AchievementType::ReachLevel(9999),
        }
    }
}
