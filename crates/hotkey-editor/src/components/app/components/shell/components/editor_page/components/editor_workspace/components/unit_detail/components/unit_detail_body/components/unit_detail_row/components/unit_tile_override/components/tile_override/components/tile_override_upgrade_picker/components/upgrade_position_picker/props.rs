use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_explainer::AltPositionPickerExplainerProps;

/// The upgraded-form position picker: a modal command grid where the upgraded-form
/// button can be dragged to a new cell. The shared editor signals the grid needs are
/// sourced from context by the component's hook.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradePositionPickerProps {
    pub upgrade_unit_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub upgrade_position_picker_open: Signal<bool>,
}

impl From<&UpgradePositionPickerProps> for AltPositionPickerExplainerProps {
    fn from(_props: &UpgradePositionPickerProps) -> Self {
        let text = String::from(
            "Drag the upgraded-form button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact.",
        );
        Self { text }
    }
}
