use super::components::normal_unit_card_id::NormalUnitCardIdProps;
use super::components::selected_unit_card_id::SelectedUnitCardIdProps;
use super::props::UnitCardIdProps;
use warcraft_api::RaceLabels;

impl From<&UnitCardIdProps> for NormalUnitCardIdProps {
    fn from(props: &UnitCardIdProps) -> Self {
        let race_attribute = RaceLabels::data_attribute(props.race);
        let unit_id = props.unit_id;
        Self {
            race_attribute,
            unit_id,
        }
    }
}

impl From<&UnitCardIdProps> for SelectedUnitCardIdProps {
    fn from(props: &UnitCardIdProps) -> Self {
        let race_attribute = RaceLabels::data_attribute(props.race);
        let unit_id = props.unit_id;
        Self {
            race_attribute,
            unit_id,
        }
    }
}
