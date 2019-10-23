use super::data::{DetailStore};
use super::lodpos::LodPos;
use super::layer::{Layer};
#[allow(unused_imports)] //not unsued, cargo is just to stupud to detect that
use super::entrylayer::EntryLayer;
#[allow(unused_imports)]
use super::traversable::Traversable;
#[allow(unused_imports)]
use super::materializeable::Materializeable;
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
    pub( super ) layer: &'a D,
}

pub struct DataWriterIter<'a, DT: 'a, CT: 'a> {
    pub( super ) delta_iter: DT,
    pub( super ) data_iter: CT,
    pub( super ) _a: PhantomData<&'a ()>,
}

//#######################################################

impl<'a, C: DetailStore + EntryLayer<'a>, D: Delta + EntryLayer<'a>> DeltaWriter<'a, C, D> {
    pub fn new(delta: &'a mut D, data: &'a mut C) -> Self {
        DeltaWriter { delta, data }
    }
}

impl<T, const L: u8> Delta for VecDelta<T, { L }> {}
impl<C: Delta, T, const L: u8> Delta for VecNestDelta<C, T, { L }> {}

//#######################################################

#[cfg(test)]
mod tests {
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
