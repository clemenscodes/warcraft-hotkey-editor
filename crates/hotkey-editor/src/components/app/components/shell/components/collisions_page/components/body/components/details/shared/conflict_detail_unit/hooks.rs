use super::props::ConflictDetailUnitProps;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// The shaped detail-header unit: its portrait icon source and alt and the open-unit
/// handler wired from the navigation read from context.
pub(super) struct ConflictDetailUnitModel {
    pub(super) icon_src: Option<String>,
    pub(super) icon_alt: String,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_conflict_detail_unit(props: &ConflictDetailUnitProps) -> ConflictDetailUnitModel {
    let unit_id = props.unit_id;
    let icon_src = props.icon_url.clone();
    let icon_alt = props.name.clone();
    let view_navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(unit_id));
    ConflictDetailUnitModel {
        icon_src,
        icon_alt,
        onclick,
    }
}
