use crate::persistence::achievement::AvailableAchievements;

use common::comp::{AchievementItem, AchievementList, InventoryUpdate, Player};
use specs::{Entities, Join, ReadExpect, ReadStorage, System};
use tracing::info;

pub struct Sys;

impl<'a> System<'a> for Sys {
    #[allow(clippy::type_complexity)] // TODO: Pending review in #587
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, AchievementList>,
        ReadStorage<'a, InventoryUpdate>,
        ReadExpect<'a, AvailableAchievements>,
    );

    fn run(
        &mut self,
        (entities, players, achievement_lists, inventory_updates, available_achievements): Self::SystemData,
    ) {
        for (_entity, _player, ach_list, inv_event) in
            (&entities, &players, &achievement_lists, &inventory_updates).join()
        {
            (available_achievements.0)
                .iter()
                .for_each(|achievement_item| {
                    let ach_item = AchievementItem::from(&achievement_item.details);
                    // pass the event to each achievement
                    // achievement checks if the event matches what it is looking for
                    if ach_item.matches_event(inv_event.event()) {
                        if let Some(event) = ach_list.process(&ach_item) {
                            info!(?event, "Achievement event");
                        }

                        // if it's a match, pass it to the characters
                        // achievement list
                        // get a result from the call to the players achievement
                        // list
                        // - It checks for an entry No entry = append and
                        //   increment Entry = Increment
                        // inrement(achievement_id, ?amount)
                        // if let Some(results) =
                        // _player_character_list.process(achievement_item) {
                        //
                        // - if its a completion result, dispatch an event which
                        //   notifies the client
                        // server_events.dispatch(ServerEvent::
                        // AchievementUpdate(TheAchievementInfo))
                        // }
                    }
                });
        }
    }
}
