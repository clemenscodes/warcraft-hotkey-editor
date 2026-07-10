use crate::components::app::components::shell::components::collisions_page::logic::UnitIconView;

/// The published `View` contract mirroring [`HotkeyDetailHeaderProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyDetailHeaderView {
    pub unit: UnitIconView,
    pub count: usize,
}

impl ddd::View for HotkeyDetailHeaderView {}
