use super::components::unit_position_conflict_card::UnitPositionConflictCardProps;
use super::props::UnitPositionDetailProps;
use crate::components::views::collisions_page::components::body::components::hotkey_detail_unit::HotkeyDetailUnitProps;
use dioxus::prelude::*;

/// The selected unit's header data and its position-conflict cards.
pub(super) struct UnitPositionDetailModel {
    pub(super) unit: HotkeyDetailUnitProps,
    pub(super) name: String,
    pub(super) unit_id_label: String,
    pub(super) count_text: String,
    pub(super) cards: Vec<UnitPositionConflictCardProps>,
}

/// Resolves the selected unit and shapes its header and conflict cards, or `None`
/// when nothing is selected (the pane shows its empty prompt).
pub(super) fn selected(props: &UnitPositionDetailProps) -> Option<UnitPositionDetailModel> {
    let selected_key = props.selected_unit.read().clone();
    let key = selected_key?;
    let unit_view = props
        .units
        .iter()
        .find(|unit_view| unit_view.key() == key)?
        .clone();
    let unit = unit_view.unit();
    let name = unit.name().to_owned();
    let unit_id_label = unit.unit_id().to_owned();
    let icon_url = unit.icon_url().map(str::to_owned);
    let unit_id_for_nav = unit.unit_id().to_owned();
    let view_navigation = props.view_navigation;
    let onclick =
        EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(&unit_id_for_nav));
    let unit_button = HotkeyDetailUnitProps {
        onclick,
        icon_url,
        name: name.clone(),
    };
    let collision_count = unit_view.collision_count();
    let noun = if collision_count == 1 {
        "collision"
    } else {
        "collisions"
    };
    let count_text = format!("{collision_count} {noun}");
    let cards = unit_view
        .conflicts()
        .iter()
        .map(|conflict| UnitPositionConflictCardProps {
            conflict: conflict.clone(),
            unit_id: unit_view.key().to_owned(),
            view_navigation,
        })
        .collect();
    Some(UnitPositionDetailModel {
        unit: unit_button,
        name,
        unit_id_label,
        count_text,
        cards,
    })
}
