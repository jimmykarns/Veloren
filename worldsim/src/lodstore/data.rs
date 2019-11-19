use super::index::ToOptionUsize;
use super::lodpos::LodPos;
use super::layer::{Layer, ParentLayer};
#[allow(unused_imports)] //not unsued, cargo is just to stupud to detect that
use super::traversable::Traversable;
#[allow(unused_imports)]
use super::materializeable::Materializeable;
#[allow(unused_imports)]
use super::entrylayer::EntryLayer;
use fxhash::FxHashMap;

pub trait IndexStore: ParentLayer {
    type INDEX: ToOptionUsize;
    fn load(&self, key: Self::KEY) -> Self::INDEX;
}

pub trait DetailStore: Layer {
    type DETAIL;
    fn load(&self, key: Self::KEY) -> &Self::DETAIL;
    fn save(&mut self, key: Self::KEY, detail: Self::DETAIL);
}
// TODO: There should be a Trait: to provide a independent Hash which doesnt choose even when having mut access to an element.
// then we can store e.g. an ID within the detail of every region (wich are prob 1kb, and get load_mut behavior for free
pub trait DetailStoreMut: DetailStore {
    fn load_mut(&mut self, key: Self::KEY) -> &mut Self::DETAIL;
}

//#######################################################

#[derive(Default, Clone)]
pub struct VecLayer<T, const L: u8> {
    pub detail: Vec<T>,
}
#[derive(Default, Clone)]
pub struct HashLayer<T, const L: u8> {
    pub detail: FxHashMap<LodPos, T>,
}
#[derive(Default, Clone)]
pub struct VecNestLayer<C: DetailStore, T, I: ToOptionUsize, const L: u8> {
    pub detail: Vec<T>,
    pub index: Vec<I>,
    pub child: C,
}
#[derive(Default, Clone)]
pub struct HashNestLayer<C: DetailStore, T, I: ToOptionUsize, const L: u8> {
    pub detail_index: FxHashMap<LodPos, (T, I)>,
    pub child: C,
}

pub struct HashIter<'a, C: DetailStore> {
    pub( super ) layer: &'a C,
    pub( super ) wanted: LodPos,
    pub( super ) layer_lod: LodPos, //LodPos aligned to layer::LEVEL
}
pub struct HashIterMut<'a, C: DetailStore> {
    pub( super ) layer: &'a mut C,
    pub( super ) wanted: LodPos,
    pub( super ) layer_lod: LodPos, //LodPos aligned to layer::LEVEL
}
pub struct VecIter<'a, C: DetailStore> {
    pub( super ) layer: &'a C,
    pub( super ) wanted: LodPos,
    pub( super ) layer_lod: LodPos, //LodPos aligned to layer::LEVEL
    pub( super ) layer_key: usize,
}
pub struct VecIterMut<'a, C: DetailStore> {
    pub( super ) layer: &'a mut C,
    pub( super ) wanted: LodPos,
    pub( super ) layer_lod: LodPos, //LodPos aligned to layer::LEVEL
    pub( super ) layer_key: usize,
}

impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> IndexStore for VecNestLayer<C, T, I, { L }> {
    type INDEX = I;
    fn load(&self, key: Self::KEY) -> Self::INDEX {
        self.index[key]
    }
}
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> IndexStore
    for HashNestLayer<C, T, I, { L }>
{
    type INDEX = I;
    fn load(&self, key: Self::KEY) -> Self::INDEX {
        debug_assert_eq!(key, key.align_to_level({ L }));
        self.detail_index[&key].1
    }
}
impl<C: DetailStore, I: ToOptionUsize, T, const L: u8> DetailStore
    for VecNestLayer<C, T, I, { L }>
{
    type DETAIL = T;
    fn load(&self, key: Self::KEY) -> &Self::DETAIL {
        &self.detail[key]
    }
    fn save(&mut self, key: Self::KEY, detail: Self::DETAIL) {
        self.detail.insert(key, detail);
    }
}
impl<C: DetailStore, I: ToOptionUsize, T, const L: u8> DetailStoreMut
    for VecNestLayer<C, T, I, { L }>
{
    fn load_mut(&mut self, key: Self::KEY) -> &mut Self::DETAIL {
        &mut self.detail[key]
    }
}
impl<C: DetailStore, I: ToOptionUsize, T, const L: u8> DetailStore
    for HashNestLayer<C, T, I, { L }>
{
    type DETAIL = T;
    fn load(&self, key: LodPos) -> &Self::DETAIL {
        debug_assert_eq!(key, key.align_to_level({ L }));
        &self.detail_index[&key].0
    }
    fn save(&mut self, key: LodPos, detail: Self::DETAIL) {
        debug_assert_eq!(key, key.align_to_level({ L }));
        self.detail_index.insert(key, (detail, I::none()));
    }
}
impl<T, const L: u8> DetailStore for VecLayer<T, { L }> {
    type DETAIL = T;
    fn load(&self, key: usize) -> &Self::DETAIL {
        &self.detail[key]
    }
    fn save(&mut self, key: usize, detail: Self::DETAIL) {
        self.detail[key] = detail;
    }
}
impl<T, const L: u8> DetailStoreMut for VecLayer<T, { L }> {
    fn load_mut(&mut self, key: usize) -> &mut Self::DETAIL {
        &mut self.detail[key]
    }
}
impl<T, const L: u8> DetailStore for HashLayer<T, { L }> {
    type DETAIL = T;
    fn load(&self, key: LodPos) -> &Self::DETAIL {
        debug_assert_eq!(key, key.align_to_level({ L }));
        &self.detail[&key]
    }
    fn save(&mut self, key: LodPos, detail: Self::DETAIL) {
        debug_assert_eq!(key, key.align_to_level({ L }));
        self.detail.insert(key, detail);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::lodstore::data::*;
    use test::Bencher;
    use std::{u16, u32};
    use crate::lodstore::traversable::Traversable;

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

    pub fn gen_simple_example() -> ExampleData {
        let mut detail_index = FxHashMap::default();
        detail_index.insert(LodPos::xyz(0, 0, 0), ((), 0));
        ExampleData {
            detail_index,
            child: VecNestLayer {
                detail: vec![(), (), ()],
                index: vec![0, 1, u32::MAX],
                child: VecNestLayer {
                    detail: vec![
                        None,
                        None,
                        None,
                        Some(()),
                        Some(()),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                    index: vec![
                        0,
                        u16::MAX,
                        u16::MAX,
                        0,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                        u16::MAX,
                    ],
                    child: VecLayer {
                        detail: vec![
                            7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        ],
                    },
                },
            },
        }
    }

    #[test]
    fn compilation() {
        let x = ExampleData::default();
        let i = LodPos::xyz(0, 1, 2);
        if false {
            let y = x.trav(i);
            let ttc = y.get().get().get();
            let _tt = ttc.mat();
        }
    }

    #[test]
    fn access_first_element() {
        let x = gen_simple_example();
        let i = LodPos::xyz(0, 0, 0);
        assert_eq!(*x.trav(i).get().get().get().mat(), 7_i8);
    }

    #[test]
    fn access_simple_elements() {
        let x = gen_simple_example();
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 0)).get().get().get().mat(), 7_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 1)).get().get().get().mat(), 6_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 2)).get().get().get().mat(), 5_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 3)).get().get().get().mat(), 4_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 1, 0)).get().get().get().mat(), 3_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 1, 1)).get().get().get().mat(), 2_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 1, 2)).get().get().get().mat(), 1_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 1, 3)).get().get().get().mat(), 0_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 3, 0)).get().get().get().mat(), 0_i8);
        assert_eq!(*x.trav(LodPos::xyz(1, 0, 0)).get().get().get().mat(), 0_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 2, 0)).get().get().get().mat(), 0_i8);
    }

    #[test]
    fn mut_simple_elements() {
        let mut x = gen_simple_example();
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 0)).get().get().get().mat(), 7_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 1)).get().get().get().mat(), 6_i8);
        x.trav_mut(LodPos::xyz(0, 0, 0)).get().get().get().store(123);
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 0)).get().get().get().mat(), 123_i8);
    }

    #[test]
    fn mut2_simple_elements() {
        let mut x = gen_simple_example();
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 0)).get().get().get().mat(), 7_i8);
        let c = *x.trav(LodPos::xyz(0, 0, 0)).get().get().get().mat();
        x.trav_mut(LodPos::xyz(0, 0, 0)).get().get().get().store(111 + c);
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 0)).get().get().get().mat(), 118_i8);
    }

    /* allow this once we guarante get to be consistent even on Hash Lookups!
    TODO: shuldnt this already ne the case ?
    #[test]
    fn mut3_simple_elements() {
        let mut x = gen_simple_example();
        let a = x.trav_mut(LodPos::xyz(0, 0, 0)).get().get().get();
        assert_eq!(*a.mat(), 7_i8);
        a.store(123);
        assert_eq!(*a.mat(), 123_i8);
        assert_eq!(*x.trav(LodPos::xyz(0, 0, 0)).get().get().get().mat(), 123_i8);
    }*/

    #[bench]
    fn bench_access_trav(b: &mut Bencher) {
        let x = gen_simple_example();
        let access = LodPos::xyz(0, 0, 0);
        b.iter(|| x.trav(access));
    }

    #[bench]
    fn bench_access_3(b: &mut Bencher) {
        let x = gen_simple_example();
        let access = LodPos::xyz(0, 0, 0);
        b.iter(|| x.trav(access).mat());
    }

    #[bench]
    fn bench_access_0(b: &mut Bencher) {
        let x = gen_simple_example();
        let access = LodPos::xyz(0, 0, 0);
        b.iter(|| x.trav(access).get().get().get().mat());
    }

    #[bench]
    fn bench_access_0_best_time(b: &mut Bencher) {
        let x = gen_simple_example();
        let access = LodPos::xyz(0, 0, 0);
        for _ in 0..10000 {
            //fill up the caches
            x.trav(access).get().get().get().mat();
        }
        b.iter(|| x.trav(access).get().get().get().mat());
    }

/*
    pub struct MyIterMut<'a, C> {
        pub( super ) layer: &'a mut C,
    }
    #[derive(Default, Clone)]
    pub struct Layer<C> {
        pub child: C,
    }
    pub trait EntryPoint {
        type TRAV_MUT<'a>;
        fn trav_mut<'a>(&'a mut self, pos: LodPos) -> Self::TRAV_MUT;
    }
    impl<C> EntryPoint
    for Layer<C>
    {
        type TRAV_MUT<'a> = MyIterMut<'a, Layer<C>>;

        fn trav_mut<'a>(&'a mut self, pos: u8) -> Self::TRAV_MUT {
            MyIterMut {
                layer: self,
            }
        }
    }*/
}
