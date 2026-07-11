use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;

/// The published `View` contract mirroring [`UnitPositionDetailModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitPositionDetailView {
    pub units: Vec<UnitPositionUnitView>,
}

impl ddd::View for UnitPositionDetailView {}
