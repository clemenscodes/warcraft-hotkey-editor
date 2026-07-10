mod props;
mod view;

pub use view::PassiveHotkeyBadgeView;
mod style;

use dioxus::prelude::*;
use props::PassiveHotkeyBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

/// The hotkey badge for a passive ability: a muted letter on a mid-panel chip.
#[component]
pub fn PassiveHotkeyBadge(props: PassiveHotkeyBadgeProps) -> Element {
    let label = props.letter.display_label();
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(PassiveHotkeyBadge);
