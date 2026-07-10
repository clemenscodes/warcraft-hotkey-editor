mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::{HotkeyBadge, HotkeyBadgeProps};
use dioxus::prelude::*;
pub use props::TileBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(TileBadge);
