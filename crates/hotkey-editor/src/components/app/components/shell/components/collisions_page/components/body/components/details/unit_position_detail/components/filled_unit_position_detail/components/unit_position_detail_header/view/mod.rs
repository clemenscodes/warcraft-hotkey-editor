use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;

/// The published `View` contract mirroring [`UnitPositionDetailHeaderModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitPositionDetailHeaderView {
    pub unit: UnitIconView,
    pub count: usize,
}

impl ddd::View for UnitPositionDetailHeaderView {}
