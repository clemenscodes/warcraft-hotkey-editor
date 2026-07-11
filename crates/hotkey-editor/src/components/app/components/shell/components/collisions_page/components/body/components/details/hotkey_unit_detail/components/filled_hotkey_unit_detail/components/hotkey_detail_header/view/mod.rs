use crate::components::app::components::shell::components::collisions_page::presentation::UnitIconView;

/// The published `View` contract mirroring [`HotkeyDetailHeaderModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyDetailHeaderView {
    pub unit: UnitIconView,
    pub count: usize,
}

impl ddd::View for HotkeyDetailHeaderView {}
