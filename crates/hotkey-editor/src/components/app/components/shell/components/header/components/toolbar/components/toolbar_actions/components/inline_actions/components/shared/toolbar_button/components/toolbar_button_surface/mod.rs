pub mod components;
mod props;
mod style;

use components::toolbar_button_icon::{ToolbarButtonIcon, ToolbarButtonIconProps};
use dioxus::prelude::*;
pub use props::ToolbarButtonSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ToolbarButtonSurface);

/// The clickable surface of a toolbar button: the single source of truth for how a
/// toolbar action button looks. It fills the container it sits in and draws its entire
/// chrome — border, radius, gradient, focus and hover treatment, glyph size — in `cqi`
/// off that container, so the whole button scales as one drawing when the container is
/// resized. Consumers swap only the icon, the click handler, and aria/disabled state.
#[component]
pub fn ToolbarButtonSurface(props: ToolbarButtonSurfaceProps) -> Element {
    let glyph = ToolbarButtonIconProps::from(&props);
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
