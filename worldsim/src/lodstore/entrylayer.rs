use super::index::ToOptionUsize;
use super::lodpos::LodPos;
use super::data::{HashNestLayer, DetailStore, HashIter, HashIterMut};
use super::delta::{VecNestDelta, DeltaStore, VecDeltaIter, VecDeltaIterMut, DataWriterIter, DeltaWriter};
use super::traversable::Traversable;

//TODO: actually implement EntryLayer
pub trait EntryLayer {
    type TRAV<'a>: Traversable;
    type TRAV_MUT<'a>: Traversable;
    fn trav<'a>(&'a self, pos: LodPos) -> Self::TRAV;
    fn trav_mut<'a>(&'a mut self, pos: LodPos) -> Self::TRAV_MUT;
}

///////////////// data types
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> HashNestLayer<C, T, I, { L }>
{
    pub fn trav<'a>(&'a self, pos: LodPos) -> HashIter<'a, HashNestLayer<C, T, I, { L }>> {
        HashIter {
            layer: self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }

    pub fn trav_mut<'a>(&'a mut self, pos: LodPos) -> HashIterMut<'a, HashNestLayer<C, T, I, { L }>> {
        HashIterMut {
            layer: self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }
}

///////////////// delta types
impl<D: DeltaStore, T, const L: u8> VecNestDelta<D, T, { L }> {
    pub fn trav<'a>(&'a self, _pos: LodPos) -> VecDeltaIter<'a, VecNestDelta<D, T, { L }>> {
        VecDeltaIter { layer: self }
    }
    pub fn trav_mut<'a>(&'a mut self, _pos: LodPos) -> VecDeltaIterMut<'a, VecNestDelta<D, T, { L }>> {
        VecDeltaIterMut { layer: self }
    }
}

impl<D: DeltaStore, C: DetailStore, T, I: ToOptionUsize, const L: u8> DeltaWriter<'_, HashNestLayer<C, T, I, { L }>, VecNestDelta<D, T, { L }>>
{
    pub fn trav<'a>(&'a self, pos: LodPos) -> DataWriterIter<VecDeltaIter<'a, VecNestDelta<D, T, { L }>>, HashIter<'a, HashNestLayer<C, T, I, { L }>>> {
        DataWriterIter {
            delta_iter: self.delta.trav(pos),
            data_iter: self.data.trav(pos),
        }
    }

    pub fn trav_mut<'a>(&'a mut self, pos: LodPos) -> DataWriterIter< VecDeltaIterMut<'a, VecNestDelta<D, T, { L }>>, HashIterMut<'a, HashNestLayer<C, T, I, { L }>>> {
        DataWriterIter {
            delta_iter: self.delta.trav_mut(pos),
            data_iter: self.data.trav_mut(pos),
        }
    }
}