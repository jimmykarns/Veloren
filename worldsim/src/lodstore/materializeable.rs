use super::lodpos::{LodPos};
use super::data::{DetailStore, HashIter, VecIter};
use super::delta::DataWriterIter;

pub trait Materializeable {
    type MAT_CHILD;
    fn mat(self) -> Self::MAT_CHILD;
}

///////////////// data types

impl<'a, L: DetailStore<KEY = LodPos>> Materializeable for HashIter<'a, L> {
    type MAT_CHILD = &'a L::DETAIL;

    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_lod)
    }
}

impl<'a, L: DetailStore<KEY = usize>> Materializeable for VecIter<'a, L> {
    type MAT_CHILD = &'a L::DETAIL;

    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_key)
    }
}

///////////////// delta types

impl<'a, DT, CT: Materializeable> Materializeable for DataWriterIter<'a, DT, CT> {
    type MAT_CHILD = CT::MAT_CHILD;

    fn mat(self) -> CT::MAT_CHILD {
        self.data_iter.mat()
    }
}
