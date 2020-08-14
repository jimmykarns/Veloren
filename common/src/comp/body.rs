pub mod biped_large;
pub mod bird_medium;
pub mod bird_small;
pub mod critter;
pub mod dragon;
pub mod fish_medium;
pub mod fish_small;
pub mod golem;
pub mod humanoid;
pub mod object;
pub mod quadruped_low;
pub mod quadruped_medium;
pub mod quadruped_small;

use crate::{
    assets::{self, Asset},
    npc::NpcKind,
};
use serde::{Deserialize, Serialize};
use specs::{Component, FlaggedStorage};
use specs_idvs::IdvStorage;
use std::{fs::File, io::BufReader};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum Body {
    Humanoid(humanoid::Body) = 0,
    QuadrupedSmall(quadruped_small::Body) = 1,
    QuadrupedMedium(quadruped_medium::Body) = 2,
    BirdMedium(bird_medium::Body) = 3,
    FishMedium(fish_medium::Body) = 4,
    Dragon(dragon::Body) = 5,
    BirdSmall(bird_small::Body) = 6,
    FishSmall(fish_small::Body) = 7,
    BipedLarge(biped_large::Body) = 8,
    Object(object::Body) = 9,
    Golem(golem::Body) = 10,
    Critter(critter::Body) = 11,
    QuadrupedLow(quadruped_low::Body) = 12,
}

/// Data representing data generic to the body together with per-species data.
///
/// NOTE: Deliberately don't (yet?) implement serialize.
#[derive(Clone, Debug, Deserialize)]
pub struct BodyData<BodyMeta, SpeciesData> {
    /// Shared metadata for this whole body type.
    pub body: BodyMeta,
    /// All the metadata for species with this body type.
    pub species: SpeciesData,
}

/// Metadata intended to be stored per-body, together with data intended to be
/// stored for each species for each body.
///
/// NOTE: Deliberately don't (yet?) implement serialize.
#[derive(Clone, Debug, Deserialize)]
pub struct AllBodies<BodyMeta, SpeciesMeta> {
    pub humanoid: BodyData<BodyMeta, humanoid::AllSpecies<SpeciesMeta>>,
    pub quadruped_small: BodyData<BodyMeta, quadruped_small::AllSpecies<SpeciesMeta>>,
    pub quadruped_medium: BodyData<BodyMeta, quadruped_medium::AllSpecies<SpeciesMeta>>,
    pub bird_medium: BodyData<BodyMeta, bird_medium::AllSpecies<SpeciesMeta>>,
    pub biped_large: BodyData<BodyMeta, biped_large::AllSpecies<SpeciesMeta>>,
    pub golem: BodyData<BodyMeta, golem::AllSpecies<SpeciesMeta>>,
    pub critter: BodyData<BodyMeta, critter::AllSpecies<SpeciesMeta>>,
    pub dragon: BodyData<BodyMeta, dragon::AllSpecies<SpeciesMeta>>,
    pub quadruped_low: BodyData<BodyMeta, quadruped_low::AllSpecies<SpeciesMeta>>,
}

/// Can only retrieve body metadata by direct index.
impl<BodyMeta, SpeciesMeta> core::ops::Index<NpcKind> for AllBodies<BodyMeta, SpeciesMeta> {
    type Output = BodyMeta;

    #[inline]
    fn index(&self, index: NpcKind) -> &Self::Output {
        match index {
            NpcKind::Humanoid => &self.humanoid.body,
            NpcKind::Pig => &self.quadruped_small.body,
            NpcKind::Wolf => &self.quadruped_medium.body,
            NpcKind::Duck => &self.bird_medium.body,
            NpcKind::Ogre => &self.biped_large.body,
            NpcKind::StoneGolem => &self.golem.body,
            NpcKind::Rat => &self.critter.body,
            NpcKind::Reddragon => &self.dragon.body,
            NpcKind::Crocodile => &self.quadruped_low.body,
        }
    }
}

impl<
    BodyMeta: Send + Sync + for<'de> serde::Deserialize<'de>,
    SpeciesMeta: Send + Sync + for<'de> serde::Deserialize<'de>,
> Asset for AllBodies<BodyMeta, SpeciesMeta>
{
    const ENDINGS: &'static [&'static str] = &["json"];

    fn parse(buf_reader: BufReader<File>) -> Result<Self, assets::Error> {
        serde_json::de::from_reader(buf_reader).map_err(assets::Error::parse_error)
    }
}

impl Body {
    pub fn is_humanoid(&self) -> bool {
        match self {
            Body::Humanoid(_) => true,
            _ => false,
        }
    }

    // Note: this might need to be refined to something more complex for realistic
    // behavior with less cylindrical bodies (e.g. wolfs)
    pub fn radius(&self) -> f32 {
        // TODO: Improve these values (some might be reliant on more info in inner type)
        match self {
            Body::Humanoid(_) => 0.2,
            Body::QuadrupedSmall(_) => 0.3,
            Body::QuadrupedMedium(_) => 0.9,
            Body::Critter(_) => 0.2,
            Body::BirdMedium(_) => 0.5,
            Body::FishMedium(_) => 0.5,
            Body::Dragon(_) => 2.5,
            Body::BirdSmall(_) => 0.2,
            Body::FishSmall(_) => 0.2,
            Body::BipedLarge(_) => 2.0,
            Body::Golem(_) => 2.5,
            Body::QuadrupedLow(_) => 1.0,
            Body::Object(_) => 0.3,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Body::Humanoid(humanoid) => match humanoid.species {
                humanoid::Species::Danari => 0.8,
                humanoid::Species::Dwarf => 0.9,
                humanoid::Species::Orc => 1.14,
                humanoid::Species::Undead => 0.95,
                _ => 1.0,
            },
            Body::QuadrupedSmall(_) => 0.6,
            Body::QuadrupedMedium(_) => 0.5,
            Body::Critter(_) => 0.4,
            Body::BirdMedium(_) => 1.2,
            Body::FishMedium(_) => 1.0,
            Body::Dragon(_) => 5.0,
            Body::BirdSmall(_) => 0.4,
            Body::FishSmall(_) => 0.4,
            Body::BipedLarge(_) => 4.0,
            Body::Golem(_) => 5.0,
            Body::QuadrupedLow(_) => 0.5,
            Body::Object(_) => 0.6,
        }
    }

    #[allow(unreachable_patterns)]
    pub fn base_health(&self) -> u32 {
        match self {
            Body::Humanoid(_) => 400,
            Body::QuadrupedSmall(quadruped_small) => match quadruped_small.species {
                quadruped_small::Species::Boar => 180,
                quadruped_small::Species::Batfox => 100,
                quadruped_small::Species::Dodarock => 320,
                quadruped_small::Species::Holladon => 250,
                quadruped_small::Species::Hyena => 150,
                quadruped_small::Species::Truffler => 180,
                _ => 80,
            },
            Body::QuadrupedMedium(quadruped_medium) => match quadruped_medium.species {
                quadruped_medium::Species::Grolgar => 300,
                quadruped_medium::Species::Saber => 200,
                quadruped_medium::Species::Tiger => 200,
                quadruped_medium::Species::Tuskram => 300,
                quadruped_medium::Species::Lion => 400,
                quadruped_medium::Species::Tarasque => 600,
                quadruped_medium::Species::Wolf => 200,
                quadruped_medium::Species::Frostfang => 400,
                quadruped_medium::Species::Mouflon => 300,
                quadruped_medium::Species::Catoblepas => 500,
                quadruped_medium::Species::Bonerattler => 300,
                _ => 200,
            },
            Body::BirdMedium(bird_medium) => match bird_medium.species {
                bird_medium::Species::Chicken => 50,
                bird_medium::Species::Duck => 50,
                bird_medium::Species::Goose => 60,
                bird_medium::Species::Parrot => 60,
                bird_medium::Species::Peacock => 60,
                bird_medium::Species::Cockatrice => 110,
                bird_medium::Species::Eagle => 110,
                _ => 100,
            },
            Body::FishMedium(_) => 50,
            Body::Dragon(dragon) => match dragon.species {
                _ => 5000,
            },
            Body::BirdSmall(_) => 50,
            Body::FishSmall(_) => 20,
            Body::BipedLarge(biped_large) => match biped_large.species {
                biped_large::Species::Ogre => 700,
                biped_large::Species::Cyclops => 800,
                biped_large::Species::Wendigo => 800,
                biped_large::Species::Troll => 600,
                biped_large::Species::Dullahan => 1200,
                _ => 1000,
            },
            Body::Object(_) => 10000,
            Body::Golem(golem) => match golem.species {
                _ => 1500,
            },
            Body::Critter(critter) => match critter.species {
                _ => 50,
            },
            Body::QuadrupedLow(quadruped_low) => match quadruped_low.species {
                quadruped_low::Species::Crocodile => 200,
                quadruped_low::Species::Alligator => 200,
                quadruped_low::Species::Salamander => 100,
                quadruped_low::Species::Monitor => 80,
                quadruped_low::Species::Asp => 80,
                quadruped_low::Species::Tortoise => 200,
                quadruped_low::Species::Rocksnapper => 500,
                quadruped_low::Species::Pangolin => 60,
                quadruped_low::Species::Maneater => 250,
                _ => 200,
            },
        }
    }

    #[allow(unreachable_patterns)]
    pub fn base_health_increase(&self) -> u32 {
        match self {
            Body::Humanoid(_) => 50,
            Body::QuadrupedSmall(quadruped_small) => match quadruped_small.species {
                quadruped_small::Species::Boar => 20,
                quadruped_small::Species::Batfox => 10,
                quadruped_small::Species::Dodarock => 30,
                quadruped_small::Species::Holladon => 30,
                quadruped_small::Species::Hyena => 20,
                quadruped_small::Species::Truffler => 20,
                _ => 10,
            },
            Body::QuadrupedMedium(quadruped_medium) => match quadruped_medium.species {
                quadruped_medium::Species::Grolgar => 30,
                quadruped_medium::Species::Saber => 20,
                quadruped_medium::Species::Tiger => 20,
                quadruped_medium::Species::Tuskram => 30,
                quadruped_medium::Species::Lion => 40,
                quadruped_medium::Species::Tarasque => 60,
                quadruped_medium::Species::Wolf => 20,
                quadruped_medium::Species::Frostfang => 40,
                quadruped_medium::Species::Mouflon => 30,
                quadruped_medium::Species::Catoblepas => 50,
                quadruped_medium::Species::Bonerattler => 30,
                _ => 20,
            },
            Body::BirdMedium(bird_medium) => match bird_medium.species {
                bird_medium::Species::Chicken => 10,
                bird_medium::Species::Duck => 10,
                bird_medium::Species::Goose => 10,
                bird_medium::Species::Parrot => 10,
                bird_medium::Species::Peacock => 10,
                bird_medium::Species::Cockatrice => 10,
                bird_medium::Species::Eagle => 10,
                _ => 10,
            },
            Body::FishMedium(_) => 10,
            Body::Dragon(dragon) => match dragon.species {
                _ => 500,
            },
            Body::BirdSmall(_) => 10,
            Body::FishSmall(_) => 10,
            Body::BipedLarge(biped_large) => match biped_large.species {
                biped_large::Species::Ogre => 70,
                biped_large::Species::Cyclops => 80,
                biped_large::Species::Wendigo => 80,
                biped_large::Species::Troll => 60,
                biped_large::Species::Dullahan => 120,
                _ => 100,
            },
            Body::Object(_) => 10,
            Body::Golem(golem) => match golem.species {
                _ => 150,
            },
            Body::Critter(critter) => match critter.species {
                _ => 10,
            },
            Body::QuadrupedLow(quadruped_low) => match quadruped_low.species {
                quadruped_low::Species::Crocodile => 20,
                quadruped_low::Species::Alligator => 20,
                quadruped_low::Species::Salamander => 10,
                quadruped_low::Species::Monitor => 10,
                quadruped_low::Species::Asp => 10,
                quadruped_low::Species::Tortoise => 20,
                quadruped_low::Species::Rocksnapper => 50,
                quadruped_low::Species::Pangolin => 10,
                quadruped_low::Species::Maneater => 30,
                _ => 20,
            },
        }
    }

    #[allow(unreachable_patterns)]
    pub fn base_exp(&self) -> u32 {
        match self {
            Body::Humanoid(_) => 5,
            Body::QuadrupedSmall(quadruped_small) => match quadruped_small.species {
                quadruped_small::Species::Boar => 6,
                quadruped_small::Species::Batfox => 6,
                quadruped_small::Species::Dodarock => 6,
                quadruped_small::Species::Holladon => 8,
                quadruped_small::Species::Hyena => 6,
                quadruped_small::Species::Truffler => 6,
                _ => 4,
            },
            Body::QuadrupedMedium(quadruped_medium) => match quadruped_medium.species {
                quadruped_medium::Species::Grolgar => 10,
                quadruped_medium::Species::Saber => 8,
                quadruped_medium::Species::Tiger => 8,
                quadruped_medium::Species::Tuskram => 9,
                quadruped_medium::Species::Lion => 10,
                quadruped_medium::Species::Tarasque => 16,
                quadruped_medium::Species::Wolf => 8,
                quadruped_medium::Species::Frostfang => 9,
                quadruped_medium::Species::Mouflon => 7,
                quadruped_medium::Species::Catoblepas => 10,
                quadruped_medium::Species::Bonerattler => 10,
                _ => 6,
            },
            Body::BirdMedium(bird_medium) => match bird_medium.species {
                bird_medium::Species::Chicken => 2,
                bird_medium::Species::Duck => 2,
                bird_medium::Species::Goose => 4,
                bird_medium::Species::Parrot => 4,
                bird_medium::Species::Peacock => 5,
                _ => 8,
            },
            Body::FishMedium(_) => 2,
            Body::Dragon(dragon) => match dragon.species {
                _ => 1000,
            },
            Body::BirdSmall(_) => 2,
            Body::FishSmall(_) => 2,
            Body::BipedLarge(biped_large) => match biped_large.species {
                biped_large::Species::Ogre => 60,
                biped_large::Species::Cyclops => 70,
                biped_large::Species::Wendigo => 70,
                biped_large::Species::Troll => 50,
                biped_large::Species::Dullahan => 100,
                _ => 100,
            },
            Body::Object(_) => 1,
            Body::Golem(golem) => match golem.species {
                _ => 75,
            },
            Body::Critter(critter) => match critter.species {
                _ => 2,
            },
            Body::QuadrupedLow(quadruped_low) => match quadruped_low.species {
                quadruped_low::Species::Crocodile => 10,
                quadruped_low::Species::Alligator => 10,
                quadruped_low::Species::Salamander => 6,
                quadruped_low::Species::Monitor => 4,
                quadruped_low::Species::Asp => 4,
                quadruped_low::Species::Tortoise => 6,
                quadruped_low::Species::Rocksnapper => 12,
                quadruped_low::Species::Pangolin => 3,
                quadruped_low::Species::Maneater => 14,
                _ => 10,
            },
        }
    }

    #[allow(unreachable_patterns)]
    pub fn base_exp_increase(&self) -> u32 {
        match self {
            Body::Humanoid(_) => 2,
            Body::QuadrupedSmall(quadruped_small) => match quadruped_small.species {
                _ => 1,
            },
            Body::QuadrupedMedium(quadruped_medium) => match quadruped_medium.species {
                _ => 1,
            },
            Body::BirdMedium(bird_medium) => match bird_medium.species {
                _ => 1,
            },
            Body::FishMedium(_) => 1,
            Body::Dragon(dragon) => match dragon.species {
                _ => 32,
            },
            Body::BirdSmall(_) => 1,
            Body::FishSmall(_) => 1,
            Body::BipedLarge(biped_large) => match biped_large.species {
                _ => 5,
            },
            Body::Object(_) => 0,
            Body::Golem(golem) => match golem.species {
                _ => 10,
            },
            Body::Critter(critter) => match critter.species {
                _ => 1,
            },
            Body::QuadrupedLow(quadruped_low) => match quadruped_low.species {
                _ => 1,
            },
        }
    }

    #[allow(unreachable_patterns)]
    pub fn base_dmg(&self) -> u32 {
        match self {
            Body::Humanoid(_) => 50,
            Body::QuadrupedSmall(quadruped_small) => match quadruped_small.species {
                quadruped_small::Species::Dodarock => 30,
                quadruped_small::Species::Hyena => 40,
                _ => 20,
            },
            Body::QuadrupedMedium(quadruped_medium) => match quadruped_medium.species {
                quadruped_medium::Species::Grolgar => 50,
                quadruped_medium::Species::Lion => 60,
                quadruped_medium::Species::Tarasque => 70,
                quadruped_medium::Species::Mouflon => 30,
                quadruped_medium::Species::Catoblepas => 20,
                quadruped_medium::Species::Bonerattler => 50,
                _ => 40,
            },
            Body::BirdMedium(bird_medium) => match bird_medium.species {
                bird_medium::Species::Chicken => 10,
                bird_medium::Species::Duck => 10,
                bird_medium::Species::Goose => 10,
                bird_medium::Species::Parrot => 20,
                bird_medium::Species::Peacock => 40,
                bird_medium::Species::Cockatrice => 60,
                bird_medium::Species::Eagle => 60,
                _ => 30,
            },
            Body::FishMedium(_) => 10,
            Body::Dragon(dragon) => match dragon.species {
                _ => 5000,
            },
            Body::BirdSmall(_) => 10,
            Body::FishSmall(_) => 10,
            Body::BipedLarge(biped_large) => match biped_large.species {
                biped_large::Species::Ogre => 60,
                biped_large::Species::Cyclops => 60,
                biped_large::Species::Wendigo => 60,
                biped_large::Species::Troll => 60,
                biped_large::Species::Dullahan => 80,
                _ => 60,
            },
            Body::Object(_) => 0,
            Body::Golem(golem) => match golem.species {
                _ => 250,
            },
            Body::Critter(critter) => match critter.species {
                _ => 10,
            },
            Body::QuadrupedLow(quadruped_low) => match quadruped_low.species {
                quadruped_low::Species::Crocodile => 50,
                quadruped_low::Species::Alligator => 50,
                quadruped_low::Species::Salamander => 30,
                quadruped_low::Species::Monitor => 30,
                quadruped_low::Species::Asp => 35,
                quadruped_low::Species::Tortoise => 10,
                quadruped_low::Species::Rocksnapper => 80,
                quadruped_low::Species::Pangolin => 10,
                quadruped_low::Species::Maneater => 40,
                _ => 20,
            },
        }
    }

    #[allow(unreachable_patterns)]
    pub fn base_range(&self) -> f32 {
        match self {
            Body::Humanoid(_) => 5.0,
            Body::QuadrupedSmall(quadruped_small) => match quadruped_small.species {
                _ => 4.5,
            },
            Body::QuadrupedMedium(quadruped_medium) => match quadruped_medium.species {
                _ => 5.5,
            },
            Body::BirdMedium(bird_medium) => match bird_medium.species {
                _ => 3.5,
            },
            Body::FishMedium(_) => 3.5,
            Body::Dragon(dragon) => match dragon.species {
                _ => 12.5,
            },
            Body::BirdSmall(_) => 3.0,
            Body::FishSmall(_) => 3.0,
            Body::BipedLarge(biped_large) => match biped_large.species {
                _ => 10.0,
            },
            Body::Object(_) => 3.0,
            Body::Golem(golem) => match golem.species {
                _ => 7.5,
            },
            Body::Critter(critter) => match critter.species {
                _ => 3.0,
            },
            Body::QuadrupedLow(quadruped_low) => match quadruped_low.species {
                _ => 4.5,
            },
        }
    }
}

impl Component for Body {
    type Storage = FlaggedStorage<Self, IdvStorage<Self>>;
}
