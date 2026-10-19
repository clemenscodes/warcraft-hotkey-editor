pub mod components;
mod model;
mod view;

pub use view::GridLayoutEditorButtonView;
mod style;

use components::grid_layout_editor_button_icon::GridLayoutEditorButtonIcon;
use components::grid_layout_editor_button_label::GridLayoutEditorButtonLabel;
use dioxus::prelude::*;
use model::GridLayoutEditorButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridLayoutEditorButton(props: GridLayoutEditorButtonModel) -> Element {
    let is_open = props.is_open;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Edit global hotkey layout",
            aria_haspopup: "dialog",
            aria_expanded: is_open,
            onclick,
            GridLayoutEditorButtonIcon {}
            GridLayoutEditorButtonLabel {}
        }
    }
}

assert_component!(GridLayoutEditorButton);
