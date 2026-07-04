use dioxus::prelude::*;

use super::components::alt_state_controls::AltStateControlsProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::AltStateLabelProps;

/// The off-state block of a toggle ability: its name, description lines, and (when
/// editable) the position button and off-state hotkey cell.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateSectionProps {
    pub alt_name_text: Option<String>,
    pub alt_description_lines: Vec<String>,
    pub show_alt_controls: bool,
    pub alt_hotkey_label: String,
    pub alt_hotkey_is_editing: bool,
    pub alt_hotkey_is_special_token: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&AltStateSectionProps> for AltStateLabelProps {
    fn from(props: &AltStateSectionProps) -> Self {
        let text = props.alt_name_text.clone();
        Self { text }
    }
}

impl From<&AltStateSectionProps> for AltStateControlsProps {
    fn from(props: &AltStateSectionProps) -> Self {
        let hotkey_label = props.alt_hotkey_label.clone();
        Self {
            show: props.show_alt_controls,
            hotkey_label,
            is_editing: props.alt_hotkey_is_editing,
            is_special: props.alt_hotkey_is_special_token,
            on_position_click: props.on_position_click,
            on_hotkey_activate: props.on_hotkey_activate,
        }
    }
}
