pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::CollisionsButtonView;

use components::collisions_button_badge::CollisionsButtonBadge;
use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::ToolbarButtonSurface;
use dioxus::prelude::*;
use model::CollisionsButtonModel;
use presentation::CollisionsButtonPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionsButton(props: CollisionsButtonModel) -> Element {
    let CollisionsButtonPresentation {
        surface_state,
        collision_count,
        count_label,
        aria_label,
        icon,
        onclick,
    } = CollisionsButtonPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            "aria-label": aria_label,
            ToolbarButtonSurface {
                icon,
                aria_label: "View collisions",
                state: surface_state,
                onclick,
            }
            if collision_count > 0 {
                CollisionsButtonBadge {
                    label: count_label,
                }
            }
        }
    }
}

assert_component!(CollisionsButton);
