use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;

#[derive(Clone, PartialEq)]
pub struct UnitPositionPagerCardHostView {
    pub unit: UnitPositionUnitView,
}

impl ddd::Layered for UnitPositionPagerCardHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitPositionPagerCardHostView {}
