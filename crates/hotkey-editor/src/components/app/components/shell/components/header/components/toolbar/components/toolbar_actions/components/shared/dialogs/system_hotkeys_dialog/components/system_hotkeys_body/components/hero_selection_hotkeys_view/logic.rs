use super::components::hero_selection_row::HeroSelectionRow;
use super::components::hero_selection_row::HeroSelectionRowProps;
use super::props::HeroSelectionHotkeysViewProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_hotkeys_section::SystemHotkeysSectionProps;
use dioxus::prelude::*;

impl From<&HeroSelectionHotkeysViewProps> for HeroSelectionRowProps {
    fn from(props: &HeroSelectionHotkeysViewProps) -> Self {
        let editing_section = props.editing_section;
        Self { editing_section }
    }
}

impl From<&HeroSelectionHotkeysViewProps> for SystemHotkeysSectionProps {
    fn from(props: &HeroSelectionHotkeysViewProps) -> Self {
        let intro = String::from("Hotkeys for selecting your heroes by index.");
        let row = HeroSelectionRowProps::from(props);
        let children = rsx! {
            HeroSelectionRow { ..row }
        };
        Self { intro, children }
    }
}
