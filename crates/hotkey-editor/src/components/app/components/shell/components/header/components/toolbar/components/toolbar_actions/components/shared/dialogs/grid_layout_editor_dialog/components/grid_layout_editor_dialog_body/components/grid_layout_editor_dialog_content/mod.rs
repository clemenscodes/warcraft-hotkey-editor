pub mod components;
mod model;
mod view;

pub use view::GridLayoutEditorDialogContentView;
mod style;

use components::layout_intro::LayoutIntro;
use components::layout_workspace::LayoutWorkspace;
use dioxus::prelude::*;
use model::GridLayoutEditorDialogContentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridLayoutEditorDialogContent(props: GridLayoutEditorDialogContentModel) -> Element {
    let cells = props.cells;
    let toggle_checked = props.toggle_checked;
    let on_toggle = props.on_toggle;
    rsx! {
        div {
            class: CLASS,
            LayoutIntro {}
            LayoutWorkspace {
                cells,
                checked: toggle_checked,
                on_toggle,
            }
        }
    }
}

assert_component!(GridLayoutEditorDialogContent);
