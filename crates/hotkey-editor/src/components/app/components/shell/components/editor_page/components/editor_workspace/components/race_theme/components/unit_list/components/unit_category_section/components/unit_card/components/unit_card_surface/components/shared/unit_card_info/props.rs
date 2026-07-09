use super::components::unit_card_id::UnitCardIdProps;
use super::components::unit_card_name::UnitCardNameProps;
use dioxus::prelude::*;
use warcraft_api::{Race, WarcraftObjectId};

/// The card's text column: the display name, the unit id, the card's race (which
/// tints the id when selected), and whether the card is selected.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardInfoProps {
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub race: Race,
    pub is_selected: bool,
}

impl From<&UnitCardInfoProps> for UnitCardNameProps {
    fn from(props: &UnitCardInfoProps) -> Self {
        let text = props.display_name.clone();
        Self { text }
    }
}

impl From<&UnitCardInfoProps> for UnitCardIdProps {
    fn from(props: &UnitCardInfoProps) -> Self {
        let unit_id = props.unit_id;
        let race = props.race;
        let is_selected = props.is_selected;
        Self {
            unit_id,
            race,
            is_selected,
        }
    }
}
