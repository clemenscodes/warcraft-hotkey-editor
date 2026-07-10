use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionUnitView;

/// The published `View` contract mirroring [`UnitPositionsContentProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitPositionsContentView {
    pub units: Vec<UnitPositionUnitView>,
}

impl ddd::View for UnitPositionsContentView {}
