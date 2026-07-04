use super::components::hero_level_backdrop::HeroLevelBackdropProps;
use super::components::hero_level_menu::HeroLevelMenuProps;
use super::components::hero_level_trigger::HeroLevelTriggerProps;
use super::props::HeroLevelPickerProps;
use dioxus::prelude::*;

impl From<&HeroLevelPickerProps> for HeroLevelTriggerProps {
    fn from(props: &HeroLevelPickerProps) -> Self {
        let number = props.current_level.to_string();
        let is_open = props.is_open;
        let mut level_picker_open = props.level_picker_open;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            let next = !level_picker_open();
            level_picker_open.set(next);
        });
        Self {
            number,
            is_open,
            onclick,
        }
    }
}

impl From<&HeroLevelPickerProps> for HeroLevelMenuProps {
    fn from(props: &HeroLevelPickerProps) -> Self {
        let current_level = props.current_level;
        let selected_hero_level = props.selected_hero_level;
        let level_picker_open = props.level_picker_open;
        Self {
            current_level,
            selected_hero_level,
            level_picker_open,
        }
    }
}

impl From<&HeroLevelPickerProps> for HeroLevelBackdropProps {
    fn from(props: &HeroLevelPickerProps) -> Self {
        let mut level_picker_open = props.level_picker_open;
        let onclick = EventHandler::new(move |_event: MouseEvent| level_picker_open.set(false));
        Self { onclick }
    }
}
