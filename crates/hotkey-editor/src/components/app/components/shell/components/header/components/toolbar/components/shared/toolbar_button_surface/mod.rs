pub mod components;
mod model;
mod view;

pub use view::ToolbarButtonSurfaceView;
mod state;

use components::attention_surface::AttentionSurface;
use components::clear_surface::ClearSurface;
use components::interactive_surface::InteractiveSurface;
use dioxus::prelude::*;
use model::ToolbarButtonSurfaceModel;
pub use state::SurfaceState;
use tw_macro::assert_component;

/// The clickable surface of a toolbar button: the single source of truth for how a
/// toolbar action button looks. A pure dispatcher — from the resting [`SurfaceState`]
/// it renders the matching look: `InteractiveSurface` xor `AttentionSurface` xor
/// `ClearSurface`. Each look owns its own `<button>` root and its full chrome, drawn in
/// `cqi` off the container so the whole button scales as one drawing; this dispatcher
/// only builds each look's props from the shared `ToolbarButtonSurfaceModel` and renders
/// the one the state selects. Consumers swap the icon, the click handler, aria/disabled
/// state, and the resting look (the inline actions use `Interactive`; the collisions
/// button uses `Attention` / `Clear`).
#[component]
pub fn ToolbarButtonSurface(props: ToolbarButtonSurfaceModel) -> Element {
    let icon = props.icon;
    let aria_label = props.aria_label;
    let aria_haspopup = props.aria_haspopup;
    let aria_expanded = props.aria_expanded;
    let aria_pressed = props.aria_pressed;
    let disabled = props.disabled;
    let onclick = props.onclick;
    match props.state {
        SurfaceState::Interactive => rsx! {
            InteractiveSurface {
                icon,
                aria_label,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                disabled,
                onclick,
            }
        },
        SurfaceState::Attention => rsx! {
            AttentionSurface {
                icon,
                aria_label,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                disabled,
                onclick,
            }
        },
        SurfaceState::Clear => rsx! {
            ClearSurface {
                icon,
                aria_label,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                disabled,
                onclick,
            }
        },
    }
}

assert_component!(ToolbarButtonSurface);
