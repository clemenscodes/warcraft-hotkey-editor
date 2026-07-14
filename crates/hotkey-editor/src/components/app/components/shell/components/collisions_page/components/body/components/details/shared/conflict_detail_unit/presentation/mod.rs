use super::model::ConflictDetailUnitModel;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

pub(super) struct ConflictDetailUnitPresentation {
    pub(super) icon_src: Option<String>,
    pub(super) icon_alt: String,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_conflict_detail_unit(
    props: &ConflictDetailUnitModel,
) -> ConflictDetailUnitPresentation {
    let unit_id = props.unit_id;
    let icon_src = props.icon_url.clone();
    let icon_alt = props.name.clone();
    let view_navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(unit_id));
    ConflictDetailUnitPresentation {
        icon_src,
        icon_alt,
        onclick,
    }
}
