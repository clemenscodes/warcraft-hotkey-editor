mod model;
mod view;

pub use view::ToolbarButtonView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::{
    SurfaceState, ToolbarButtonSurface,
};
use dioxus::prelude::*;
use model::ToolbarButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToolbarButton(props: ToolbarButtonModel) -> Element {
    let icon = props.icon;
    let aria_label = props.aria_label;
    let state = SurfaceState::Interactive;
    let disabled = props.disabled;
    let aria_haspopup = props.aria_haspopup;
    let aria_expanded = props.aria_expanded;
    let aria_pressed = props.aria_pressed;
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            ToolbarButtonSurface {
                icon,
                aria_label,
                state,
                disabled,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                onclick,
            }
        }
    }
}

assert_component!(ToolbarButton);
