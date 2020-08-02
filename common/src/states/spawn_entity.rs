use crate::{
    comp::{Agent, CharacterState, Scale, StateUpdate},
    event::ServerEvent,
    states::utils::*,
    sys::character_behavior::*,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// Can you hold the abilty beyond the prepare duration
    pub holdable: bool,
    /// How long we have to prepare the weapon
    pub prepare_duration: Duration,
    /// How long we have already prepared the weapon
    pub prepare_timer: Duration,
    /// How long the state has until exiting
    pub recover_duration: Duration,
    /// Whether the attack fired already
    pub exhausted: bool,
}

impl CharacterBehavior for Data {
    fn behavior(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);

        handle_move(data, &mut update, 0.3);
        handle_jump(data, &mut update);

        println!("Yo");

        if !self.exhausted
            && if self.holdable {
                data.inputs.holding_ability_key() || self.prepare_timer < self.prepare_duration
            } else {
                self.prepare_timer < self.prepare_duration
            }
        {
            // Prepare
            update.character = CharacterState::SpawnEntity(Data {
                prepare_timer: self.prepare_timer + Duration::from_secs_f32(data.dt.0),
                holdable: self.holdable,
                prepare_duration: self.prepare_duration,
                recover_duration: self.recover_duration,
                exhausted: false,
            });
        } else if !self.exhausted {
            // Fire
            update.server_events.push_front(ServerEvent::CreateNpc {
                pos: data.pos.clone(),
                stats: data.stats.clone(),
                loadout: data.loadout.clone(),
                body: data.body.clone(),
                agent: Some(Agent::new(data.pos.0, true)),
                alignment: crate::comp::Alignment::Owned(*data.uid),
                scale: Scale(1.0),
                drop_item: None,
            });

            update.character = CharacterState::SpawnEntity(Data {
                prepare_timer: self.prepare_timer,
                holdable: self.holdable,
                prepare_duration: self.prepare_duration,
                recover_duration: self.recover_duration,
                exhausted: true,
            });
        } else if self.recover_duration != Duration::default() {
            // Recovery
            update.character = CharacterState::SpawnEntity(Data {
                prepare_timer: self.prepare_timer,
                holdable: self.holdable,
                prepare_duration: self.prepare_duration,
                recover_duration: self
                    .recover_duration
                    .checked_sub(Duration::from_secs_f32(data.dt.0))
                    .unwrap_or_default(),
                exhausted: true,
            });
            return update;
        } else {
            // Done
            update.character = CharacterState::Wielding;
        }

        update
    }
}
