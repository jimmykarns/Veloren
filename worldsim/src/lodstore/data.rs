use super::area::LodArea;
//use super::delta::LodDelta;
use super::lodpos::{self, multily_with_2_pow_n, relative_to_1d, two_pow_u32, AbsIndex, LodPos};
use super::index::ToOptionUsize;
use fxhash::FxHashMap;
use std::collections::HashMap;
use std::{u16, u32};
use vek::*;

/*
 Terminology:
 - Layer: the layer of the LoDTree, a tree can have n layers, every layer contains their child layer, except for the last one.
          Each layer contains a level, a number from 15-0. the level of each child must be lower than the parents layer!
 - Detail: Each Layer contains information about that layer, here called Detail. This is the usable information we want to store in the LoDTree
 - LodPos: A LodPos marks a specific position inside the LoDTree, but not their layer.
           Each Detail has a LodPos. Multiple Details can exist at the same LodPos on different layers!
 - Index: This refers to the actually storage for the index for the next layer (often a u16,u32).
          The Index is used to find the child in a spare storage.
 - Key: always refers to the storage of a LAYER. Any keyword with KEY is either of type usize or LodPos.

 traits:
 - Layer: Every layer must implement this. KEY is the storage Type and either usize/LodPos. Layer is also defined here.
 - ParentLayer: Is a Layer that contains a CHILD layer and some const functions based on their const properties
 - IndexStore: Every layer must implement this for their Layer::KEY and INDEX is often u16/u32.
               The index is accessed by this layer to get the corresponding child.
               Every Indexstore is a ParentLayer.
 - DetailStore: Every layer must implement this for their KEY.
                This is used to store the actual DETAIL of every layer.
 !!Calculations will be implemented on these 2 Stores, rather than the actual structs to reduce duplciate coding!!
 - ToOptionUsize: to store INDEX in z16/u32 efficiently and move up to usize on calculation
 - Traversable: trait is used to get child layer and child Index for a concrete position.
 - Materializeable: trait is used to actually return a Detail for a concrete position.

 Actual structs regarding of position in the chain. They represent the Layers and contain the Details, they implement (some of) the 2 Store traits
 Naming Scheme is <Own Detail Type>[Nest]Layer
 - VecLayer: KEY=usize, stores in Vec, leaf layer
 - HashLayer:KEY=LodPos, stores in Vec, leaf layer
 - VecNestLayer: KEY=usize, stores in Vec, has childs
 - HashNestLayer: KEY=LodPos, stores in Vec, has childs

 Result Structs:
 - HashIter/VecIter: Is used to access a layer meta information or Detail via LoDTree.trav().get().get().get().mat().
                     When LoDTree.trav() returns a HashIter.
                     It keeps information to next layer to not recalculate it
*/

pub trait Layer {
    type KEY;
    const LEVEL: u8;
}

pub trait ParentLayer: Layer {
    type CHILD: Layer;
    fn child(&self) -> &Self::CHILD;
    fn CHILDS_PER_OWN_TOTAL() -> usize {
        two_pow_u32(Self::LOG2_OF_CHILDS_PER_OWN_TOTAL()) as usize
    }
    fn LOG2_OF_CHILDS_PER_OWN_TOTAL() -> u8 {
        3 * ({ Self::LEVEL } - Self::CHILD::LEVEL)
    }
    fn CHILDS_PER_OWN() -> Vec3<u32> {
        Vec3 {
            x: two_pow_u32(Self::LEVEL - Self::CHILD::LEVEL) as u32,
            y: two_pow_u32(Self::LEVEL - Self::CHILD::LEVEL) as u32,
            z: two_pow_u32(Self::LEVEL - Self::CHILD::LEVEL) as u32,
        }
    }
}

pub trait IndexStore: ParentLayer {
    type INDEX: ToOptionUsize;
    fn load(&self, key: Self::KEY) -> Self::INDEX;
}

pub trait DetailStore: Layer {
    type DETAIL;
    fn load(&self, key: Self::KEY) -> &Self::DETAIL;
}

pub trait Traversable<C> {
    fn get(self) -> C;
}
pub trait Materializeable<T> {
    fn mat(self) -> T;
}

//#######################################################

#[derive(Default, Clone, Debug)]
pub struct VecLayer<T, const L: u8> {
    pub detail: Vec<T>,
}
#[derive(Default, Clone, Debug)]
pub struct HashLayer<T, const L: u8> {
    pub detail: FxHashMap<LodPos, T>,
}
#[derive(Default, Clone, Debug)]
pub struct VecNestLayer<C: DetailStore, T, I: ToOptionUsize, const L: u8> {
    pub detail: Vec<T>,
    pub index: Vec<I>,
    pub child: C,
}
#[derive(Default, Clone, Debug)]
pub struct HashNestLayer<C: DetailStore, T, I: ToOptionUsize, const L: u8> {
    pub detail_index: FxHashMap<LodPos, (T, I)>,
    pub child: C,
}

pub struct HashIter<'a, C: DetailStore> {
    layer: &'a C,
    wanted: LodPos,
    layer_lod: LodPos, //LodPos aligned to layer::LEVEL
}
pub struct VecIter<'a, C: DetailStore> {
    layer: &'a C,
    wanted: LodPos,
    layer_lod: LodPos, //LodPos aligned to layer::LEVEL
    layer_key: usize,
}

#[rustfmt::skip]
impl<T, const L: u8> Layer for VecLayer<T, { L }> {
    type KEY = ( usize ); const LEVEL: u8 = { L };
}
#[rustfmt::skip]
impl<T, const L: u8> Layer for HashLayer<T, { L }> {
    type KEY = ( LodPos ); const LEVEL: u8 = { L };
}
#[rustfmt::skip]
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> Layer for VecNestLayer<C, T, I, { L }> {
    type KEY = ( usize ); const LEVEL: u8 = { L };
}
#[rustfmt::skip]
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> Layer for HashNestLayer<C, T, I, { L }> {
    type KEY = ( LodPos ); const LEVEL: u8 = { L };
}

#[rustfmt::skip]
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> ParentLayer for VecNestLayer<C, T, I, { L }> {
    type CHILD = C;
    fn child(&self) -> &Self::CHILD { &self.child }
}
#[rustfmt::skip]
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> ParentLayer for HashNestLayer<C, T, I, { L }> {
    type CHILD = C;
    fn child(&self) -> &Self::CHILD { &self.child }
}

impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> HashNestLayer<C, T, I, { L }> {
    fn trav(&self, pos: LodPos) -> HashIter<Self> {
        HashIter {
            layer: &self,
            wanted: pos,
            layer_lod: pos.align_to_level({ L }),
        }
    }
}

#[rustfmt::skip]
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> IndexStore for VecNestLayer<C, T, I, { L }> {
    type INDEX = I;
    fn load(&self, key: Self::KEY) -> Self::INDEX { self.index[key] }
}
#[rustfmt::skip]
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> IndexStore for HashNestLayer<C, T, I, { L }> {
    type INDEX = I;
    fn load(&self, key: Self::KEY) -> Self::INDEX {
        debug_assert_eq!(key, key.align_to_level({ L }));
        self.detail_index[&key].1
    }
}

#[rustfmt::skip]
impl<C: DetailStore, I: ToOptionUsize, T, const L: u8> DetailStore for VecNestLayer<C, T, I, { L }> {
    type DETAIL = T;
    fn load(&self, key: Self::KEY) -> &Self::DETAIL {
        &self.detail[key]
    }
}
#[rustfmt::skip]
impl<C: DetailStore, I: ToOptionUsize, T, const L: u8> DetailStore for HashNestLayer<C, T, I, { L }> {
    type DETAIL = T;
    fn load(&self, key: LodPos) -> &Self::DETAIL {
        debug_assert_eq!(key, key.align_to_level({ L }));
        &self.detail_index[&key].0
    }
}
#[rustfmt::skip]
impl<T, const L: u8> DetailStore for VecLayer<T, { L }> {
    type DETAIL = T;
    fn load(&self, key: usize) -> &Self::DETAIL {
        &self.detail[key]
    }
}
#[rustfmt::skip]
impl<T, const L: u8> DetailStore for HashLayer<T, { L }> {
    type DETAIL = T;
    fn load(&self, key: LodPos) -> &Self::DETAIL {
        debug_assert_eq!(key, key.align_to_level({ L }));
        &self.detail[&key]
    }
}

impl<'a, L: DetailStore<KEY = LodPos> + IndexStore> Traversable<VecIter<'a, L::CHILD>>
for HashIter<'a, L>
    where
        L::CHILD: DetailStore, {
    fn get(self) -> VecIter<'a, L::CHILD> {
        let child_lod = self.wanted.align_to_level(L::CHILD::LEVEL );
        let pos_offset = relative_to_1d(child_lod, self.layer_lod, L::CHILD::LEVEL, L::CHILDS_PER_OWN());
        let layer_key = ( multily_with_2_pow_n( IndexStore::load(self.layer, self.layer_lod).into_usize(), L::LOG2_OF_CHILDS_PER_OWN_TOTAL()) ) + pos_offset;
        VecIter {
            layer: self.layer.child(),
            wanted: self.wanted,
            layer_key,
            layer_lod: child_lod,
        }
    }
}

impl<'a, L: DetailStore<KEY = usize> + IndexStore> Traversable<VecIter<'a, L::CHILD>>
for VecIter<'a, L>
    where
        L::CHILD: DetailStore, {
    fn get(self) -> VecIter<'a, L::CHILD> {
        let child_lod = self.wanted.align_to_level(L::CHILD::LEVEL );
        let pos_offset = relative_to_1d(child_lod, self.layer_lod, L::CHILD::LEVEL, L::CHILDS_PER_OWN());
        let layer_key = ( multily_with_2_pow_n( IndexStore::load(self.layer, self.layer_key).into_usize(), L::LOG2_OF_CHILDS_PER_OWN_TOTAL()) ) + pos_offset;
        VecIter {
            layer: self.layer.child(),
            wanted: self.wanted,
            layer_key,
            layer_lod: child_lod,
        }
    }
}

impl<'a, L: DetailStore<KEY=LodPos>> Materializeable<&'a L::DETAIL> for HashIter<'a, L> {
    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_lod)
    }
}

impl<'a, L: DetailStore<KEY=usize>> Materializeable<&'a L::DETAIL> for VecIter<'a, L> {
    fn mat(self) -> &'a L::DETAIL {
        DetailStore::load(self.layer, self.layer_key)
    }
}

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

#[cfg(test)]
mod tests {
    use crate::lodstore::data::*;
    use test::Bencher;

    fn gen_simple_example() -> ExampleData {
        let mut detail_index = FxHashMap::default();
        detail_index.insert(LodPos::xyz(0, 0, 0), ((), 0));
        ExampleData {
            detail_index,
            child: VecNestLayer {
                detail: vec!((),(),()),
                index: vec!(0,1,u32::MAX),
                child: VecNestLayer {
                    detail: vec!(None,None,None,Some(()),Some(()),None,None,None,None,None,None,None,None,None,None,None),
                    index: vec!(0,u16::MAX,u16::MAX,0,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX,u16::MAX),
                    child: VecLayer {
                        detail: vec!(7,6,5,4,3,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
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
            let tt = ttc.mat();
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
}