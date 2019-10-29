use super::lodpos::{LodPos};
use super::delta::{DeltaStore, VecDeltaIterMut};

pub trait Deltalizeable {
    type DELTA: DeltaStore;
    fn store(self, pos: LodPos, value: Option<<Self::DELTA as DeltaStore>::DETAIL>);
}

///////////////// delta types

impl<'a, D: DeltaStore> Deltalizeable for VecDeltaIterMut<'a, D> {
    type DELTA = D;
    fn store(self, pos: LodPos, value: Option<D::DETAIL>) {
        self.layer.store(pos, value);
    }
}