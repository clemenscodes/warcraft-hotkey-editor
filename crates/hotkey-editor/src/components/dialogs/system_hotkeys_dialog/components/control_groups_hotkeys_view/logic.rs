use super::components::control_groups_row::ControlGroupsRow;
use super::components::control_groups_row::ControlGroupsRowProps;
use super::props::ControlGroupsHotkeysViewProps;
use crate::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_section::SystemHotkeysSectionProps;
use dioxus::prelude::*;

impl From<&ControlGroupsHotkeysViewProps> for ControlGroupsRowProps {
    fn from(props: &ControlGroupsHotkeysViewProps) -> Self {
        let loaded_keys = props.loaded_keys;
        let editing_section = props.editing_section;
        Self {
            loaded_keys,
            editing_section,
        }
    }
}

impl From<&ControlGroupsHotkeysViewProps> for SystemHotkeysSectionProps {
    fn from(props: &ControlGroupsHotkeysViewProps) -> Self {
        let intro = String::from("Hotkeys for control groups 1–10.");
        let row = ControlGroupsRowProps::from(props);
        let children = rsx! {
            ControlGroupsRow { ..row }
        };
        Self { intro, children }
    }
}
