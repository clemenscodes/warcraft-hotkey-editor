pub mod components;
mod model;
mod view;

pub use view::AltStateHeaderView;
mod style;

use components::alt_state_controls::AltStateControls;
use components::alt_state_header_label_column::AltStateHeaderLabelColumn;
use dioxus::prelude::*;
use model::AltStateHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

/// The off-state block's top row: the label column beside the editable controls.
#[component]
pub fn AltStateHeader(props: AltStateHeaderModel) -> Element {
    let AltStateHeaderModel {
        alt_name_text,
        show,
        hotkey_label,
        is_editing,
        is_special,
        on_position_click,
        on_hotkey_activate,
    } = props;
    rsx! {
        div {
            class: CLASS,
            AltStateHeaderLabelColumn {
                text: alt_name_text,
            }
            AltStateControls {
                show,
                hotkey_label,
                is_editing,
                is_special,
                on_position_click,
                on_hotkey_activate,
            }
        }
    }
}

assert_component!(AltStateHeader);
