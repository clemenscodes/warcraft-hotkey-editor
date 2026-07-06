mod props;
mod style;

use super::hotkey_badge::{HotkeyBadge, HotkeyBadgeProps};
use dioxus::prelude::*;
pub use props::TileBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TileBadge);

/// The hotkey badge's placement inside a tile: pinned to the top-right corner.
/// Shared by both the filled and empty tiles.
#[component]
pub fn TileBadge(props: TileBadgeProps) -> Element {
    let badge = HotkeyBadgeProps::from(&props);
    rsx! {
        div { class: CLASS,
            HotkeyBadge { ..badge }
        }
    }
}
