use crate::persistence::achievement::AvailableAchievements;

use crate::client::Client;
use common::{
    comp::{Achievement, AchievementItem, AchievementList, AchievementUpdate},
    msg::ServerMsg,
};
use specs::{Join, ReadExpect, System, WriteStorage};
pub struct Sys;

impl<'a> System<'a> for Sys {
    #[allow(clippy::type_complexity)] // TODO: Pending review in #587
    type SystemData = (
        WriteStorage<'a, Client>,
        WriteStorage<'a, AchievementList>,
        WriteStorage<'a, AchievementUpdate>,
        ReadExpect<'a, AvailableAchievements>,
    );

    fn run(
        &mut self,
        (mut clients, mut achievement_lists, mut achievement_updates, available_achievements): Self::SystemData,
    ) {
        // TODO filter out achievements which do not care about this event here, then
        // iterate over them
        for (client, achievement_list, ach_update) in
            (&mut clients, &mut achievement_lists, &achievement_updates).join()
        {
            (available_achievements.0).iter().for_each(|achievement| {
                let achievement_item = AchievementItem::from(&achievement.details);

                if achievement_item.matches_event(ach_update.event()) {
                    // Calls to `process_achievement` return true to indicate that the
                    // achievement is complete. In this case, we notify the client to notify them of
                    // completing the achievement
                    if achievement_list
                        .process_achievement(Achievement::from(achievement), ach_update.event())
                        == true
                    {
                        client.notify(ServerMsg::AchievementCompletion);
                    }
                }
            });
        }

        achievement_updates.clear();
    }
}
