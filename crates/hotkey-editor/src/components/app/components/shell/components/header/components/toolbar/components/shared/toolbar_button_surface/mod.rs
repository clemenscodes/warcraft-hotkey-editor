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
