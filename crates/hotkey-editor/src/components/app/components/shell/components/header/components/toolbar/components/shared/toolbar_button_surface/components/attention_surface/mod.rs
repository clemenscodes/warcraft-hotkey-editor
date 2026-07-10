mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::components::shared::toolbar_button_icon::ToolbarButtonIcon;
use dioxus::prelude::*;
pub use props::AttentionSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AttentionSurface);

/// The attention resting look of a toolbar button: a persistently gold surface used
/// when the button is surfacing a condition that needs the user's eye. Presentational
/// — the dispatcher builds its props and renders it when the surface state is
/// `Attention`.
#[component]
pub fn AttentionSurface(props: AttentionSurfaceProps) -> Element {
    let glyph = props.glyph;
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
            ToolbarButtonIcon { ..glyph }
        }
    }
}
