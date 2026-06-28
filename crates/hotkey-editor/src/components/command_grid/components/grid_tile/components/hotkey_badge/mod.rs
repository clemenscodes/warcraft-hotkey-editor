mod props;
mod state;
mod style;

use dioxus::prelude::*;

use style::HOTKEY_BADGE_STYLES;

pub use props::HotkeyBadgeProps;
pub use state::HotkeyBadgeState;

#[component]
pub fn HotkeyBadge(props: HotkeyBadgeProps) -> Element {
    let HotkeyBadgeProps { letter, state } = props;
    let class_name = state.class();
    rsx! {
        document::Stylesheet { href: HOTKEY_BADGE_STYLES }
        span { class: class_name, {letter} }
    }
}
