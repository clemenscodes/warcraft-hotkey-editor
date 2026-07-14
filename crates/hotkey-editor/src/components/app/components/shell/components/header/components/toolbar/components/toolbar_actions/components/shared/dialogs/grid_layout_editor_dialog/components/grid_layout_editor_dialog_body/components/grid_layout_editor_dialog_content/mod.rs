pub mod components;
mod model;
mod view;

pub use view::GridLayoutEditorDialogContentView;
mod style;

use components::apply_button::ApplyButton;
use components::layout_grid::LayoutGrid;
use components::layout_intro::LayoutIntro;
use components::move_hotkey_toggle::MoveHotkeyToggle;
use dioxus::prelude::*;
use model::GridLayoutEditorDialogContentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridLayoutEditorDialogContent(props: GridLayoutEditorDialogContentModel) -> Element {
    let cells = props.cells;
    let toggle_checked = props.toggle_checked;
    let on_toggle = props.on_toggle;
    let on_apply = props.on_apply;
    rsx! {
        div {
            class: CLASS,
            LayoutIntro {



            }
            LayoutGrid {
                cells,
            }
            MoveHotkeyToggle {
                checked: toggle_checked,
                on_toggle,
            }
            ApplyButton {
                on_apply,
            }
        }
    }
}

assert_component!(GridLayoutEditorDialogContent);
