use super::components::hero_selection_row::HeroSelectionRowProps;
use super::props::HeroSelectionHotkeysViewProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::SystemHotkeysSectionIntroProps;

impl From<&HeroSelectionHotkeysViewProps> for HeroSelectionRowProps {
    fn from(props: &HeroSelectionHotkeysViewProps) -> Self {
        let editing_section = props.editing_section;
        Self { editing_section }
    }
}

impl From<&HeroSelectionHotkeysViewProps> for SystemHotkeysSectionIntroProps {
    fn from(_props: &HeroSelectionHotkeysViewProps) -> Self {
        let text = String::from("Hotkeys for selecting your heroes by index.");
        Self { text }
    }
}
