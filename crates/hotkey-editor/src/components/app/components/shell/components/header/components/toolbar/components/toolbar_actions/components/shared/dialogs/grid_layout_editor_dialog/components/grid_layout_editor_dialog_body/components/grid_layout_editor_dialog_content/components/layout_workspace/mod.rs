pub mod components;
mod model;
mod view;

pub use view::LayoutWorkspaceView;
mod style;

use components::layout_grid::LayoutGrid;
use components::move_hotkey_toggle::MoveHotkeyToggle;
use dioxus::prelude::*;
use model::LayoutWorkspaceModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn LayoutWorkspace(props: LayoutWorkspaceModel) -> Element {
    let cells = props.cells;
    let checked = props.checked;
    let on_toggle = props.on_toggle;
    rsx! {
        div {
            class: CLASS,
            LayoutGrid {
                cells,
            }
            MoveHotkeyToggle {
                checked,
                on_toggle,
            }
        }
    }
}

assert_component!(LayoutWorkspace);
