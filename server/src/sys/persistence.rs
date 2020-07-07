use crate::{
    persistence::character,
    sys::{SysScheduler, SysTimer},
};
use common::comp::{AchievementList, Inventory, Loadout, Player, Stats};
use specs::{Join, ReadExpect, ReadStorage, System, Write};

pub struct Sys;

impl<'a> System<'a> for Sys {
    #[allow(clippy::type_complexity)] // TODO: Pending review in #587
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Stats>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, Loadout>,
        ReadStorage<'a, AchievementList>,
        ReadExpect<'a, character::CharacterUpdater>,
        Write<'a, SysScheduler<Self>>,
        Write<'a, SysTimer<Self>>,
    );

    fn run(
        &mut self,
        (
            players,
            player_stats,
            player_inventories,
            player_loadouts,
            player_achievements,
            updater,
            mut scheduler,
            mut timer,
        ): Self::SystemData,
    ) {
        if scheduler.should_run() {
            timer.start();

            updater.batch_update(
                (
                    &players,
                    &player_stats,
                    &player_inventories,
                    &player_loadouts,
                    &player_achievements,
                )
                    .join()
                    .filter_map(
                        |(player, stats, inventory, loadout, achievements)| {
                            player
                                .character_id
                                .map(|id| (id, stats, inventory, loadout, achievements))
                        },
                    ),
            );
            timer.end();
        }
    }
}
