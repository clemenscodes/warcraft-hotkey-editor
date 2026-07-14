mod model;
mod view;

pub use view::TileBadgeView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::components::shared::hotkey_badge::HotkeyBadge;
use dioxus::prelude::*;
use model::TileBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TileBadge(props: TileBadgeModel) -> Element {
    let letter = props.letter;
    let state = props.state;
    rsx! {
        div {
            class: CLASS,
            HotkeyBadge {
                letter,
                state,
            }
        }
    }
}

assert_component!(TileBadge);
