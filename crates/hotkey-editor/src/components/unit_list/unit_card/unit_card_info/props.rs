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
