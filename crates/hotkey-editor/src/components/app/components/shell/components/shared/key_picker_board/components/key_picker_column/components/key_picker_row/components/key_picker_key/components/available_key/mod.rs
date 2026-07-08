mod props;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::AvailableKeyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AvailableKey);

/// The look for a free, pickable key: gold-bordered, brightening on hover. It owns its
/// own button and shares only the shaped attributes and the conflict tooltip leaf.
#[component]
pub fn AvailableKey(props: AvailableKeyProps) -> Element {
    let AvailableKeyProps {
        label,
        data_label,
        data_wide,
        disabled,
        onclick,
        tooltip,
    } = props;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled,
            "data-wide": data_wide,
            "data-label": data_label,
            onclick,
            {label}
            Tooltip { ..tooltip }
        }
    }
}
