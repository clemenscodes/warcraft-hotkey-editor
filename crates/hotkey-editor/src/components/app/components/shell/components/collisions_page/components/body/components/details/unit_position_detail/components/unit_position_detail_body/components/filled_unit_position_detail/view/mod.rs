use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;

/// The published `View` contract mirroring [`FilledUnitPositionDetailModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FilledUnitPositionDetailView {
    pub unit_view: UnitPositionUnitView,
}

impl ddd::View for FilledUnitPositionDetailView {}
