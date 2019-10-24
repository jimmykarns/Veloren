use super::index::ToOptionUsize;
use super::lodpos::{multily_with_2_pow_n, relative_to_1d, LodPos};
use super::data::{DetailStore, IndexStore, HashIter, VecIter, HashIterMut, VecIterMut};
use super::delta::{Delta, VecDataIter, DataWriterIter};
#[allow(unused_imports)] //not unsued, cargo is just to stupud to detect that
use super::layer::{Layer, ParentLayer};
use std::marker::PhantomData;

pub trait Traversable {
    type TRAV_CHILD;
    fn get(self) -> Self::TRAV_CHILD;
}

///////////////// data types

impl<'a, L: DetailStore<KEY = LodPos> + IndexStore> Traversable for HashIter<'a, L>
    where
        L::CHILD: DetailStore,
{
    type TRAV_CHILD = VecIter<'a, L::CHILD>;

    fn get(self) -> VecIter<'a, L::CHILD> {
        let child_lod = self.wanted.align_to_level(L::CHILD::LEVEL);
        let pos_offset = relative_to_1d(
            child_lod,
            self.layer_lod,
            L::CHILD::LEVEL,
            L::CHILDS_PER_OWN,
        );
        let layer_key = (multily_with_2_pow_n(
            IndexStore::load(self.layer, self.layer_lod).into_usize(),
            L::LOG2_OF_CHILDS_PER_OWN_TOTAL,
        )) + pos_offset;
        VecIter {
            layer: self.layer.child(),
            wanted: self.wanted,
            layer_key,
            layer_lod: child_lod,
        }
    }
}

impl<'a, L: DetailStore<KEY = LodPos> + IndexStore> Traversable for HashIterMut<'a, L>
    where
        L::CHILD: DetailStore,
{
    type TRAV_CHILD = VecIterMut<'a, L::CHILD>;

    fn get(self) -> VecIterMut<'a, L::CHILD> {
        let child_lod = self.wanted.align_to_level(L::CHILD::LEVEL);
        let pos_offset = relative_to_1d(
            child_lod,
            self.layer_lod,
            L::CHILD::LEVEL,
            L::CHILDS_PER_OWN,
        );
        let layer_key = (multily_with_2_pow_n(
            IndexStore::load(self.layer, self.layer_lod).into_usize(),
            L::LOG2_OF_CHILDS_PER_OWN_TOTAL,
        )) + pos_offset;
        VecIterMut {
            layer: self.layer.child_mut(),
            wanted: self.wanted,
            layer_key,
            layer_lod: child_lod,
        }
    }
}

impl<'a, L: DetailStore<KEY = usize> + IndexStore> Traversable for VecIter<'a, L>
    where
        L::CHILD: DetailStore,
{
    type TRAV_CHILD = VecIter<'a, L::CHILD>;

    fn get(self) -> VecIter<'a, L::CHILD> {
        let child_lod = self.wanted.align_to_level(L::CHILD::LEVEL);
        let pos_offset = relative_to_1d(
            child_lod,
            self.layer_lod,
            L::CHILD::LEVEL,
            L::CHILDS_PER_OWN,
        );
        let layer_key = (multily_with_2_pow_n(
            IndexStore::load(self.layer, self.layer_key).into_usize(),
            L::LOG2_OF_CHILDS_PER_OWN_TOTAL,
        )) + pos_offset;
        VecIter {
            layer: self.layer.child(),
            wanted: self.wanted,
            layer_key,
            layer_lod: child_lod,
        }
    }
}

impl<'a, L: DetailStore<KEY = usize> + IndexStore> Traversable for VecIterMut<'a, L>
    where
        L::CHILD: DetailStore,
{
    type TRAV_CHILD = VecIterMut<'a, L::CHILD>;

    fn get(self) -> VecIterMut<'a, L::CHILD> {
        let child_lod = self.wanted.align_to_level(L::CHILD::LEVEL);
        let pos_offset = relative_to_1d(
            child_lod,
            self.layer_lod,
            L::CHILD::LEVEL,
            L::CHILDS_PER_OWN,
        );
        let layer_key = (multily_with_2_pow_n(
            IndexStore::load(self.layer, self.layer_key).into_usize(),
            L::LOG2_OF_CHILDS_PER_OWN_TOTAL,
        )) + pos_offset;
        VecIterMut {
            layer: self.layer.child_mut(),
            wanted: self.wanted,
            layer_key,
            layer_lod: child_lod,
        }
    }
}

///////////////// delta types

impl<'a, D: Delta + ParentLayer> Traversable for VecDataIter<'a, D>
    where
        D::CHILD: Delta,
{
    type TRAV_CHILD = VecDataIter<'a, D::CHILD>;

    fn get(self) -> VecDataIter<'a, D::CHILD> {
        VecDataIter {
            layer: self.layer.child(),
        }
    }
}

impl<'a, DT: Traversable, CT: Traversable> Traversable for DataWriterIter<'a, DT, CT> {
    type TRAV_CHILD = DataWriterIter<'a, DT::TRAV_CHILD, CT::TRAV_CHILD>;

    fn get(self) -> DataWriterIter<'a, DT::TRAV_CHILD, CT::TRAV_CHILD> {
        DataWriterIter {
            delta_iter: self.delta_iter.get(),
            data_iter: self.data_iter.get(),
            _a: PhantomData::<&'a ()>::default(),
        }
    }
}