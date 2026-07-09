use super::super::super::logic::HotkeyUnitDetailData;
use super::components::hotkey_conflict_grid::components::hotkey_conflict_card::HotkeyConflictCardProps;
use super::components::hotkey_conflict_grid::HotkeyConflictGridProps;
use super::components::hotkey_detail_header::HotkeyDetailHeaderProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnitProps;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The populated shared-hotkey detail pane: the selected unit's header data and its
/// shared-hotkey conflict cards.
#[derive(Props, Clone, PartialEq)]
pub struct FilledHotkeyUnitDetailProps {
    pub unit: ConflictDetailUnitProps,
    pub name: String,
    pub unit_id: WarcraftObjectId,
    pub count: usize,
    pub cards: Vec<HotkeyConflictCardProps>,
}

impl From<&HotkeyUnitDetailData> for FilledHotkeyUnitDetailProps {
    fn from(data: &HotkeyUnitDetailData) -> Self {
        let unit = data.unit.clone();
        let name = data.name.clone();
        let unit_id = data.unit_id;
        let count = data.count;
        let cards = data.cards.clone();
        Self {
            unit,
            name,
            unit_id,
            count,
            cards,
        }
    }
}

impl From<&FilledHotkeyUnitDetailProps> for HotkeyDetailHeaderProps {
    fn from(props: &FilledHotkeyUnitDetailProps) -> Self {
        let unit = props.unit.clone();
        let name = props.name.clone();
        let unit_id = props.unit_id;
        let count = props.count;
        Self {
            unit,
            name,
            unit_id,
            count,
        }
    }
}

impl From<&FilledHotkeyUnitDetailProps> for HotkeyConflictGridProps {
    fn from(props: &FilledHotkeyUnitDetailProps) -> Self {
        let cards = props.cards.clone();
        Self { cards }
    }
}
