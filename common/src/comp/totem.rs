use crate::sync::Uid;
use serde::{Deserialize, Serialize};
use specs::{Component, Entity, FlaggedStorage};
use specs_idvs::IdvStorage;

/// A component that stores a reference to a totem entity
/// along with its type, used to give characteristics to totems
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TotemKind {
    Generic,
    Thunder,
}
#[derive(Debug, PartialEq)]
pub struct Totem {
    pub entity: Entity,
    pub kind: TotemKind,
}

impl Component for Totem {
    type Storage = FlaggedStorage<Self, IdvStorage<Self>>;
}
