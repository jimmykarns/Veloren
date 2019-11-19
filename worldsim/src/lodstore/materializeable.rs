use super::data::{DetailStore, HashIter, HashIterMut, VecIter, VecIterMut};
use super::delta::DataWriterIter;
use super::deltalizeable::Deltalizeable;
use super::lodpos::LodPos;

/*

TODO: how do we want traversable and meterializeable to work?
I e.g. should
let value = lodtree,trav().get().get().mat();
be possibe? should it create a read lock on lodtree?

ideally:
let mut v = lodtree.trav().get().get().get().mutmat()
has a read lock on lottree and a writelock on that specific field, but that wont stop us from taking another get.get.get on lodtree on the same item. mhhh arrgg, this is dificult to do statically. so better not do it ?


VLT multiple types ? for efficient returns?

*/

pub trait Materializeable<'a> {
    type MAT_CHILD;
    fn mat(self) -> &'a Self::MAT_CHILD;
    fn store(self, mat: Self::MAT_CHILD);
}

///////////////// data types

impl<'a, L: DetailStore<KEY = LodPos>> Materializeable<'a> for HashIter<'a, L> {
    type MAT_CHILD = L::DETAIL;

    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_lod)
    }
    fn store(self, _mat: L::DETAIL) {
        unimplemented!("only call on mut Iter");
        //DetailStore::save(self.layer, self.layer_key, mat)
    }
}

impl<'a, L: DetailStore<KEY = LodPos>> Materializeable<'a> for HashIterMut<'a, L> {
    type MAT_CHILD = L::DETAIL;

    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_lod)
    }
    fn store(self, mat: L::DETAIL) {
        DetailStore::save(self.layer, self.layer_lod, mat)
    }
}

impl<'a, L: DetailStore<KEY = usize>> Materializeable<'a> for VecIter<'a, L> {
    type MAT_CHILD = L::DETAIL;

    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_key)
    }
    fn store(self, _mat: L::DETAIL) {
        unimplemented!("only call on mut Iter");
        //DetailStore::save(self.layer, self.layer_key, mat)
    }
}

impl<'a, L: DetailStore<KEY = usize>> Materializeable<'a> for VecIterMut<'a, L> {
    type MAT_CHILD = L::DETAIL;

    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_key)
    }
    fn store(self, mat: L::DETAIL) {
        DetailStore::save(self.layer, self.layer_key, mat)
    }
}

///////////////// delta types

impl<'a, DT: Deltalizeable, CT: Materializeable<'a>> Materializeable<'a>
    for DataWriterIter<DT, CT>
{
    type MAT_CHILD = CT::MAT_CHILD;

    fn mat(self) -> &'a CT::MAT_CHILD {
        self.data_iter.mat()
    }
    fn store(self, mat: CT::MAT_CHILD) {
        //self.delta_iter.register(LodPos::xyz(2,2,2,), mat);

        //<DT as Deltalizeable>::DELTA::store(self.delta_iter,LodPos::xyz(2,2,2), None);
        self.delta_iter.store(LodPos::xyz(2, 2, 2), None);
        println!("saaave");
        self.data_iter.store(mat);
        //self.data_iter.store(mat)
    }
}
