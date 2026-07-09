use super::components::filled_hotkey_unit_detail::components::hotkey_conflict_grid::components::hotkey_conflict_card::HotkeyConflictCardProps;
use super::props::HotkeyUnitDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnitProps;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The selected unit's header data and its shared-hotkey conflict cards, shaped for
/// the detail pane to render.
#[derive(Clone, PartialEq)]
pub(super) struct HotkeyUnitDetailData {
    pub(super) unit: ConflictDetailUnitProps,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) count: usize,
    pub(super) cards: Vec<HotkeyConflictCardProps>,
}

/// Resolves the selected unit and shapes its header and conflict cards, or `None`
/// when nothing is selected. The selection is read from context by the caller and
/// passed in.
pub(super) fn selected(
    props: &HotkeyUnitDetailProps,
    selected_unit: Signal<Option<String>>,
) -> Option<HotkeyUnitDetailData> {
    let selected_key = selected_unit.read().clone();
    let key = selected_key?;
    let unit_view = props
        .units
        .iter()
        .find(|unit_view| unit_view.key() == key)?
        .clone();
    let unit = unit_view.unit();
    let name = unit.name().to_owned();
    let unit_id = unit.unit_id();
    let icon_url = unit.icon_url().map(str::to_owned);
    let unit_button = ConflictDetailUnitProps {
        unit_id,
        icon_url,
        name: name.clone(),
    };
    let collision_count = unit_view.collision_count();
    let cards = unit_view
        .conflicts()
        .iter()
        .map(|conflict| HotkeyConflictCardProps {
            conflict: conflict.clone(),
            unit_id,
        })
        .collect();
    let data = HotkeyUnitDetailData {
        unit: unit_button,
        name,
        unit_id,
        count: collision_count,
        cards,
    };
    Some(data)
}
