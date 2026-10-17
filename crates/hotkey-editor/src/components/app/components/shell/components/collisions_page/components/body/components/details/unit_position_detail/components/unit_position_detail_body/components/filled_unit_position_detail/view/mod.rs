use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;

#[derive(Clone, PartialEq)]
pub struct FilledUnitPositionDetailView {
    pub unit_view: UnitPositionUnitView,
}

impl ddd::Layered for FilledUnitPositionDetailView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FilledUnitPositionDetailView {}
