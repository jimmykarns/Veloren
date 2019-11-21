use super::data::DetailStore;
#[allow(unused_imports)] //not unsued, cargo is just to stupud to detect that
use super::entrylayer::EntryLayer;
use super::layer::Layer;
use super::layer::ParentLayer;
use super::lodpos::LodPos;
#[allow(unused_imports)]
use super::materializeable::Materializeable;
#[allow(unused_imports)]
use super::traversable::Traversable;
use std::slice::Iter;
/*
    A LodDelta applies a change to a Lod
    The rules for LodDeltas are strict in order to make them as simple as possible.
    A LodDelta created from LodData A can only be applied safely to another LodData equal to A.
    However LodDeltas can be combined and reverted

    I am not sure about a Vec or Hashmap, the thing is Vec is easier to fill, but might contain duplicate entries:
    E.g. change a item multiple time, bloats the Delta, with a Hashmap only the lastest state is kept.
    However i belive that most algorithms only change every Value once.
*/

pub trait DeltaStore: Layer {
    type DETAIL;
    fn store(&mut self, pos: LodPos, value: Option<Self::DETAIL>);
    fn iter(&self) -> Iter<(LodPos, Option<Self::DETAIL>)>;
}

pub trait DeltaApplyer<C, D> {
    //pub trait DeltaApplyer<COMMONDETAIL, C: DetailStore<DETAIL=COMMONDETAIL>, D: DeltaStore<DETAIL=COMMONDETAIL>> {
    fn apply_delta(&mut self);
}

#[derive(Default, Clone)]
pub struct VecDelta<T, const L: u8> {
    pub detail: Vec<(LodPos, Option<T>)>,
}
#[derive(Default, Clone)]
pub struct VecNestDelta<D: DeltaStore, T, const L: u8> {
    pub detail: Vec<(LodPos, Option<T>)>,
    pub child: D,
}

pub struct DeltaWriter<'a, COMMONDETAIL, C: DetailStore<DETAIL=COMMONDETAIL>, D: DeltaStore<DETAIL=COMMONDETAIL>> {
    pub delta: &'a mut D,
    pub data: &'a mut C,
}

pub struct VecDeltaIter<'a, D: DeltaStore> {
    pub(super) layer: &'a D,
}

pub struct VecDeltaIterMut<'a, D: DeltaStore> {
    pub(super) layer: &'a mut D,
}

pub struct DataWriterIter<CT, DT> {
    pub(super) data_iter: CT,
    pub(super) delta_iter: DT,
}

//#######################################################

ERROR, we want DeltaApply on delta and mut data, the Iterator is the wrong place because its only for 1 pos. the DeltaWriter is part. wrong because we only want non mut delta

impl<'a, COMMONDETAIL, C: DetailStore<DETAIL=COMMONDETAIL>, D: DeltaStore<DETAIL=COMMONDETAIL>> DeltaWriter<'a, COMMONDETAIL, C, D> {
    pub fn new(delta: &'a mut D, data: &'a mut C) -> Self {
        DeltaWriter { delta, data }
    }
}

pub fn apply_delta<COMMONDETAIL, C: DetailStore<DETAIL=COMMONDETAIL>, D: DeltaStore<DETAIL=COMMONDETAIL>>(data: &mut C, delta: &D) {

}

impl<C, D> DeltaApplyer<C, D> for DataWriterIter<C,D> {
    default fn apply_delta(&mut self) {

    }
}

impl<COMMONDETAIL, C: DetailStore<DETAIL=COMMONDETAIL>, D: DeltaStore<DETAIL=COMMONDETAIL>> DeltaApplyer<C, D> for DataWriterIter<C,D>
    where
        C: ParentLayer<CHILD: DetailStore>,
        D: ParentLayer<CHILD: DeltaStore>,
{
    fn apply_delta(&mut self) {

    }
}


impl<T, const L: u8> DeltaStore for VecDelta<T, { L }> {
    type DETAIL = T;
    fn store(&mut self, pos: LodPos, value: Option<Self::DETAIL>) {
        self.detail.push((pos, value));
    }
    fn iter(&self) -> Iter<(LodPos, Option<Self::DETAIL>)> {
        self.detail.iter()
    }
}
impl<C: DeltaStore, T, const L: u8> DeltaStore for VecNestDelta<C, T, { L }> {
    type DETAIL = T;
    fn store(&mut self, pos: LodPos, value: Option<Self::DETAIL>) {
        self.detail.push((pos, value));
    }
    fn iter(&self) -> Iter<(LodPos, Option<Self::DETAIL>)> {
        self.detail.iter()
    }
}

//#######################################################

#[cfg(test)]
mod stests {
    use crate::lodstore::data::tests::gen_simple_example;
    use crate::lodstore::data::tests::ExampleData;
    use crate::lodstore::delta::*;
    use test::Bencher;

    #[rustfmt::skip]
    pub type ExampleDelta =
    VecNestDelta<
        VecNestDelta<
            VecNestDelta<
                VecDelta<
                    i8, 0
                >, Option<()>, 2
            >, (), 3
        >, (), 4
    >;

    #[test]
    fn compilation() {
        let mut x = ExampleData::default();
        let mut d = ExampleDelta::default();
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            let i = LodPos::xyz(0, 1, 2);
            if false {
                let y = w.trav_mut(i);
                let ttc = y.get().get().get();
                let _tt = ttc.mat();
            }
        }
    }

    #[test]
    fn access_first_element() {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            let i = LodPos::xyz(0, 0, 0);
            assert_eq!(*w.trav_mut(i).get().get().get().mat(), 7_i8);
        }
    }

    #[test]
    fn mut_first_element() {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        //assert_eq!(x.detail_index.len(),1);
        assert_eq!(d.detail.len(), 0);
        assert_eq!(d.child.detail.len(), 0);
        assert_eq!(d.child.child.detail.len(), 0);
        assert_eq!(d.child.child.child.detail.len(), 0);
        let i = LodPos::xyz(0, 0, 0);
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            assert_eq!(*w.trav_mut(i).get().get().get().mat(), 7_i8);
            w.trav_mut(i).get().get().get().store(123);
            assert_eq!(*w.trav_mut(i).get().get().get().mat(), 123_i8);
            assert_eq!(x.detail_index.len(), 1);
            assert_eq!(d.detail.len(), 0);
            assert_eq!(d.child.detail.len(), 0);
            assert_eq!(d.child.child.detail.len(), 0);
            assert_eq!(d.child.child.child.detail.len(), 1);
        }
    }

    #[test]
    fn mut_multiple_elements() {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        let i = LodPos::xyz(0, 0, 0);
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            assert_eq!(*w.trav_mut(i).get().get().get().mat(), 7_i8);
            w.trav_mut(i).get().get().get().store(123);
            w.trav_mut(LodPos::xyz(0, 0, 1))
                .get()
                .get()
                .get()
                .store(111);
            w.trav_mut(LodPos::xyz(0, 0, 2))
                .get()
                .get()
                .get()
                .store(112);
            w.trav_mut(LodPos::xyz(0, 0, 3))
                .get()
                .get()
                .get()
                .store(111);
            let i = LodPos::xyz(0, 0, 0);
            assert_eq!(*w.trav_mut(i).get().get().get().mat(), 123_i8);
            assert_eq!(x.detail_index.len(), 1);
            assert_eq!(d.child.detail.len(), 0);
            assert_eq!(d.child.child.detail.len(), 0);
            assert_eq!(d.child.child.child.detail.len(), 3);
        }
    }

    #[test]
    fn apply() {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        let i = LodPos::xyz(2, 2, 2);
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            w.trav_mut(i).get().get().get().store(123);
            assert_eq!(d.child.child.child.detail.len(), 1);
        }
        let mut x = gen_simple_example();
        //assert_ne!(w.trav(i).get().get().get().mat(), 123);
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            w.trav_mut().apply_delta()
        }
    }

    #[bench]
    fn bench_access_trav(b: &mut Bencher) {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            b.iter(|| {
                w.trav_mut(LodPos::xyz(0, 0, 0));
            });
        }
    }

    #[bench]
    fn bench_access_3(b: &mut Bencher) {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            b.iter(|| {
                w.trav_mut(LodPos::xyz(0, 0, 0)).get().get().get().mat();
            });
        }
    }

    #[bench]
    fn bench_iter_3(b: &mut Bencher) {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            b.iter(|| {
                w.trav_mut(LodPos::xyz(0, 0, 0)).get().get().get();
            });
        }
    }

    #[bench]
    fn bench_trav(b: &mut Bencher) {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        {
            let mut w = DeltaWriter::new(&mut d, &mut x);
            b.iter(|| {
                w.trav_mut(LodPos::xyz(0, 0, 0));
            });
        }
    }
}
