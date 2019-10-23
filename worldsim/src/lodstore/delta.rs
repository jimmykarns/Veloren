use super::data::{DetailStore, EntryLayer, Layer, Materializeable, ParentLayer, Traversable};
use super::lodpos::LodPos;
use std::marker::PhantomData;
/*
    A LodDelta applies a change to a Lod
    The rules for LodDeltas are strict in order to make them as simple as possible.
    A LodDelta created from LodData A can only be applied safely to another LodData equal to A.
    However LodDeltas can be combined and reverted

    I am not sure about a Vec or Hashmap, the thing is Vec is easier to fill, but might contain duplicate entries:
    E.g. change a item multiple time, bloats the Delta, with a Hashmap only the lastest state is kept.
    However i belive that most algorithms only change every Value once.
*/

/*
The Idea is to create a DeltaWriter that has a mutable Detla and Data and implements the Data interaces.
While it borrows a mutable reference to both it locks both with rusts system
When the writing is done, we destroy the Writer but keep the Delta and Data.
The DeltaWriter will output its own iterator,
We only need a traversable trait
*/
pub trait Delta: Layer {}

#[derive(Default, Clone)]
pub struct VecDelta<T, const L: u8> {
    pub detail: Vec<(LodPos, Option<T>)>,
}
#[derive(Default, Clone)]
pub struct VecNestDelta<D: Delta, T, const L: u8> {
    pub detail: Vec<(LodPos, Option<T>)>,
    pub child: D,
}

pub struct DeltaWriter<'a, C: EntryLayer<'a> + DetailStore, D: EntryLayer<'a> + Delta> {
    pub delta: &'a mut D,
    pub data: &'a mut C,
}

pub struct VecDataIter<'a, D: Delta> {
    layer: &'a D,
}

pub struct DataWriterIter<'a, DT: 'a, CT: 'a> {
    delta_iter: DT,
    data_iter: CT,
    _a: PhantomData<&'a ()>,
}

//#######################################################

impl<T, const L: u8> Layer for VecDelta<T, { L }> {
    type KEY = (usize);
    const LEVEL: u8 = { L };
}
impl<D: Delta, T, const L: u8> Layer for VecNestDelta<D, T, { L }> {
    type KEY = (usize);
    const LEVEL: u8 = { L };
}

impl<D: Delta, T, const L: u8> ParentLayer for VecNestDelta<D, T, { L }> {
    type CHILD = D;
    fn child(&self) -> &Self::CHILD {
        &self.child
    }
}

impl<'a, C: DetailStore + EntryLayer<'a>, D: Delta + EntryLayer<'a>> DeltaWriter<'a, C, D> {
    pub fn new(delta: &'a mut D, data: &'a mut C) -> Self {
        DeltaWriter { delta, data }
    }
}

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

impl<'a, DT, CT: Materializeable> Materializeable for DataWriterIter<'a, DT, CT> {
    type MAT_CHILD = CT::MAT_CHILD;

    fn mat(self) -> CT::MAT_CHILD {
        self.data_iter.mat()
    }
}

impl<T, const L: u8> Delta for VecDelta<T, { L }> {}
impl<C: Delta, T, const L: u8> Delta for VecNestDelta<C, T, { L }> {}

//#######################################################

#[cfg(test)]
mod tests {
    use crate::lodstore::data::tests::gen_simple_example;
    use crate::lodstore::data::tests::ExampleData;
    use crate::lodstore::data::*;
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
            let w = DeltaWriter::new(&mut d, &mut x);
            let i = LodPos::xyz(0, 1, 2);
            if false {
                let y = w.trav(i);
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
            let w = DeltaWriter::new(&mut d, &mut x);
            let i = LodPos::xyz(0, 0, 0);
            assert_eq!(*w.trav(i).get().get().get().mat(), 7_i8);
        }
    }

    #[bench]
    fn bench_access_trav(b: &mut Bencher) {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        {
            let w = DeltaWriter::new(&mut d, &mut x);
            let access = LodPos::xyz(0, 0, 0);
            b.iter(|| w.trav(access));
        }
    }

    #[bench]
    fn bench_access_3(b: &mut Bencher) {
        let mut x = gen_simple_example();
        let mut d = ExampleDelta::default();
        {
            let w = DeltaWriter::new(&mut d, &mut x);
            let access = LodPos::xyz(0, 0, 0);
            b.iter(|| w.trav(access).get().get().get().mat());
        }
    }
}
