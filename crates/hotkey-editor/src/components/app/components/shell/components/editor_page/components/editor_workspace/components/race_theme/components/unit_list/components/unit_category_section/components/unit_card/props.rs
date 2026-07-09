use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind, WarcraftObjectId};

/// One unit's own catalog data: its id, display name, portrait, kind, and race. The
/// selection it drives (whether it is selected, the selected slot, the active category)
/// is read from context by the card's hook, so none of that is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardProps {
    pub unit_id: WarcraftObjectId,
    pub display_name: String,
    pub icon_path: Option<IconUrl>,
    pub unit_kind: UnitKind,
    pub race: Race,
}
