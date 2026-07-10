mod props;
mod view;

pub use view::CurrentKeyView;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
use props::CurrentKeyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The look for the key currently bound here: a filled gold panel with a steady glow. It
/// owns its own button and shares only the shaped attributes and the conflict tooltip
/// leaf.
#[component]
pub fn CurrentKey(props: CurrentKeyProps) -> Element {
    let CurrentKeyProps {
        label,
        disabled,
        onclick,
        tooltip_text,
        tooltip_placement,
        tooltip_anchor,
    } = props;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled,
            onclick,
            {label}
            Tooltip {
                text: tooltip_text,
                placement: tooltip_placement,
                anchor: tooltip_anchor,
            }
        }
    }
}

assert_component!(CurrentKey);
