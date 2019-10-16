use super::delta::Delta;
use super::index::ToOptionUsize;
use fxhash::FxHashMap;
use std::{u16, u32};
use vek::*;

/*
 traits:
 - DrillDown
 - DrillUp
*/

pub trait DrillDownable {
    type DELTA: Delta;
    fn drill_down(detail: Self) -> Self::DELTA;
}

pub trait DrillUpable {
    type DELTA: Delta;
    fn drill_up(detail: Self) -> Self::DELTA;
}

//#######################################################
