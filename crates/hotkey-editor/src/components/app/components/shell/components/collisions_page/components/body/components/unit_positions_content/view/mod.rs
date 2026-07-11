use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;

/// The published `View` contract mirroring [`UnitPositionsContentModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitPositionsContentView {
    pub units: Vec<UnitPositionUnitView>,
}

impl ddd::View for UnitPositionsContentView {}
