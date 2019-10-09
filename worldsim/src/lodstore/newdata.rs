use std::u32;
use std::collections::HashMap;
use vek::*;
use super::index::{
    self,
    LodIndex,
    AbsIndex,
    relative_to_1d,
    two_pow_u,
};
use super::area::LodArea;
use super::delta::LodDelta;

/*
 Terminology:
 - Layer: the layer of the LoDTree, a tree can have n layers, every layer contains their child layer, except for the last one.
          Each layer contains a level, a number from 15-0. the level of each child must be lower than the parents layer!
 - Detail: Each Layer contains information about that layer, here called Detail. This is the usable information we want to store in the LoDTree
           Each detail has a position. Multiple Details can exist at the same position on different layers!
 - Index: this is a bit ambiguous yet, it either means a value from type LodIndex, a LodIndex always marks a specific position in the LoDTree(but not layer)
          or this refers to the actually storage for the index for the next layer (often a u16,u32)
 //TODO: define the LodIndex as LoDPosition
 - Key: always refers to the storage of a LAYER. Any keyword with KEY is either of type usize or LodIndex.

 traits:
 - IndexStore: Every layer must implement this for either KEY=usize or KEY=LodIndex and INDEX is often u16/u32. depending on the store of the parent detail.
               It is accessed by parent layer to store the index when a detail is added or removed.
 - DetailStore: Every layer must implement this for either KEY=usize or KEY=LodIndex, independent from the parent.
                This is used to store the actual detail of every layer.
 - Nestable: All layers, except the lowest one implement this trait. It links the below layer to interact with the child layer.
 !!Calculations will be implemented on these 3 Stores, rather than the actual structs to reduce duplciate coding!!
 - Traversable: trait is used to get child layer and child Index for a concrete position.
 - Materializeable: trait is used to actually return a Detail for a concrete position.

 Actual structs regarding of position in the chain. They represent the Layers and contain the Details, they implement (some of) the 3 Store traits
 Naming Scheme is <Own Detail Type><Parent Detail Type>[Nest]Layer
 - VecVecLayer/VecHashLayer: Vec Leaf Layers that have a vec/hash index and dont have a child layer.
 - VecVecNestLayer/VecHashNestLayer: Vec Layers that have a vec/hash index and are middle layers
 - HashNoneNestLayer: Hash Layer that has no index and must be parent layer

 Result Structs:
 - LayerResult: Is used to access a layer meta information or Detail via LoDTree.traverse().get().get().get().mat().
                When LoDTree.traverse() returns a LayerResult.
*/

//K: Key is either usize or LodIndex
//I: Index stored, often u16 or u32
pub trait IndexStore {
    type KEY;
    type INDEX: Copy;

    fn load(&mut self, key: Self::KEY) -> Self::INDEX;
    fn store(&mut self, key: Self::KEY, index: Self::INDEX);
}
pub trait DetailStore {
    type KEY;
    type DETAIL;

    fn load(&mut self, key: Self::KEY) -> &Self::DETAIL;
    fn load_mut(&mut self, key: Self::KEY) -> &mut Self::DETAIL;
    fn store(&mut self, key: Self::KEY, detail: Self::DETAIL);
}

pub trait Nestable {
    type NESTED: IndexStore + DetailStore;

    fn nested(&self) -> &Self::NESTED;
}

pub trait Traversable<C> {
    fn get(&self) -> C;
}
pub trait Materializeable<T> {
    fn mat(&self) -> T;
}

//struct LayerResult<'a, N: IndexStore<PK, I> + DetailStore<K, CT>, PK, I: Copy, K, CT> {
pub struct LayerResult<'a, N: IndexStore + DetailStore, PK> {
    child: &'a N,
    wanted: LodIndex,
    index: PK,
}

//#######################################################

// Name <Own detail><Parent Index>
pub struct VecVecLayer<T, PI: Copy, const L: u8> {
    pub detail: Vec<T>,
    pub index: Vec<PI>,
}
pub struct VecHashLayer<T, PI: Copy, const L: u8> {
    pub detail: Vec<T>,
    pub index: HashMap<LodIndex, PI>,
}

//K: Child detail storage type usize or LodIndex
//T: own detail type
//PI: parents index type u16, u32
//CT: Child detail type
//I: own index type u16, u32
//pub struct VecVecNestLayer<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> {
pub struct VecVecNestLayer<N: IndexStore + DetailStore, T, PI: Copy, const L: u8> {
    pub detail: Vec<T>,
    pub index: Vec<PI>,
    pub nested: N,
}

//pub struct VecHashNestLayer<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> {
pub struct VecHashNestLayer<N: IndexStore + DetailStore, T, PI: Copy, const L: u8> {
    pub detail: Vec<T>,
    pub index: HashMap<LodIndex, PI>,
    pub nested: N,
}

//pub struct HashNoneNestLayer<N: IndexStore<LodIndex, I> + DetailStore<K, CT>, K, T, CT, I: Copy, const L: u8> {
pub struct HashNoneNestLayer<N: IndexStore + DetailStore, T, const L: u8> {
    pub detail: HashMap<LodIndex, T>,
    pub nested: N,
}

#[rustfmt::skip]
//impl<T, I: Copy, const L: u8> IndexStore<usize, I> for VecVecLayer<T, I, {L}> {
impl<T, PI: Copy, const L: u8> IndexStore for VecVecLayer<T, PI, {L}> {
    type KEY=usize; type INDEX=PI;
    fn load(&mut self, key: usize) -> PI {  *self.index.get(key).unwrap() }
    fn store(&mut self, key: usize, index: PI) { self.index.insert(key, index); }
}
#[rustfmt::skip]
//impl<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> IndexStore<usize, PI> for VecVecNestLayer<N, K, T, PI, CT, I, {L}>  {
impl<N: IndexStore<KEY=usize> + DetailStore, T, PI: Copy, const L: u8> IndexStore for VecVecNestLayer<N, T, PI, {L}> {
    type KEY=usize; type INDEX=PI;
    fn load(&mut self, key: usize) -> PI { *self.index.get(key).unwrap() }
    fn store(&mut self, key: usize, index: PI) { self.index.insert(key, index); }
}
#[rustfmt::skip]
//impl<T, I: Copy, const L: u8> IndexStore<LodIndex, I> for VecHashLayer<T, I, {L}> {
impl<T, PI: Copy, const L: u8> IndexStore for VecHashLayer<T, PI, {L}> {
    type KEY=LodIndex; type INDEX=PI;
    fn load(&mut self, key: LodIndex) -> PI { *self.index.get(&key).unwrap() }
    fn store(&mut self, key: LodIndex, index: PI) { self.index.insert(key, index); }
}
#[rustfmt::skip]
//impl<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> IndexStore<LodIndex, PI> for VecHashNestLayer<N, K, T, PI, CT, I, {L}>  {
impl<N: IndexStore<KEY=usize> + DetailStore, T, PI: Copy, const L: u8> IndexStore for VecHashNestLayer<N, T, PI, {L}>  {
    type KEY=LodIndex; type INDEX=PI;
    fn load(&mut self, key: LodIndex) -> PI { *self.index.get(&key).unwrap() }
    fn store(&mut self, key: LodIndex, index: PI) { self.index.insert(key, index); }
}

#[rustfmt::skip]
//impl<T, I: Copy, const L: u8> DetailStore<usize, T> for VecVecLayer<T, I, {L}> {
impl<T, PI: Copy, const L: u8> DetailStore for VecVecLayer<T, PI, {L}> {
    type KEY=usize; type DETAIL=T;
    fn load(&mut self, key: usize) -> &T {  self.detail.get(key).unwrap() }
    fn load_mut(&mut self, key: usize) -> &mut T {  self.detail.get_mut(key).unwrap() }
    fn store(&mut self, key: usize, detail: T) { self.detail.insert(key, detail); }
}
#[rustfmt::skip]
//impl<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> DetailStore<usize, T> for VecVecNestLayer<N, K, T, PI, CT, I, {L}>  {
impl<N: IndexStore<KEY=usize> + DetailStore, T, PI: Copy, const L: u8> DetailStore for VecVecNestLayer<N, T, PI, {L}>  {
    type KEY=usize; type DETAIL=T;
    fn load(&mut self, key: usize) -> &T { self.detail.get(key).unwrap() }
    fn load_mut(&mut self, key: usize) -> &mut T {  self.detail.get_mut(key).unwrap() }
    fn store(&mut self, key: usize, detail: T) { self.detail.insert(key, detail); }
}
#[rustfmt::skip]
//impl<T, I: Copy, const L: u8> DetailStore<usize, T> for VecHashLayer<T, I, {L}> {
impl<T, PI: Copy, const L: u8> DetailStore for VecHashLayer<T, PI, {L}> {
    type KEY=usize; type DETAIL=T;
    fn load(&mut self, key: usize) -> &T { self.detail.get(key).unwrap() }
    fn load_mut(&mut self, key: usize) -> &mut T {  self.detail.get_mut(key).unwrap() }
    fn store(&mut self, key: usize, detail: T) { self.detail.insert(key, detail); }
}
#[rustfmt::skip]
//impl<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> DetailStore<usize, T> for VecHashNestLayer<N, K, T, PI, CT, I, {L}>  {
impl<N: IndexStore<KEY=usize> + DetailStore, T, PI: Copy, const L: u8> DetailStore for VecHashNestLayer<N, T, PI, {L}>  {
    type KEY=usize; type DETAIL=T;
    fn load(&mut self, key: usize) -> &T { self.detail.get(key).unwrap() }
    fn load_mut(&mut self, key: usize) -> &mut T {  self.detail.get_mut(key).unwrap() }
    fn store(&mut self, key: usize, detail: T) { self.detail.insert(key, detail); }
}
#[rustfmt::skip]
//impl<N: IndexStore<LodIndex, I> + DetailStore<K, CT>, K, T, CT, I: Copy, const L: u8> DetailStore<LodIndex, T> for HashNoneNestLayer<N, K, T, CT, I, {L}>  {
impl<N: IndexStore<KEY=LodIndex> + DetailStore, T, const L: u8> DetailStore for HashNoneNestLayer<N, T, {L}>  {
    type KEY=LodIndex; type DETAIL=T;
    fn load(&mut self, key: LodIndex) -> &T { self.detail.get(&key).unwrap() }
    fn load_mut(&mut self, key: LodIndex) -> &mut T {  self.detail.get_mut(&key).unwrap() }
    fn store(&mut self, key: LodIndex, detail: T) { self.detail.insert(key, detail); }
}


#[rustfmt::skip]
//impl<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> Nestable<N, usize, I, K, CT> for VecVecNestLayer<N, K, T, PI, CT, I, {L}>  {
impl<N: IndexStore<KEY=usize> + DetailStore, T, PI: Copy, const L: u8> Nestable for VecVecNestLayer<N, T, PI, {L}>  {
    type NESTED=N;
    fn nested(&self) -> &N { &self.nested }
}
#[rustfmt::skip]
//impl<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> Nestable<N, usize, I, K, CT> for VecHashNestLayer<N, K, T, PI, CT, I, {L}>  {
impl<N: IndexStore<KEY=usize> + DetailStore, T, PI: Copy, const L: u8> Nestable for VecHashNestLayer<N, T, PI, {L}>  {
    type NESTED=N;
    fn nested(&self) -> &N { &self.nested }
}
#[rustfmt::skip]
//impl<N: IndexStore<LodIndex, I> + DetailStore<K, CT>, K, T, CT, I: Copy, const L: u8> Nestable<N, LodIndex, I, K, CT> for HashNoneNestLayer<N, K, T, CT, I, {L}>  {
impl<N: IndexStore<KEY=LodIndex> + DetailStore, T, const L: u8> Nestable for HashNoneNestLayer<N, T, {L}>  {
    type NESTED=N;
    fn nested(&self) -> &N { &self.nested }
}

//#######################################################

//impl<N: IndexStore<usize, I> + DetailStore<K, CT>, K, T, PI: Copy, CT, I: Copy, const L: u8> VecVecNestLayer<N, K, T, PI, CT, I, {L}> {
impl<N: IndexStore<KEY=LodIndex> + DetailStore, T, const L: u8> HashNoneNestLayer<N, T, {L}> {
    // fn get<'a>(&'a self, index: LodIndex) -> LayerResult<'a, N, usize, I, K, CT> {
    pub fn trav<'a>(&'a self, index: LodIndex) -> LayerResult<'a, N, usize> {
        LayerResult{
            child: &self.nested,
            wanted: index,
            index: 0,
        }
    }
}

impl<'a, N: IndexStore + DetailStore + Nestable, PK> Traversable<LayerResult<'a, N::NESTED, <N as IndexStore>::KEY>> for LayerResult<'a, N, PK> {
    fn get(&self) -> LayerResult<'a, N::NESTED, <N as IndexStore>::KEY> {
        unimplemented!();
    }
}

impl<'a, N: IndexStore + DetailStore, PK> Materializeable<N::DETAIL> for LayerResult<'a, N, PK> {
    fn mat(&self) -> N::DETAIL {
        unimplemented!();
    }
}

//pub struct HashNoneNestLayer<N: IndexStore<LodIndex, I> + DetailStore<K, CT>, K, T, CT, I: Copy, const L: u8> {
pub type ExampleDelta =
    HashNoneNestLayer<
        VecHashNestLayer<
            VecVecNestLayer<
                VecVecLayer<
                    (), u16, 0
                > ,(), u32, 4
            > ,Option<()> , u16, 9
        > ,() , 13
    >;

#[cfg(test)]
mod tests {
    use crate::lodstore::newdata::*;

    #[test]
    fn newdata() {
        let x = ExampleDelta {
            detail: HashMap::new(),
            nested: VecHashNestLayer {
                detail: Vec::new(),
                index: HashMap::new(),
                nested: VecVecNestLayer {
                    detail: Vec::new(),
                    index: Vec::new(),
                    nested: VecVecLayer {
                        detail: Vec::new(),
                        index: Vec::new(),
                    }
                }
            }
        };
        let i = LodIndex::new(Vec3::new(0,1,2));
        let y = x.trav(i);
        let ttc = y.get().get();
        let tt = ttc.mat();
    }
}

// TODO: instead of storing the absolute index in index, we store (index / number of entities), which means a u16 in Block can not only hold 2 full Subblocks (32^3 subblocks per block). but the full 2^16-1 ones.