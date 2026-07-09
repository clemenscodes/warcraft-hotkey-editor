use super::components::conflict_detail_unit_icon::ConflictDetailUnitIconProps;
use super::props::ConflictDetailUnitProps;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// The shaped detail-header unit: its portrait icon and the open-unit handler wired
/// from the navigation read from context.
pub(super) struct ConflictDetailUnitModel {
    pub(super) icon: ConflictDetailUnitIconProps,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_conflict_detail_unit(props: &ConflictDetailUnitProps) -> ConflictDetailUnitModel {
    let unit_id = props.unit_id;
    let icon = ConflictDetailUnitIconProps {
        src: props.icon_url.clone(),
        alt: props.name.clone(),
    };
    let view_navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(unit_id));
    ConflictDetailUnitModel { icon, onclick }
}
