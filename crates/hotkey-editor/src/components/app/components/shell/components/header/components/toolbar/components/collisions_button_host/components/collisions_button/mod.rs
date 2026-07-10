pub mod components;
mod logic;
mod props;
mod view;

pub use view::CollisionsButtonView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::ToolbarButtonSurface;
use components::collisions_button_badge::CollisionsButtonBadge;
use dioxus::prelude::*;
use logic::CollisionsButtonPresentation;
use props::CollisionsButtonProps;
use style::CLASS;
use tw_macro::assert_component;

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
                CollisionsButtonBadge { label: count_label }
            }
        }
    }
}

assert_component!(CollisionsButton);
