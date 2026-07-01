use super::components::hero_level_picker::HeroLevelPickerProps;
use super::components::unit_name::UnitNameProps;
use super::props::UnitNameRowProps;

impl From<&UnitNameRowProps> for UnitNameProps {
    fn from(props: &UnitNameRowProps) -> Self {
        let text = props.unit_name;
        Self { text }
    }
}

impl From<&UnitNameRowProps> for HeroLevelPickerProps {
    fn from(props: &UnitNameRowProps) -> Self {
        let current_level = props.current_level;
        let is_open = props.is_open;
        let selected_hero_level = props.selected_hero_level;
        let level_picker_open = props.level_picker_open;
        Self {
            current_level,
            is_open,
            selected_hero_level,
            level_picker_open,
        }
    }
}
