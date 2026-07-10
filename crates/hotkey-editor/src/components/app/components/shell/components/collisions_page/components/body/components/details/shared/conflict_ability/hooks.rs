use super::props::ConflictAbilityProps;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The shaped ability: its deep-link trigger inputs (the icon source and alt plus the
/// open-unit handler read from navigation context), its name, and its id. The
/// dialog-free carriers deep-link is wired here, not in the body.
pub(super) struct ConflictAbilityModel {
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) icon_src: Option<String>,
    pub(super) icon_alt: String,
    pub(super) name: String,
    pub(super) ability_id: WarcraftObjectId,
}

pub(super) fn use_conflict_ability(props: &ConflictAbilityProps) -> ConflictAbilityModel {
    let name = props.ability_name.clone();
    let ability_id = props.ability_id;
    let unit_id = props.unit_id;
    let icon_src = props.icon_url.clone();
    let icon_alt = name.clone();
    let view_navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(unit_id));
    ConflictAbilityModel {
        onclick,
        icon_src,
        icon_alt,
        name,
        ability_id,
    }
}
