mod model;
mod view;

pub use view::AttentionSurfaceView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::components::shared::toolbar_button_icon::ToolbarButtonIcon;
use dioxus::prelude::*;
use model::AttentionSurfaceModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AttentionSurface(props: AttentionSurfaceModel) -> Element {
    let icon = props.icon;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: props.aria_label,
            aria_haspopup: props.aria_haspopup,
            aria_expanded: props.aria_expanded,
            aria_pressed: props.aria_pressed,
            disabled: props.disabled,
            onclick: props.onclick,
            ToolbarButtonIcon {
                icon,
            }
        }
    }
}

assert_component!(AttentionSurface);
