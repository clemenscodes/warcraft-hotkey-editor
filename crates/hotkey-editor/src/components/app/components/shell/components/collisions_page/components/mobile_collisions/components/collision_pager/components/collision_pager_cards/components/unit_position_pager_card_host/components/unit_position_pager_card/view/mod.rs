use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;

#[derive(Clone, PartialEq)]
pub struct UnitPositionPagerCardView {
    pub unit: UnitPositionUnitView,
}

impl ddd::Layered for UnitPositionPagerCardView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitPositionPagerCardView {}
