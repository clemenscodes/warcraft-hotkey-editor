use super::components::unit_id::UnitIdProps;
use super::components::unit_name_row::UnitNameRowProps;
use super::props::UnitDetailTitleProps;

impl From<&UnitDetailTitleProps> for UnitNameRowProps {
    fn from(props: &UnitDetailTitleProps) -> Self {
        let unit_name = props.unit_name;
        let has_hero_attributes = props.has_hero_attributes;
        Self {
            unit_name,
            has_hero_attributes,
        }
    }
}

impl From<&UnitDetailTitleProps> for UnitIdProps {
    fn from(props: &UnitDetailTitleProps) -> Self {
        let text = props.unit_id.clone();
        Self { text }
    }
}
