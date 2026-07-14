use dioxus::prelude::*;

use super::model::CurrentHotkeySlotModel;

/// The domain data for the hotkey / research-hotkey cell shown in the slot.
pub(super) struct HotkeySlotKeyCell {
    pub(super) label: String,
    pub(super) is_editing: bool,
    pub(super) is_special: bool,
    pub(super) title: String,
    pub(super) on_activate: EventHandler<()>,
}

/// Which of the three mutually-exclusive slot contents applies: the hotkey/research
/// cell, the passive note, or nothing.
pub(super) struct CurrentHotkeySlotDispatch {
    pub(super) key_cell: Option<HotkeySlotKeyCell>,
    pub(super) info_text: Option<String>,
}

impl From<&CurrentHotkeySlotModel> for CurrentHotkeySlotDispatch {
    fn from(props: &CurrentHotkeySlotModel) -> Self {
        if props.show_hotkey_field {
            let title = String::from("Hotkey");
            let key_cell = HotkeySlotKeyCell {
                label: props.hotkey_label.clone(),
                is_editing: props.hotkey_is_editing,
                is_special: props.hotkey_is_special,
                title,
                on_activate: props.on_hotkey_activate,
            };
            return Self {
                key_cell: Some(key_cell),
                info_text: None,
            };
        }
        if props.show_research_field {
            let title = String::from("Research hotkey");
            let key_cell = HotkeySlotKeyCell {
                label: props.research_label.clone(),
                is_editing: props.research_is_editing,
                is_special: props.research_is_special,
                title,
                on_activate: props.on_research_activate,
            };
            return Self {
                key_cell: Some(key_cell),
                info_text: None,
            };
        }
        if props.is_info_only {
            let info_text = String::from("Passive racial ability");
            return Self {
                key_cell: None,
                info_text: Some(info_text),
            };
        }
        Self {
            key_cell: None,
            info_text: None,
        }
    }
}

impl ddd::Presentation for CurrentHotkeySlotDispatch {
    type Model = CurrentHotkeySlotModel;
}
