use crate::lodstore::{
    HashNestLayer,
    VecNestLayer,
    HashLayer,
    VecLayer,
    lodpos::LodPos,
    lodpos::AbsIndex,
};
use vek::*;
use std::u32;
pub type LodIndex = LodPos;

#[derive(Debug, Clone, Default)]
pub struct Region {
    precent_air: f32,
    percent_forrest: f32,
    percent_lava: f32,
    percent_water: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    precent_air: f32,
    percent_forrest: f32,
    percent_lava: f32,
    percent_water: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Block {
    material: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SubBlock {
    material: u32,
}


#[rustfmt::skip]
pub type TerrainLod =
    HashNestLayer<
        VecNestLayer<
            VecNestLayer<
                VecLayer<
                    SubBlock, 0
                > ,Block, u16, 4 // In reality 2^(16*3) SubBlock_4 should be possible, but 2^48 subblocks would kill anything anyway, so save 75% bytes here. Limit is 65536 full blocks in SubBlocks, means (2^16) * ((2^4)^3) = 268 million
            > ,Chunk , u32,9 // we coult use u16 which would allow 2^31 blocks, however the world could have 2^33 blocks inside, which would mean only 25% could ever be on block level, which is prob never reacher, however ue to handling neibors and that this only would save 1/4 MB on this level, we just skip it for now
        > ,Region ,u16, 13
    >;