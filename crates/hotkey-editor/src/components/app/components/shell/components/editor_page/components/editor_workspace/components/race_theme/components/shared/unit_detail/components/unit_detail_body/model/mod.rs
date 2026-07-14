use super::view::UnitDetailBodyView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::presentation::{
    UnitCommandGridSlots, UnitOverrideTarget,
};
use dioxus::prelude::*;

/// The body of the card below the stats: the grids-and-override row. It threads the
/// unit's grid slots and override target down to the row, which splits them between the
/// command grids and the hotkey-override section.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailBodyModel {
    pub(crate) grid_slots: UnitCommandGridSlots,
    pub(crate) override_target: UnitOverrideTarget,
}

impl From<&UnitDetailBodyView> for UnitDetailBodyModel {
    fn from(view: &UnitDetailBodyView) -> Self {
        let UnitDetailBodyView {
            grid_slots,
            override_target,
        } = view.clone();
        Self {
            grid_slots,
            override_target,
        }
    }
}

impl ddd::Model for UnitDetailBodyModel {
    type View = UnitDetailBodyView;
}
