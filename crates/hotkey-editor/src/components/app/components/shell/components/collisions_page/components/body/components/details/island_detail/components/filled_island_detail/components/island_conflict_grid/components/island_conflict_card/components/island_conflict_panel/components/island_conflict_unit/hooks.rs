use super::props::IslandConflictUnitProps;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The shaped affected-unit heading: its icon source and alt text, name, and id, plus
/// the open-unit handler wired from the navigation read from context.
pub(super) struct IslandConflictUnitModel {
    pub(super) icon_src: Option<String>,
    pub(super) icon_alt: String,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
}

pub(super) fn use_island_conflict_unit(props: &IslandConflictUnitProps) -> IslandConflictUnitModel {
    let name = props.name.clone();
    let unit_id = props.unit_id;
    let icon_src = props.icon_url.clone();
    let icon_alt = name.clone();
    let view_navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(unit_id));
    IslandConflictUnitModel {
        icon_src,
        icon_alt,
        onclick,
        name,
        unit_id,
    }
}
