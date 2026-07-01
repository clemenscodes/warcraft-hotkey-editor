use crate::model::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};
use warcraft_keybinds::GridSlotId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardProps {
    pub unit_id: String,
    pub display_name: String,
    pub icon_path: Option<IconUrl>,
    pub unit_kind: UnitKind,
    pub race: Race,
    pub is_selected: bool,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub active_category: Signal<UnitKind>,
}
