use super::index::ToOptionUsize;
use super::lodpos::{two_pow_u, LodPos};
use super::data::{VecLayer, HashLayer, VecNestLayer, HashNestLayer, DetailStore};
use super::delta::{VecDelta, VecNestDelta, Delta};
use vek::Vec3;

pub trait Layer {
    type KEY;
    const LEVEL: u8;
}

pub trait ParentLayer: Layer {
    type CHILD: Layer;
    fn child(&self) -> &Self::CHILD;
    fn child_mut(&mut self) -> &mut Self::CHILD;
    const CHILDS_PER_OWN_TOTAL: usize = two_pow_u(Self::LOG2_OF_CHILDS_PER_OWN_TOTAL) as usize;
    const LOG2_OF_CHILDS_PER_OWN_TOTAL: u8 = 3 * ({ Self::LEVEL } - Self::CHILD::LEVEL);
    const CHILDS_PER_OWN: Vec3<u32> = Vec3 {
        x: two_pow_u(Self::LEVEL - Self::CHILD::LEVEL) as u32,
        y: two_pow_u(Self::LEVEL - Self::CHILD::LEVEL) as u32,
        z: two_pow_u(Self::LEVEL - Self::CHILD::LEVEL) as u32,
    };
}

///////////////// data types

impl<T, const L: u8> Layer for VecLayer<T, { L }> {
    type KEY = (usize);
    const LEVEL: u8 = { L };
}
impl<T, const L: u8> Layer for HashLayer<T, { L }> {
    type KEY = (LodPos);
    const LEVEL: u8 = { L };
}
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> Layer for VecNestLayer<C, T, I, { L }> {
    type KEY = (usize);
    const LEVEL: u8 = { L };
}
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> Layer for HashNestLayer<C, T, I, { L }> {
    type KEY = (LodPos);
    const LEVEL: u8 = { L };
}

impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> ParentLayer
    for VecNestLayer<C, T, I, { L }>
{
    type CHILD = C;
    fn child(&self) -> &Self::CHILD {
        &self.child
    }
    fn child_mut(&mut self) -> &mut Self::CHILD {
        &mut self.child
    }
}
impl<C: DetailStore, T, I: ToOptionUsize, const L: u8> ParentLayer
    for HashNestLayer<C, T, I, { L }>
{
    type CHILD = C;
    fn child(&self) -> &Self::CHILD {
        &self.child
    }
    fn child_mut(&mut self) -> &mut Self::CHILD {
        &mut self.child
    }
}

///////////////// delta types

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
    fn child_mut(&mut self) -> &mut Self::CHILD {
        &mut self.child
    }
}