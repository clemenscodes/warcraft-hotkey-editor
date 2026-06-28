mod props;
mod style;

use dioxus::prelude::*;

use crate::components::command_grid::{HotkeyBadge, HotkeyBadgeProps};
use style::FOLLOWER_BADGE_STYLES;

pub use props::FollowerBadgeProps;

#[component]
pub fn FollowerBadge(props: FollowerBadgeProps) -> Element {
    let badge = HotkeyBadgeProps::from(&props);
    rsx! {
        document::Stylesheet { href: FOLLOWER_BADGE_STYLES }
        div { class: "follower-badge",
            HotkeyBadge { ..badge }
        }
    }
}
