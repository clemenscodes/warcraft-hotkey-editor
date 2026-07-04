use super::components::unit_card_icon::UnitCardIconProps;
use super::components::unit_card_info::UnitCardInfoProps;
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

impl From<&UnitCardProps> for UnitCardIconProps {
    fn from(props: &UnitCardProps) -> Self {
        let icon_path = props.icon_path.clone();
        let display_name = props.display_name.clone();
        Self {
            icon_path,
            display_name,
        }
    }
}

impl From<&UnitCardProps> for UnitCardInfoProps {
    fn from(props: &UnitCardProps) -> Self {
        let display_name = props.display_name.clone();
        let unit_id = props.unit_id.clone();
        let race = props.race;
        let is_selected = props.is_selected;
        Self {
            display_name,
            unit_id,
            race,
            is_selected,
        }
    }
}
