use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id and the `data-race` attribute value the code element carries
/// (which selects the race accent color).
#[derive(Props, Clone, PartialEq)]
pub struct SelectedUnitCardIdProps {
    pub race_attribute: &'static str,
    pub unit_id: WarcraftObjectId,
}
