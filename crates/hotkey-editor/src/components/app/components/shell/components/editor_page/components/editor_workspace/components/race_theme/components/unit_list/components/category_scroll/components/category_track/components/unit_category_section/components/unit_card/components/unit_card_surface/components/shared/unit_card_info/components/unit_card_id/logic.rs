use super::components::normal_unit_card_id::NormalUnitCardIdProps;
use super::components::selected_unit_card_id::SelectedUnitCardIdProps;
use super::props::UnitCardIdProps;

impl From<&UnitCardIdProps> for NormalUnitCardIdProps {
    fn from(props: &UnitCardIdProps) -> Self {
        let unit_id = props.unit_id;
        Self { unit_id }
    }
}

impl From<&UnitCardIdProps> for SelectedUnitCardIdProps {
    fn from(props: &UnitCardIdProps) -> Self {
        let unit_id = props.unit_id;
        Self { unit_id }
    }
}
