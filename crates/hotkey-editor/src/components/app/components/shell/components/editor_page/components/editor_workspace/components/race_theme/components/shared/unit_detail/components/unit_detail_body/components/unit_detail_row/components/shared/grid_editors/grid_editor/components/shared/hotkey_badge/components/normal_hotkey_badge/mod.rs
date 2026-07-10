mod props;
mod style;

use dioxus::prelude::*;
pub use props::NormalHotkeyBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

/// The hotkey badge for an ordinary binding: the warcraft-gold letter on a dark chip.
#[component]
pub fn NormalHotkeyBadge(props: NormalHotkeyBadgeProps) -> Element {
    let label = props.letter.display_label();
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(NormalHotkeyBadge);
