use super::components::control_groups_row::ControlGroupsRowProps;
use super::props::ControlGroupsHotkeysViewProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::SystemHotkeysSectionIntroProps;

impl From<&ControlGroupsHotkeysViewProps> for ControlGroupsRowProps {
    fn from(props: &ControlGroupsHotkeysViewProps) -> Self {
        let editing_section = props.editing_section;
        Self { editing_section }
    }
}

impl From<&ControlGroupsHotkeysViewProps> for SystemHotkeysSectionIntroProps {
    fn from(_props: &ControlGroupsHotkeysViewProps) -> Self {
        let text = String::from("Hotkeys for control groups 1–10.");
        Self { text }
    }
}
