pub mod components;
mod model;
mod view;

pub use view::AltStateSectionView;
mod style;

use components::alt_state_header::AltStateHeader;
use dioxus::prelude::*;
use model::AltStateSectionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AltStateSection(props: AltStateSectionModel) -> Element {
    let AltStateSectionModel {
        alt_name_text,
        show_alt_controls,
        alt_hotkey_label,
        alt_hotkey_is_editing,
        alt_hotkey_is_special_token,
        on_position_click,
        on_hotkey_activate,
    } = props;
    if !show_alt_controls {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            AltStateHeader {
                alt_name_text,
                show: show_alt_controls,
                hotkey_label: alt_hotkey_label,
                is_editing: alt_hotkey_is_editing,
                is_special: alt_hotkey_is_special_token,
                on_position_click,
                on_hotkey_activate,
            }
        }
    }
}

assert_component!(AltStateSection);
