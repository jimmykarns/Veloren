use super::data::{DetailStore, Layer, ParentLayer, Traversable};
use super::index::ToOptionUsize;
use super::lodpos::{multily_with_2_pow_n, relative_to_1d, two_pow_u32, LodPos};
use serde::export::PhantomData;
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

pub struct DeltaWriter<'a, C: DetailStore, D: Delta> {
    pub delta: &'a mut D,
    pub data: &'a mut C,
}

struct VecDataIter<'a, D: Delta> {
    layer: &'a D,
}

struct DataWriterIter<DT: Traversable, CT: Traversable> {
    delta_iter: DT,
    data_iter: CT,
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

impl<'a, C: DetailStore, D: Delta> DeltaWriter<'a, C, D> {
    pub fn new(delta: &'a mut D, data: &'a mut C) -> Self {
        DeltaWriter { delta, data }
    }
}

impl<'a, C: DetailStore, D: Delta> DeltaWriter<'a, C, D> {
    #[allow(dead_code)]
    fn trav(&'a self, pos: LodPos) -> VecDataIter<'a, D> {
        VecDataIter { layer: &self.delta }
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

impl<'a, DT: Traversable, CT: Traversable> Traversable for DataWriterIter<DT, CT>
where
    DT::TRAV_CHILD: Traversable,
    CT::TRAV_CHILD: Traversable,
{
    type TRAV_CHILD = DataWriterIter<DT::TRAV_CHILD, CT::TRAV_CHILD>;

    fn get(self) -> DataWriterIter<DT::TRAV_CHILD, CT::TRAV_CHILD> {
        DataWriterIter {
            delta_iter: self.delta_iter.get(),
            data_iter: self.data_iter.get(),
        }
    }
}

impl<T, const L: u8> Delta for VecDelta<T, { L }> {}
impl<C: Delta, T, const L: u8> Delta for VecNestDelta<C, T, { L }> {}

//#######################################################

#[cfg(test)]
mod tests {
    use crate::lodstore::data::*;
    use crate::lodstore::delta::*;
    use test::Bencher;

    #[rustfmt::skip]
    pub type ExampleData =
    HashNestLayer<
        VecNestLayer<
            VecNestLayer<
                VecLayer<
                    i8, 0
                > ,Option<()>, u16, 2
            > ,() , u32, 3
        > ,() ,u16, 4
    >;

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

            if false {}
        }
    }

    #[test]
    fn access_first_element() {
        let x = ExampleDelta::default();
        let i = LodPos::xyz(0, 0, 0);
    }
}
