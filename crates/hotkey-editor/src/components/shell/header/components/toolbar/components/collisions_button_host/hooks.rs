use super::components::collisions_button::CollisionsButtonProps;
use crate::services::customkeys::service::CustomKeysService;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use warcraft_keybinds::{CollisionSummary, GridLayout};

/// The seam: reads the live config from the [`CustomKeysService`] and the grid
/// layout from context, asks the domain to count the collisions (recomputing only
/// when either changes), and wires the click that routes to the Collisions page.
/// Hands the leaf the finished summary and handler — the minimal data it paints.
pub(super) fn use_collisions_button() -> CollisionsButtonProps {
    let custom_keys_service = use_context::<CustomKeysService>();
    let keys = custom_keys_service.keys();
    let grid_layout = use_context::<Signal<GridLayout>>();
    let summary_memo = use_memo(move || {
        let read_guard = keys.read();
        let Some(file) = read_guard.as_ref() else {
            return CollisionSummary::default();
        };
        let layout = *grid_layout.read();
        CollisionSummary::compute(file, layout)
    });
    let navigation = use_context::<ViewNavigationContext>();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let target = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        navigation.apply(target);
    });
    let summary = summary_memo();
    CollisionsButtonProps { summary, onclick }
}
