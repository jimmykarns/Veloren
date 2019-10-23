use super::delta::Delta;

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
