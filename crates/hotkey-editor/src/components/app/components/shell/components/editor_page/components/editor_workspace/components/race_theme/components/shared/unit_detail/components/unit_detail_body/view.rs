use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::logic::{
    UnitCommandGridSlots, UnitOverrideTarget,
};

/// The published `View` contract mirroring [`UnitDetailBodyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitDetailBodyView {
    pub(crate) grid_slots: UnitCommandGridSlots,
    pub(crate) override_target: UnitOverrideTarget,
}

impl ddd::View for UnitDetailBodyView {}
