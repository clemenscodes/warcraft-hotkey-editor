use crate::components::app::components::shell::components::collisions_page::logic::UnitIconView;

/// The published `View` contract mirroring [`UnitPositionDetailHeaderProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitPositionDetailHeaderView {
    pub unit: UnitIconView,
    pub count: usize,
}

impl ddd::View for UnitPositionDetailHeaderView {}
