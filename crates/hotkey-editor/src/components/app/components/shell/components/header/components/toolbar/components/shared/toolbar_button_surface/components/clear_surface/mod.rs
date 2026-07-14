mod model;
mod view;

pub use view::ClearSurfaceView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::components::shared::toolbar_button_icon::ToolbarButtonIcon;
use dioxus::prelude::*;
use model::ClearSurfaceModel;
use style::CLASS;
use tw_macro::assert_component;

/// The clear resting look of a toolbar button: a gold-bordered surface with a soft
/// resting glow, the affirmative "all clear" look. Presentational — the dispatcher
/// builds its props and renders it when the surface state is `Clear`.
#[component]
pub fn ClearSurface(props: ClearSurfaceModel) -> Element {
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

assert_component!(ClearSurface);
