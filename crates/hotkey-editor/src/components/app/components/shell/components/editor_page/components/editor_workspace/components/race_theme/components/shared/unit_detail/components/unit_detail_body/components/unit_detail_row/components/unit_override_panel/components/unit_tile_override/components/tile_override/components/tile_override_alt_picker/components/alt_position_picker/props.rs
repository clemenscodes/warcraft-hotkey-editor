use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_body::components::alt_position_picker_explainer::AltPositionPickerExplainerProps;

/// The off-state position picker: a modal command grid where the off-state button can
/// be dragged to a new cell. The shared editor signals the grid needs are sourced from
/// context by the component's hook, so only the picker's own identity is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerProps {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub alt_position_picker_open: Signal<bool>,
}

impl From<&AltPositionPickerProps> for AltPositionPickerExplainerProps {
    fn from(_props: &AltPositionPickerProps) -> Self {
        let text = String::from(
            "Drag the off-state button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact.",
        );
        Self { text }
    }
}
