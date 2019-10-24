use super::index::ToOptionUsize;
use super::lodpos::LodPos;
use super::data::{HashNestLayer, DetailStore, HashIter, HashIterMut};
use super::delta::{VecNestDelta, Delta, VecDataIter, DataWriterIter, DeltaWriter};
use super::traversable::Traversable;
use std::marker::PhantomData;

pub trait EntryLayer<'a> {
    type TRAV: Traversable;
    type TRAV_MUT: Traversable;
    fn trav(&'a self, pos: LodPos) -> Self::TRAV;
    fn trav_mut(&'a mut self, pos: LodPos) -> Self::TRAV_MUT;
}

///////////////// data types

impl<'a, C: 'a + DetailStore, T: 'a, I: 'a + ToOptionUsize, const L: u8> EntryLayer<'a>
for HashNestLayer<C, T, I, { L }>
{
    type TRAV = HashIter<'a, HashNestLayer<C, T, I, { L }>>;
    type TRAV_MUT = HashIterMut<'a, HashNestLayer<C, T, I, { L }>>;

    //ERROR make the HashIter C: remove the &'a from HashIter coding and implement it here
    fn trav(&'a self, pos: LodPos) -> Self::TRAV {
        HashIter {
            layer: self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }

    fn trav_mut(&'a mut self, pos: LodPos) -> Self::TRAV_MUT {
        HashIterMut {
            layer: self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }
}

///////////////// delta types

impl<'a, D: 'a + Delta, T: 'a, const L: u8> EntryLayer<'a> for VecNestDelta<D, T, { L }> {
    type TRAV = VecDataIter<'a, VecNestDelta<D, T, { L }>>;
    type TRAV_MUT = VecDataIter<'a, VecNestDelta<D, T, { L }>>;

    fn trav(&'a self, _pos: LodPos) -> Self::TRAV {
        VecDataIter { layer: self }
    }
    fn trav_mut(&'a mut self, _pos: LodPos) -> Self::TRAV_MUT {
        VecDataIter { layer: self }
    }
}

impl<'a, C: DetailStore + EntryLayer<'a>, D: Delta + EntryLayer<'a>> EntryLayer<'a>
for DeltaWriter<'a, C, D>
    where
        <<C as EntryLayer<'a>>::TRAV as Traversable>::TRAV_CHILD: Traversable,
        <<D as EntryLayer<'a>>::TRAV as Traversable>::TRAV_CHILD: Traversable,
{
    type TRAV = DataWriterIter<'a, D::TRAV, C::TRAV>;
    type TRAV_MUT = DataWriterIter<'a, D::TRAV, C::TRAV>;

    fn trav(&'a self, pos: LodPos) -> DataWriterIter<D::TRAV, C::TRAV> {
        DataWriterIter {
            delta_iter: self.delta.trav(pos),
            data_iter: self.data.trav(pos),
            _a: PhantomData::<&'a ()>::default(),
        }
    }

    fn trav_mut(&'a mut self, pos: LodPos) -> DataWriterIter<D::TRAV, C::TRAV> {
        DataWriterIter {
            delta_iter: self.delta.trav(pos),
            data_iter: self.data.trav(pos),
            _a: PhantomData::<&'a ()>::default(),
        }
    }
}