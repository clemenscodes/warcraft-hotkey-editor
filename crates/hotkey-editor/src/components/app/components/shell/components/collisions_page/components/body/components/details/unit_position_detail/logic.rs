use super::components::filled_unit_position_detail::components::unit_position_conflict_grid::components::unit_position_conflict_card::UnitPositionConflictCardProps;
use super::props::UnitPositionDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnitProps;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The selected unit's header data and its position-conflict cards, shaped for the
/// detail pane to render.
#[derive(Clone, PartialEq)]
pub(super) struct UnitPositionDetailData {
    pub(super) unit: ConflictDetailUnitProps,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
    pub(super) count: usize,
    pub(super) cards: Vec<UnitPositionConflictCardProps>,
}

/// Resolves the selected unit and shapes its header and conflict cards, or `None`
/// when nothing is selected.
pub(super) fn selected(
    props: &UnitPositionDetailProps,
    selected_unit: Signal<Option<String>>,
) -> Option<UnitPositionDetailData> {
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
        .map(|conflict| UnitPositionConflictCardProps {
            conflict: conflict.clone(),
            unit_id,
        })
        .collect();
    let data = UnitPositionDetailData {
        unit: unit_button,
        name,
        unit_id,
        count: collision_count,
        cards,
    };
    Some(data)
}
