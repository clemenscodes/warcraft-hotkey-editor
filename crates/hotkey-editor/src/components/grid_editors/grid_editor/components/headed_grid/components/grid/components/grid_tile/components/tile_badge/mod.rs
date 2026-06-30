mod props;
mod style;

use dioxus::prelude::*;

use super::hotkey_badge::{HotkeyBadge, HotkeyBadgeProps};
use style::TILE_BADGE_STYLES;

pub use props::TileBadgeProps;

#[component]
pub fn TileBadge(props: TileBadgeProps) -> Element {
    rsx! {
        document::Stylesheet { href: TILE_BADGE_STYLES }
        div {
            class: "tile-badge",
            HotkeyBadge { ..HotkeyBadgeProps::from(&props) }
        }
    }
}
