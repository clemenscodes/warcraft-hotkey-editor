mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::components::shared::toolbar_button_icon::ToolbarButtonIcon;
use dioxus::prelude::*;
pub use props::ClearSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearSurface);

/// The clear resting look of a toolbar button: a gold-bordered surface with a soft
/// resting glow, the affirmative "all clear" look. Presentational — the dispatcher
/// builds its props and renders it when the surface state is `Clear`.
#[component]
pub fn ClearSurface(props: ClearSurfaceProps) -> Element {
    let glyph = props.glyph;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: props.aria_label,
            aria_haspopup: props.aria_haspopup,
            aria_expanded: props.aria_expanded,
            aria_pressed: props.aria_pressed,
            "data-action": props.data_action,
            disabled: props.disabled,
            onclick: props.onclick,
            ToolbarButtonIcon { ..glyph }
        }
    }
}
