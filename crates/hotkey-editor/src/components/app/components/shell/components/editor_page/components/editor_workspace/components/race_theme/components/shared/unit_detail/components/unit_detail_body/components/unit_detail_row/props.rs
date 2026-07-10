use super::view::UnitDetailRowView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::logic::{
    UnitCommandGridSlots, UnitOverrideTarget,
};
use dioxus::prelude::*;

/// The grids-and-override row: the command grids beside the override panel. Threads the
/// unit's grid slots and override target, splitting them between its two children.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailRowProps {
    pub(crate) grid_slots: UnitCommandGridSlots,
    pub(crate) override_target: UnitOverrideTarget,
}

impl From<&UnitDetailRowView> for UnitDetailRowProps {
    fn from(view: &UnitDetailRowView) -> Self {
        let UnitDetailRowView {
            grid_slots,
            override_target,
        } = view.clone();
        Self {
            grid_slots,
            override_target,
        }
    }
}

impl ddd::Props for UnitDetailRowProps {
    type View = UnitDetailRowView;
}
