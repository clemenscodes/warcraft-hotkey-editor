use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::{UnitKind, WarcraftObjectId};

/// One unit's own catalog data: its id, display name, portrait, and kind. The selection
/// it drives (whether it is selected, the selected slot, the active category) is read
/// from context by the card's hook, and its accent colour from `--race-accent`, so
/// neither is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardProps {
    pub unit_id: WarcraftObjectId,
    pub display_name: String,
    pub icon_path: Option<IconUrl>,
    pub unit_kind: UnitKind,
}
