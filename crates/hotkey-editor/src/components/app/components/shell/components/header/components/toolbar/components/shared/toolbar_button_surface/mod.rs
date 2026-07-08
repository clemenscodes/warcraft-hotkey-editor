pub mod components;
mod props;
mod state;
mod style;

use components::toolbar_button_icon::{ToolbarButtonIcon, ToolbarButtonIconProps};
use dioxus::prelude::*;
pub use props::ToolbarButtonSurfaceProps;
pub use state::SurfaceState;
use tw_macro::assert_component;
assert_component!(ToolbarButtonSurface);

/// The clickable surface of a toolbar button: the single source of truth for how a
/// toolbar action button looks. It fills the container it sits in and draws its entire
/// chrome — border, radius, gradient, focus and hover treatment, glyph size — in `cqi`
/// off that container, so the whole button scales as one drawing when the container is
/// resized. Consumers swap the icon, the click handler, aria/disabled state, and the
/// resting [`SurfaceState`] look (the inline actions use `Interactive`; the collisions
/// button uses `Attention` / `Clear`).
#[component]
pub fn ToolbarButtonSurface(props: ToolbarButtonSurfaceProps) -> Element {
    let glyph = ToolbarButtonIconProps::from(&props);
    let class = style::class(props.state);
    rsx! {
        button {
            class,
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
