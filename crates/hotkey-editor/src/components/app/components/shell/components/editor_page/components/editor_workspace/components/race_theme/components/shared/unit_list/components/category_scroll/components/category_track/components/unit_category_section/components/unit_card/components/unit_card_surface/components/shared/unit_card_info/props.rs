use super::components::unit_card_id::UnitCardIdProps;
use super::components::unit_card_name::UnitCardNameProps;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The card's text column: the display name, the unit id, and whether the card is
/// selected. The id's selected accent is read from `--race-accent`, so no race is
/// threaded in.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardInfoProps {
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
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
        let is_selected = props.is_selected;
        Self {
            unit_id,
            is_selected,
        }
    }
}
