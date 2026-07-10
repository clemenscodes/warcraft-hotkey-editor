use super::props::FightColumnProps;
use crate::services::carriers::InspectedAbility;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The mover column's shaped state: the name-button fields (with the open-unit handler
/// wired from the navigation context) and the ability-icon fields. The mover never wins
/// its own cell, so the icon is not ringed.
pub(super) struct FightColumnModel {
    pub(super) name: String,
    pub(super) object_id: WarcraftObjectId,
    pub(super) has_unit: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) icon_url: Option<String>,
    pub(super) carrier_count: usize,
    pub(super) disabled: bool,
    pub(super) inspected: InspectedAbility,
}

/// Reads the navigation from context and shapes the mover column: its name button (which
/// opens the mover's unit through that navigation) and its ability icon.
pub(super) fn use_fight_column(props: &FightColumnProps) -> FightColumnModel {
    let view_navigation = use_view_navigation();
    let move_view = props.move_view.clone();
    let mover = move_view.mover();
    let mover_unit_id = move_view.mover_unit_id();
    let has_unit = mover_unit_id.is_some();
    let name = mover.name().to_owned();
    let object_id = mover.object_id();
    let icon_url = mover.icon_url().map(str::to_owned);
    let carrier_count = move_view.mover_carriers();
    let carrier_unit_ids_ref = move_view.mover_carrier_unit_ids();
    let carrier_unit_ids = carrier_unit_ids_ref.to_vec();
    let disabled = carrier_unit_ids.is_empty();
    let name_for_inspected = name.clone();
    let inspected = InspectedAbility::new(name_for_inspected, carrier_unit_ids);
    let open_unit_id = mover_unit_id;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        if let Some(unit_id) = open_unit_id {
            view_navigation.open_unit(unit_id);
        }
    });
    FightColumnModel {
        name,
        object_id,
        has_unit,
        onclick,
        icon_url,
        carrier_count,
        disabled,
        inspected,
    }
}
