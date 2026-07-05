use super::components::collisions_button::CollisionsButtonProps;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::grid_layout::context::use_grid_layout;
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

    let mut summary = use_signal(CollisionSummary::default);
    let mut debounce_generation = use_signal(|| 0_u32);

    use_effect(move || {
        // Subscribe to the inputs so this effect re-runs on each edit...
        let _keys_subscribe = keys.read();
        let _layout_subscribe = grid_layout.read();
        // ...but do the expensive scan only after a 150 ms quiet period,
        // guarded by a generation counter so superseded runs no-op.
        let next_generation = debounce_generation.peek().wrapping_add(1);
        debounce_generation.set(next_generation);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(150).await;
            if *debounce_generation.peek() != next_generation {
                return;
            }
            let read_guard = keys.peek();
            let computed = match read_guard.as_ref() {
                Some(file) => {
                    let layout = *grid_layout.peek();
                    CollisionSummary::compute(file, layout)
                }
                None => CollisionSummary::default(),
            };
            drop(read_guard);
            summary.set(computed);
        });
    });

    let navigation = use_view_navigation();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let target = AppView::Collisions {
            kind: CollisionKind::Positions,
        };
        navigation.apply(target);
    });

    // Subscribing read: the button must re-render when the debounced summary lands.
    let summary_value = *summary.read();
    CollisionsButtonProps {
        summary: summary_value,
        onclick,
    }
}
