use crate::components::app::components::shell::components::resolve_page::logic::{
    MiniGridPlacement, UnresolvedView,
};

/// The single mini-grid placement for a stuck ability: its icon pinned to the cell it
/// could not be moved off. Wrapped in a `Vec` because the mini grid renders a list.
pub(super) fn placements(unresolved_view: &UnresolvedView) -> Vec<MiniGridPlacement> {
    let ability = unresolved_view.ability();
    let name = ability.name().to_owned();
    let icon_url = ability.icon_url().map(str::to_owned);
    let position = unresolved_view.position();
    let placement = MiniGridPlacement::new(position, icon_url, name);
    vec![placement]
}
