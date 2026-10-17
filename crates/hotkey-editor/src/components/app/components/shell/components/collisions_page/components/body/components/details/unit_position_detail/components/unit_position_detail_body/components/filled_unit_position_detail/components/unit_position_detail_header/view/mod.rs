use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;

#[derive(Clone, PartialEq)]
pub struct UnitPositionDetailHeaderView {
    pub unit: UnitIconView,
    pub count: usize,
}

impl ddd::Layered for UnitPositionDetailHeaderView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitPositionDetailHeaderView {}
