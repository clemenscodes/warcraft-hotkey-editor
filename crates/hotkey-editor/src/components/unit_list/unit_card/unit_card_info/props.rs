use super::components::unit_card_id::UnitCardIdProps;
use super::components::unit_card_name::UnitCardNameProps;
use dioxus::prelude::*;

/// The card's text column: the display name, the unit id, and whether the card is
/// selected (which tints the id).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardInfoProps {
    #[props(into)]
    pub display_name: String,
    #[props(into)]
    pub unit_id: String,
    pub is_selected: bool,
}

impl From<&UnitCardInfoProps> for UnitCardNameProps {
    fn from(props: &UnitCardInfoProps) -> Self {
        let text = props.display_name.clone();
        let is_selected = props.is_selected;
        Self { text, is_selected }
    }
}

impl From<&UnitCardInfoProps> for UnitCardIdProps {
    fn from(props: &UnitCardInfoProps) -> Self {
        let text = props.unit_id.clone();
        let is_selected = props.is_selected;
        Self { text, is_selected }
    }
}
