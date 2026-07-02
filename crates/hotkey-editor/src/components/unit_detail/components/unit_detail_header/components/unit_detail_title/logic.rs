use super::components::unit_id::UnitIdProps;
use super::components::unit_name_row::UnitNameRowProps;
use super::props::UnitDetailTitleProps;

impl From<&UnitDetailTitleProps> for UnitNameRowProps {
    fn from(props: &UnitDetailTitleProps) -> Self {
        let unit_name = props.unit_name;
        let has_hero_attributes = props.has_hero_attributes;
        let current_level = props.current_level;
        let is_open = props.is_open;
        let selected_hero_level = props.selected_hero_level;
        let level_picker_open = props.level_picker_open;
        Self {
            unit_name,
            has_hero_attributes,
            current_level,
            is_open,
            selected_hero_level,
            level_picker_open,
        }
    }
}

impl From<&UnitDetailTitleProps> for UnitIdProps {
    fn from(props: &UnitDetailTitleProps) -> Self {
        let text = props.unit_id.clone();
        Self { text }
    }
}
