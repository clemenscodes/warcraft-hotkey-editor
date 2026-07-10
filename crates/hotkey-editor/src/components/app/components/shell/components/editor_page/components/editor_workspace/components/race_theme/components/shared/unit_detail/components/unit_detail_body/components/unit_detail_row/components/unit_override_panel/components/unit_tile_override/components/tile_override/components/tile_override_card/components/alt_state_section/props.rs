use dioxus::prelude::*;

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
