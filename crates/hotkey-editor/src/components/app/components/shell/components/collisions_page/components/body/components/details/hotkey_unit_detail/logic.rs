use super::components::hotkey_conflict_grid::components::hotkey_conflict_card::HotkeyConflictCardProps;
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
/// when nothing is selected.
pub(super) fn selected(props: &HotkeyUnitDetailProps) -> Option<HotkeyUnitDetailData> {
    let selected_key = props.selected_unit.read().clone();
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
    let unit_id_for_navigation = unit.unit_id();
    let view_navigation = props.view_navigation;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        view_navigation.open_unit(unit_id_for_navigation)
    });
    let unit_button = ConflictDetailUnitProps {
        onclick,
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
            view_navigation,
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
