use super::delta::DeltaStore;

/*
 traits:
 - DrillDown
 - DrillUp
*/

pub trait DrillDownable {
    type DELTA: DeltaStore;
    fn drill_down(detail: Self) -> Self::DELTA;
}

pub trait DrillUpable {
    type DELTA: DeltaStore;
    fn drill_up(detail: Self) -> Self::DELTA;
}

//#######################################################
