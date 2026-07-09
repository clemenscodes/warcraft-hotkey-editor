use super::props::IslandConflictAbilityProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_icon::ConflictAbilityIconProps;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The conflict ability's shaped state: the open-state signal the column owns, the icon
/// props, the click handler that opens this ability's carriers dialog, and the display
/// text the column places directly.
pub(super) struct IslandConflictAbilityView {
    pub(super) open_state: Signal<Option<InspectedAbility>>,
    pub(super) icon: ConflictAbilityIconProps,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) ability_name: String,
    pub(super) ability_id: WarcraftObjectId,
    pub(super) extra_count: usize,
}

/// Owns the column's local open state and wires the click that opens its carriers
/// dialog, so the body only names the result and renders.
pub(super) fn use_island_conflict_ability(
    props: &IslandConflictAbilityProps,
) -> IslandConflictAbilityView {
    let mut open_state = use_signal(|| None::<InspectedAbility>);
    let inspected = props.inspected.clone();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let opened = inspected.clone();
        open_state.set(Some(opened));
    });
    let ability_name = props.ability_name.clone();
    let icon = ConflictAbilityIconProps {
        src: props.icon_url.clone(),
        alt: ability_name.clone(),
    };
    let ability_id = props.ability_id;
    let extra_count = props.extra_count;
    IslandConflictAbilityView {
        open_state,
        icon,
        onclick,
        ability_name,
        ability_id,
        extra_count,
    }
}
