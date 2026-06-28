mod props;
mod style;

use dioxus::prelude::*;

pub use props::{HotkeyBadgeProps, HotkeyBadgeState};
use style::HOTKEY_BADGE_STYLES;

#[component]
pub fn HotkeyBadge(props: HotkeyBadgeProps) -> Element {
    rsx! {
        document::Stylesheet { href: HOTKEY_BADGE_STYLES }
        span { class: props.state.class(), {props.letter} }
    }
}
