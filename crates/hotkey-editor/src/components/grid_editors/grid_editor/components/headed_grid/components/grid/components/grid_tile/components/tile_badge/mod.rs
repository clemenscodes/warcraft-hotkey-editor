mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use super::hotkey_badge::{HotkeyBadge, HotkeyBadgeProps};

pub use props::TileBadgeProps;

assert_component!(TileBadge);

/// The hotkey badge's placement inside a tile: pinned to the top-right corner.
/// Shared by both the filled and empty tiles.
#[component]
pub fn TileBadge(props: TileBadgeProps) -> Element {
    let badge = HotkeyBadgeProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            HotkeyBadge { ..badge }
        }
    }
}
