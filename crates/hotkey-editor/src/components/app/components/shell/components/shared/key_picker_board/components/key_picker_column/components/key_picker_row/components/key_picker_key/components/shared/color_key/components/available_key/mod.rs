mod props;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::AvailableKeyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The look for a free, pickable key: gold-bordered, brightening on hover. It owns its
/// own button and shares only the shaped attributes and the conflict tooltip leaf.
#[component]
pub fn AvailableKey(props: AvailableKeyProps) -> Element {
    let AvailableKeyProps {
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

assert_component!(AvailableKey);
