#[derive(Debug, Clone, Copy)]
pub enum ServerMsg {
    Attach(),
}

pub type RegionIdSize = i8;
pub type RegionId = (
    /*x*/ RegionIdSize,
    /*y*/ RegionIdSize, /*z = 0*/
);

pub const REGION_MIN: i8 = -64;
pub const REGION_MAX: i8 = 63;
