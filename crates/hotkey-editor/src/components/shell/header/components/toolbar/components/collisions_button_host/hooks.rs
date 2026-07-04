use super::components::collisions_button::CollisionsButtonProps;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::grid_layout::use_grid_layout;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_keybinds::CollisionSummary;

/// The seam: reads the live config from the [`CustomKeysService`](crate::services::customkeys::service::CustomKeysService) and the grid
/// layout from context, asks the domain to count the collisions (recomputing only
/// when either changes), and wires the click that routes to the Collisions page.
/// Hands the leaf the finished summary and handler — the minimal data it paints.
pub(super) fn use_collisions_button() -> CollisionsButtonProps {
    let custom_keys_service = use_custom_keys_service();
    let keys = custom_keys_service.keys();
    let grid_layout = use_grid_layout();
    let summary_memo = use_memo(move || {
        let read_guard = keys.read();
        let Some(file) = read_guard.as_ref() else {
            return CollisionSummary::default();
        };
        let layout = *grid_layout.read();
        CollisionSummary::compute(file, layout)
    });
    let navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let target = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        navigation.apply(target);
    });
    let summary = summary_memo();
    CollisionsButtonProps { summary, onclick }
}
