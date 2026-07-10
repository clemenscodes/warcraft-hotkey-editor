mod props;
mod view;

pub use view::ConflictHotkeyBadgeView;
mod style;

use dioxus::prelude::*;
use props::ConflictHotkeyBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

/// The hotkey badge for a conflicting binding: a danger-red letter on an orc-tinted chip.
#[component]
pub fn ConflictHotkeyBadge(props: ConflictHotkeyBadgeProps) -> Element {
    let label = props.letter.display_label();
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(ConflictHotkeyBadge);
