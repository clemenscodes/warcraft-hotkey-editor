use dioxus::prelude::*;
use super::components::unit_detail_title::UnitDetailTitleProps;
use super::components::unit_portrait::UnitPortraitProps;
use super::props::UnitDetailHeaderProps;

impl From<&UnitDetailHeaderProps> for UnitPortraitProps {
    fn from(props: &UnitDetailHeaderProps) -> Self {
        let src = props.portrait_url.clone();
        let alt = props.unit_name;
        Self { src, alt }
    }
}

impl From<&UnitDetailHeaderProps> for UnitDetailTitleProps {
    fn from(props: &UnitDetailHeaderProps) -> Self {
        let unit_name = props.unit_name;
        let unit_id = props.unit_id.clone();
        let has_hero_attributes = props.has_hero_attributes;
        let current_level = *props.selected_hero_level.read();
        let is_open = *props.level_picker_open.read();
        let selected_hero_level = props.selected_hero_level;
        let level_picker_open = props.level_picker_open;
        Self {
            unit_name,
            unit_id,
            has_hero_attributes,
            current_level,
            is_open,
            selected_hero_level,
            level_picker_open,
        }
    }
}
