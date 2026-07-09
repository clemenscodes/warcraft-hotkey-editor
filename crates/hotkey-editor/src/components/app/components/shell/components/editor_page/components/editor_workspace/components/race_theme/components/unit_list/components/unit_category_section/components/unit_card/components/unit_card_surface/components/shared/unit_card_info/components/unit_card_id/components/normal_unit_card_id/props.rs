use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id and the `data-race` attribute value the code element carries.
#[derive(Props, Clone, PartialEq)]
pub struct NormalUnitCardIdProps {
    pub race_attribute: &'static str,
    pub unit_id: WarcraftObjectId,
}
