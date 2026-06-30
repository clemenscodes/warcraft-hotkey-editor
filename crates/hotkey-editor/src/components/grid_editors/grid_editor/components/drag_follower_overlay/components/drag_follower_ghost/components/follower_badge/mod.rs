mod props;
mod style;

use dioxus::prelude::*;

use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::components::hotkey_badge::{HotkeyBadge, HotkeyBadgeProps};
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
