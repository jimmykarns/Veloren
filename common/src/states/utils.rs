use crate::{
    comp::{
        item::{Hands, ItemKind, Tool},
        Body, CharacterState, StateUpdate,
    },
    event::LocalEvent,
    states::*,
    sys::{character_behavior::JoinData, phys::GRAVITY},
    util::Dir,
};
use vek::*;

pub const MOVEMENT_THRESHOLD_VEL: f32 = 3.0;
const BASE_HUMANOID_AIR_ACCEL: f32 = 8.0;
const BASE_HUMANOID_WATER_ACCEL: f32 = 150.0;
const BASE_HUMANOID_WATER_SPEED: f32 = 180.0;
// const BASE_HUMANOID_CLIMB_ACCEL: f32 = 10.0;
// const ROLL_SPEED: f32 = 17.0;
// const CHARGE_SPEED: f32 = 20.0;
// const GLIDE_ACCEL: f32 = 15.0;
// const GLIDE_SPEED: f32 = 45.0;
// const BLOCK_ACCEL: f32 = 30.0;
// const BLOCK_SPEED: f32 = 75.0;
// Gravity is 9.81 * 4, so this makes gravity equal to .15 //TODO: <- is wrong
//
// const GLIDE_ANTIGRAV: f32 = GRAVITY * 0.96;
// const CLIMB_SPEED: f32 = 5.0;
// const CLIMB_COST: i32 = 5;

impl Body {
    pub fn base_accel(&self) -> f32 {
        match self {
            Body::Humanoid(_) => 100.0,
            Body::QuadrupedSmall(_) => 125.0,
            Body::QuadrupedMedium(_) => 180.0,
            Body::BirdMedium(_) => 80.0,
            Body::FishMedium(_) => 50.0,
            Body::Dragon(_) => 250.0,
            Body::BirdSmall(_) => 75.0,
            Body::FishSmall(_) => 40.0,
            Body::BipedLarge(_) => 75.0,
            Body::Object(_) => 40.0,
            Body::Golem(_) => 60.0,
            Body::Critter(_) => 85.0,
            Body::QuadrupedLow(_) => 120.0,
        }
    }

    pub fn base_ori_rate(&self) -> f32 {
        match self {
            Body::Humanoid(_) => 20.0,
            Body::QuadrupedSmall(_) => 15.0,
            Body::QuadrupedMedium(_) => 10.0,
            Body::BirdMedium(_) => 30.0,
            Body::FishMedium(_) => 5.0,
            Body::Dragon(_) => 5.0,
            Body::BirdSmall(_) => 35.0,
            Body::FishSmall(_) => 10.0,
            Body::BipedLarge(_) => 12.0,
            Body::Object(_) => 5.0,
            Body::Golem(_) => 8.0,
            Body::Critter(_) => 35.0,
            Body::QuadrupedLow(_) => 12.0,
        }
    }
}

/// Handles updating `Components` to move player based on state of `JoinData`
pub fn handle_move(data: &JoinData, update: &mut StateUpdate, efficiency: f32) {
    if let Some(depth) = data.physics.in_fluid {
        swim_move(data, update, efficiency, depth);
    } else {
        basic_move(data, update, efficiency);
    }
}

/// Updates components to move player as if theyre on ground or in air
#[allow(clippy::assign_op_pattern)] // TODO: Pending review in #587
fn basic_move(data: &JoinData, update: &mut StateUpdate, efficiency: f32) {
    let accel = if data.physics.on_ground {
        data.body.base_accel()
    } else {
        BASE_HUMANOID_AIR_ACCEL
    };

    update.vel.0 =
        update.vel.0 + Vec2::broadcast(data.dt.0) * data.inputs.move_dir * accel * efficiency;

    handle_orientation(data, update, data.body.base_ori_rate());
}

pub fn handle_orientation(data: &JoinData, update: &mut StateUpdate, rate: f32) {
    // Set direction based on move direction
    let ori_dir = if update.character.is_attack() || update.character.is_block() {
        data.inputs.look_dir.xy()
    } else if !data.inputs.move_dir.is_approx_zero() {
        data.inputs.move_dir
    } else {
        update.ori.0.xy()
    };

    // Smooth orientation
    update.ori.0 = Dir::slerp_to_vec3(update.ori.0, ori_dir.into(), rate * data.dt.0);
}

/// Updates components to move player as if theyre swimming
fn swim_move(data: &JoinData, update: &mut StateUpdate, efficiency: f32, depth: f32) {
    // Update velocity
    update.vel.0 += Vec2::broadcast(data.dt.0)
        * data.inputs.move_dir
        * if update.vel.0.magnitude_squared() < BASE_HUMANOID_WATER_SPEED.powf(2.0) {
            BASE_HUMANOID_WATER_ACCEL
        } else {
            0.0
        }
        * efficiency;

    handle_orientation(data, update, if data.physics.on_ground { 9.0 } else { 2.0 });

    // Swim
    if data.inputs.swimup.is_pressed() {
        update.vel.0.z = (update.vel.0.z
            + data.dt.0 * GRAVITY * 4.0 * depth.clamped(0.0, 1.0).powf(3.0))
        .min(BASE_HUMANOID_WATER_SPEED);
    }
    // Swim
    if data.inputs.swimdown.is_pressed() {
        update.vel.0.z =
            (update.vel.0.z + data.dt.0 * GRAVITY * -3.5).min(BASE_HUMANOID_WATER_SPEED);
    }
}

/// First checks whether `primary`, `secondary` or `ability3` input is pressed,
/// then attempts to go into Equipping state, otherwise Idle
pub fn handle_wield(data: &JoinData, update: &mut StateUpdate) {
    if data.inputs.primary.is_pressed()
        || data.inputs.secondary.is_pressed()
        || data.inputs.ability3.is_pressed()
    {
        attempt_wield(data, update);
    }
}

/// If a tool is equipped, goes into Equipping state, otherwise goes to Idle
pub fn attempt_wield(data: &JoinData, update: &mut StateUpdate) {
    if let Some(ItemKind::Tool(tool)) = data.loadout.active_item.as_ref().map(|i| &i.item.kind) {
        update.character = CharacterState::Equipping(equipping::Data {
            time_left: tool.equip_time(),
        });
    } else {
        update.character = CharacterState::Idle;
    };
}

/// Checks that player can `Sit` and updates `CharacterState` if so
pub fn attempt_sit(data: &JoinData, update: &mut StateUpdate) {
    if data.physics.on_ground {
        update.character = CharacterState::Sit;
    }
}

pub fn attempt_dance(data: &JoinData, update: &mut StateUpdate) {
    if data.physics.on_ground && data.body.is_humanoid() {
        update.character = CharacterState::Dance;
    }
}

pub fn attempt_sneak(data: &JoinData, update: &mut StateUpdate) {
    if data.physics.on_ground && data.body.is_humanoid() {
        update.character = CharacterState::Sneak;
    }
}

/// Checks that player can `Climb` and updates `CharacterState` if so
pub fn handle_climb(data: &JoinData, update: &mut StateUpdate) {
    if data.inputs.climb.is_some()
        && data.physics.on_wall.is_some()
        && !data.physics.on_ground
        //&& update.vel.0.z < 0.0
        && data.body.is_humanoid()
        && update.energy.current() > 100
    {
        update.character = CharacterState::Climb;
    }
}

/// Checks that player can Swap Weapons and updates `Loadout` if so
pub fn attempt_swap_loadout(data: &JoinData, update: &mut StateUpdate) {
    if data.loadout.second_item.is_some() {
        update.swap_loadout = true;
    }
}

/// Checks that player can wield the glider and updates `CharacterState` if so
pub fn attempt_glide_wield(data: &JoinData, update: &mut StateUpdate) {
    if data.physics.on_ground
        && !data
            .physics
            .in_fluid
            .map(|depth| depth > 1.0)
            .unwrap_or(false)
        && data.body.is_humanoid()
    {
        update.character = CharacterState::GlideWield;
    }
}

/// Checks that player can jump and sends jump event if so
pub fn handle_jump(data: &JoinData, update: &mut StateUpdate) {
    if data.inputs.jump.is_pressed()
        && data.physics.on_ground
        && !data
            .physics
            .in_fluid
            .map(|depth| depth > 1.0)
            .unwrap_or(false)
    {
        update
            .local_events
            .push_front(LocalEvent::Jump(data.entity));
    }
}

/// Will attempt to go into `loadout.active_item.ability1`
pub fn handle_ability1_input(data: &JoinData, update: &mut StateUpdate) {
    if data.inputs.primary.is_pressed() {
        if let Some(ability) = data
            .loadout
            .active_item
            .as_ref()
            .and_then(|i| i.ability1.as_ref())
            .filter(|ability| ability.requirements_paid(data, update))
        {
            update.character = ability.into();
        }
    }
}

/// Will attempt to go into `loadout.active_item.ability2`
pub fn handle_ability2_input(data: &JoinData, update: &mut StateUpdate) {
    if data.inputs.secondary.is_pressed() {
        let active_tool_kind = match data.loadout.active_item.as_ref().map(|i| &i.item.kind) {
            Some(ItemKind::Tool(Tool { kind, .. })) => Some(kind),
            _ => None,
        };

        let second_tool_kind = match data.loadout.second_item.as_ref().map(|i| &i.item.kind) {
            Some(ItemKind::Tool(Tool { kind, .. })) => Some(kind),
            _ => None,
        };

        match (
            active_tool_kind.map(|tk| tk.hands()),
            second_tool_kind.map(|tk| tk.hands()),
        ) {
            (Some(Hands::TwoHand), _) => {
                if let Some(ability) = data
                    .loadout
                    .active_item
                    .as_ref()
                    .and_then(|i| i.ability2.as_ref())
                    .filter(|ability| ability.requirements_paid(data, update))
                {
                    update.character = ability.into();
                }
            },
            (_, Some(Hands::OneHand)) => {
                if let Some(ability) = data
                    .loadout
                    .second_item
                    .as_ref()
                    .and_then(|i| i.ability2.as_ref())
                    .filter(|ability| ability.requirements_paid(data, update))
                {
                    update.character = ability.into();
                }
            },
            (_, _) => {},
        };
    }
}

/// Will attempt to go into `loadout.active_item.ability3`
pub fn handle_ability3_input(data: &JoinData, update: &mut StateUpdate) {
    if data.inputs.ability3.is_pressed() {
        if let Some(ability) = data
            .loadout
            .active_item
            .as_ref()
            .and_then(|i| i.ability3.as_ref())
            .filter(|ability| ability.requirements_paid(data, update))
        {
            update.character = ability.into();
        }
    }
}

/// Checks that player can perform a dodge, then
/// attempts to go into `loadout.active_item.dodge_ability`
pub fn handle_dodge_input(data: &JoinData, update: &mut StateUpdate) {
    if data.inputs.roll.is_pressed() {
        if let Some(ability) = data
            .loadout
            .active_item
            .as_ref()
            .and_then(|i| i.dodge_ability.as_ref())
            .filter(|ability| ability.requirements_paid(data, update))
        {
            if data.character.is_wield() {
                update.character = ability.into();
                if let CharacterState::Roll(roll) = &mut update.character {
                    roll.was_wielded = true;
                }
            } else {
                update.character = ability.into();
            }
        }
    }
}

pub fn unwrap_tool_data<'a>(data: &'a JoinData) -> Option<&'a Tool> {
    if let Some(ItemKind::Tool(tool)) = data.loadout.active_item.as_ref().map(|i| &i.item.kind) {
        Some(tool)
    } else {
        None
    }
}
