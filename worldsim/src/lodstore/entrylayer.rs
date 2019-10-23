use super::index::ToOptionUsize;
use super::lodpos::LodPos;
use super::data::{HashNestLayer, DetailStore, HashIter};
use super::delta::{VecNestDelta, Delta, VecDataIter, DataWriterIter, DeltaWriter};
use super::traversable::Traversable;
use std::marker::PhantomData;

pub trait EntryLayer<'a> {
    type TRAV: Traversable;
    fn trav(&'a self, pos: LodPos) -> Self::TRAV;
}

///////////////// data types

impl<'a, C: 'a + DetailStore, T: 'a, I: 'a + ToOptionUsize, const L: u8> EntryLayer<'a>
for HashNestLayer<C, T, I, { L }>
{
    type TRAV = HashIter<'a, HashNestLayer<C, T, I, { L }>>;

    fn trav(&'a self, pos: LodPos) -> Self::TRAV {
        HashIter {
            layer: &self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }
}

///////////////// delta types

impl<'a, D: 'a + Delta, T: 'a, const L: u8> EntryLayer<'a> for VecNestDelta<D, T, { L }> {
    type TRAV = VecDataIter<'a, VecNestDelta<D, T, { L }>>;

    fn trav(&'a self, _pos: LodPos) -> Self::TRAV {
        VecDataIter { layer: &self }
    }
}

impl<'a, C: DetailStore + EntryLayer<'a>, D: Delta + EntryLayer<'a>> EntryLayer<'a>
for DeltaWriter<'a, C, D>
    where
        <<C as EntryLayer<'a>>::TRAV as Traversable>::TRAV_CHILD: Traversable,
        <<D as EntryLayer<'a>>::TRAV as Traversable>::TRAV_CHILD: Traversable,
{
    type TRAV = DataWriterIter<'a, D::TRAV, C::TRAV>;
    fn trav(&'a self, pos: LodPos) -> DataWriterIter<D::TRAV, C::TRAV> {
        DataWriterIter {
            delta_iter: self.delta.trav(pos),
            data_iter: self.data.trav(pos),
            _a: PhantomData::<&'a ()>::default(),
        }
    }
}