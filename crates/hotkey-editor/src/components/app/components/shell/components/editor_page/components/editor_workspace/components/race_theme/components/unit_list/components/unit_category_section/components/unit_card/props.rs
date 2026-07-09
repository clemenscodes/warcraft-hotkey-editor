use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind, WarcraftObjectId};
use warcraft_keybinds::GridSlotId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardProps {
    pub unit_id: WarcraftObjectId,
    pub display_name: String,
    pub icon_path: Option<IconUrl>,
    pub unit_kind: UnitKind,
    pub race: Race,
    pub is_selected: bool,
    pub selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub active_category: Signal<UnitKind>,
}
