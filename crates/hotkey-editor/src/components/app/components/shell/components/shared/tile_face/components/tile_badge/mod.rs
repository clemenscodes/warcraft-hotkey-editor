mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadge;
use dioxus::prelude::*;
use props::TileBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

/// The hotkey badge's placement inside a tile: pinned to the top-right corner.
/// Shared by both the filled and empty tiles.
#[component]
pub fn TileBadge(props: TileBadgeProps) -> Element {
    let letter = props.letter;
    let state = props.state;
    rsx! {
        div { class: CLASS,
            HotkeyBadge {
                letter,
                state,
            }
        }
    }
}

assert_component!(TileBadge);
