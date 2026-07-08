pub mod components;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::{
    ToolbarButtonSurface, ToolbarButtonSurfaceProps,
};
use components::collisions_button_badge::{CollisionsButtonBadge, CollisionsButtonBadgeProps};
use dioxus::prelude::*;
use logic::CollisionsButtonPresentation;
pub use props::CollisionsButtonProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CollisionsButton);

/// Toolbar control that surfaces collision health and routes to the Collisions page on
/// click. It reuses the shared gold `ToolbarButtonSurface` for its entire look — amber
/// warning triangle in the `Attention` state while collisions remain, gold circled
/// checkmark in the `Clear` state when the config is clean — and layers a corner count
/// badge on top. Presentational: the collision summary and click handler arrive as props
/// from `CollisionsButtonHost`.
#[component]
pub fn CollisionsButton(props: CollisionsButtonProps) -> Element {
    let CollisionsButtonPresentation {
        surface_state,
        collision_count,
        cross_unit_count,
        per_unit_position_count,
        per_unit_hotkey_count,
        count_label,
        aria_label,
        state_attribute,
        icon,
        onclick,
    } = CollisionsButtonPresentation::from(&props);
    let surface = ToolbarButtonSurfaceProps {
        icon,
        aria_label: "View collisions",
        state: surface_state,
        onclick,
        ..ToolbarButtonSurfaceProps::default()
    };
    let badge_props = CollisionsButtonBadgeProps { label: count_label };
    rsx! {
        div {
            class: CLASS,
            "aria-label": aria_label,
            "data-action": "view-collisions",
            "data-collision-count": collision_count,
            "data-collision-cross-unit": cross_unit_count,
            "data-collision-per-unit-position": per_unit_position_count,
            "data-collision-per-unit-hotkey": per_unit_hotkey_count,
            "data-collision-state": state_attribute,
            ToolbarButtonSurface { ..surface }
            if collision_count > 0 {
                CollisionsButtonBadge { ..badge_props }
            }
        }
    }
}
