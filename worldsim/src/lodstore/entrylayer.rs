use super::index::ToOptionUsize;
use super::lodpos::LodPos;
use super::data::{HashNestLayer, DetailStore, HashIter, HashIterMut};
use super::delta::{VecNestDelta, DeltaStore, VecDeltaIter, VecDeltaIterMut, DataWriterIter, DeltaWriter};
use super::traversable::Traversable;

pub trait EntryLayer {
    type TRAV<'a>: Traversable;
    type TRAV_MUT<'a>: Traversable;
    fn trav<'a>(&'a self, pos: LodPos) -> Self::TRAV;
    fn trav_mut<'a>(&'a mut self, pos: LodPos) -> Self::TRAV_MUT;
}

///////////////// data types

impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> EntryLayer
for HashNestLayer<C, T, I, { L }>
{
    type TRAV<'a> = HashIter<'a, HashNestLayer<C, T, I, { L }>>;
    type TRAV_MUT<'a> = HashIterMut<'a, HashNestLayer<C, T, I, { L }>>;

    //ERROR make the HashIter C: remove the &'a from HashIter coding and implement it here
    fn trav<'a>(&'a self, pos: LodPos) -> HashIter<'a, HashNestLayer<C, T, I, { L }>> {
        HashIter {
            layer: self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }

    fn trav_mut<'a>(&'a mut self, pos: LodPos) -> Self::TRAV_MUT {
        HashIterMut {
            layer: self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }
}

///////////////// delta types

impl<D: DeltaStore, T, const L: u8> EntryLayer for VecNestDelta<D, T, { L }> {
    type TRAV<'a> = VecDeltaIter<'a, VecNestDelta<D, T, { L }>>;
    type TRAV_MUT<'a> = VecDeltaIterMut<'a, VecNestDelta<D, T, { L }>>;

    fn trav<'a>(&'a self, _pos: LodPos) -> Self::TRAV {
        VecDeltaIter { layer: self }
    }
    fn trav_mut<'a>(&'a mut self, _pos: LodPos) -> Self::TRAV_MUT {
        VecDeltaIterMut { layer: self }
    }
}

impl<C: DetailStore + EntryLayer, D: DeltaStore + EntryLayer> EntryLayer
for DeltaWriter<'_, C, D>
    where
        <<C as EntryLayer>::TRAV as Traversable>::TRAV_CHILD: Traversable,
        <<D as EntryLayer>::TRAV as Traversable>::TRAV_CHILD: Traversable,
{
    type TRAV<'a> = DataWriterIter<D::TRAV, C::TRAV>;
    type TRAV_MUT<'a> = DataWriterIter<D::TRAV_MUT, C::TRAV_MUT>;

    fn trav<'a>(&'a self, pos: LodPos) -> Self::TRAV {
        DataWriterIter {
            delta_iter: self.delta.trav(pos),
            data_iter: self.data.trav(pos),
        }
    }

    fn trav_mut<'a>(&'a mut self, pos: LodPos) -> Self::TRAV_MUT {
        DataWriterIter {
            delta_iter: self.delta.trav_mut(pos),
            data_iter: self.data.trav_mut(pos),
        }
    }
}

impl<'a, C: DetailStore + EntryLayer, D: DeltaStore + EntryLayer> DeltaWriter<'a, C, D>
    where
        <<C as EntryLayer>::TRAV as Traversable>::TRAV_CHILD: Traversable,
        <<D as EntryLayer>::TRAV as Traversable>::TRAV_CHILD: Traversable,
{
    pub fn trav_mut_xxx(&mut self, pos: LodPos) -> DataWriterIter<D::TRAV_MUT, C::TRAV_MUT> {
        DataWriterIter {
            delta_iter: self.delta.trav_mut(pos),
            data_iter: self.data.trav_mut(pos),
        }
    }
}