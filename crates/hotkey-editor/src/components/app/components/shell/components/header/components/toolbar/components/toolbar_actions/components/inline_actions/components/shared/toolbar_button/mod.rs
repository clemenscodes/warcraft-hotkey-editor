mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::{
    SurfaceState, ToolbarButtonSurface,
};
use dioxus::prelude::*;
use props::ToolbarButtonProps;
use style::CLASS;
use tw_macro::assert_component;

/// The container for a toolbar action button. It owns only the button's box: it fills
/// the height it is given, locks itself square (`aspect-square`, so it is never
/// stretched), and marks itself a query container. The parent that places it decides
/// how large the box is — a single length on the toolbar sizes the whole row, a Host
/// container resizes one button — and the surface inside scales its entire look off
/// this box in `cqi`, like one drawing.
#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
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
