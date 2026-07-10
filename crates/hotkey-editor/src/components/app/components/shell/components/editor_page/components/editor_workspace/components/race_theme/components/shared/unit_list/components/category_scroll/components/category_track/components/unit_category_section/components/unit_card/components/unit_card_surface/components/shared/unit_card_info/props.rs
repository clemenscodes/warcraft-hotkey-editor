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
