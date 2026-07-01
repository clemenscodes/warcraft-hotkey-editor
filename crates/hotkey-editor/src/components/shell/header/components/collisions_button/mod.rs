pub mod components;
mod hooks;
mod logic;
mod props;
mod state;
mod style;

use dioxus::prelude::*;

use crate::assert_component;

use components::collisions_button_badge::{CollisionsButtonBadge, CollisionsButtonBadgeProps};
use components::collisions_button_icon::{CollisionsButtonIcon, CollisionsButtonIconProps};
use hooks::use_collisions_button;
use logic::CollisionsButtonPresentation;

pub use props::CollisionsButtonProps;

assert_component!(CollisionsButton);

/// Toolbar icon that surfaces collision health and routes to the Collisions page
/// on click. Amber warning triangle with a corner count while collisions remain,
/// gold circled checkmark when the config is clean.
#[component]
pub fn CollisionsButton(props: CollisionsButtonProps) -> Element {
    let CollisionsButtonPresentation {
        class,
        collision_count,
        cross_unit_count,
        per_unit_position_count,
        per_unit_hotkey_count,
        count_label,
        aria_label,
        state_attribute,
        icon,
        onclick,
    } = use_collisions_button(&props);
    let icon_props = CollisionsButtonIconProps { svg: icon };
    let badge_props = CollisionsButtonBadgeProps { label: count_label };
    rsx! {
        button {
            class,
            r#type: "button",
            "aria-label": aria_label,
            "data-action": "view-collisions",
            "data-collision-count": collision_count,
            "data-collision-cross-unit": cross_unit_count,
            "data-collision-per-unit-position": per_unit_position_count,
            "data-collision-per-unit-hotkey": per_unit_hotkey_count,
            "data-collision-state": state_attribute,
            onclick,
            CollisionsButtonIcon { ..icon_props }
            if collision_count > 0 {
                CollisionsButtonBadge { ..badge_props }
            }
        }
    }
}
