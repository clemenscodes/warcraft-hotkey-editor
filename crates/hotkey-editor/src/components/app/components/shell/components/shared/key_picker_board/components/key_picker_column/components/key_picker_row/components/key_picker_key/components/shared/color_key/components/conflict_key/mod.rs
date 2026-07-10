mod props;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
use props::ConflictKeyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The look for a key already taken by another binding: orc-red, escalating to danger on
/// hover. It owns its own button and shares only the shaped attributes and the conflict
/// tooltip leaf, which carries the "already used by" message.
#[component]
pub fn ConflictKey(props: ConflictKeyProps) -> Element {
    let ConflictKeyProps {
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

assert_component!(ConflictKey);
