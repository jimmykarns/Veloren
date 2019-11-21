use super::data::{DetailStore, HashIter, HashIterMut, IndexStore, VecIter, VecIterMut};
use super::delta::{DataWriterIter, DeltaStore, VecDeltaIter, VecDeltaIterMut};
use super::index::ToOptionUsize;
#[allow(unused_imports)] //not unsued, cargo is just to stupud to detect that
use super::layer::{Layer, ParentLayer};
use super::lodpos::{multily_with_2_pow_n, relative_to_1d, LodPos};

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

impl<'a, D: DeltaStore + ParentLayer> Traversable for VecDeltaIter<'a, D>
where
    D::CHILD: DeltaStore,
{
    type TRAV_CHILD = VecDeltaIter<'a, D::CHILD>;

    fn get(self) -> VecDeltaIter<'a, D::CHILD> {
        VecDeltaIter {
            layer: self.layer.child(),
        }
    }
}

impl<'a, D: DeltaStore + ParentLayer> Traversable for VecDeltaIterMut<'a, D>
where
    D::CHILD: DeltaStore,
{
    type TRAV_CHILD = VecDeltaIterMut<'a, D::CHILD>;

    fn get(self) -> VecDeltaIterMut<'a, D::CHILD> {
        VecDeltaIterMut {
            layer: self.layer.child_mut(),
        }
    }
}

impl<CT: Traversable, DT: Traversable> Traversable for DataWriterIter<CT, DT> {
    type TRAV_CHILD = DataWriterIter<CT::TRAV_CHILD, DT::TRAV_CHILD>;

    fn get(self) -> DataWriterIter<CT::TRAV_CHILD, DT::TRAV_CHILD> {
        DataWriterIter {
            data_iter: self.data_iter.get(),
            delta_iter: self.delta_iter.get(),
        }
    }
}
