use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;

#[derive(Clone, PartialEq)]
pub struct UnitPositionDetailView {
    pub units: Vec<UnitPositionUnitView>,
}

impl ddd::View for UnitPositionDetailView {}
