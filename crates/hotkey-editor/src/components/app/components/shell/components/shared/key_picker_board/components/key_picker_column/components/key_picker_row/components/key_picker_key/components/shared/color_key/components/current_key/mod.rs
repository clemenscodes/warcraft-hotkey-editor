mod props;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::CurrentKeyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CurrentKey);

/// The look for the key currently bound here: a filled gold panel with a steady glow. It
/// owns its own button and shares only the shaped attributes and the conflict tooltip
/// leaf.
#[component]
pub fn CurrentKey(props: CurrentKeyProps) -> Element {
    let CurrentKeyProps {
        label,
        disabled,
        onclick,
        tooltip,
    } = props;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled,
            onclick,
            {label}
            Tooltip { ..tooltip }
        }
    }
}
