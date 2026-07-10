use super::view::UnitDetailBodyView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::logic::{
    UnitCommandGridSlots, UnitOverrideTarget,
};
use dioxus::prelude::*;

/// The body of the card below the stats: the grids-and-override row. It threads the
/// unit's grid slots and override target down to the row, which splits them between the
/// command grids and the override panel.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailBodyProps {
    pub(crate) grid_slots: UnitCommandGridSlots,
    pub(crate) override_target: UnitOverrideTarget,
}

impl From<&UnitDetailBodyView> for UnitDetailBodyProps {
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

impl ddd::Props for UnitDetailBodyProps {
    type View = UnitDetailBodyView;
}
