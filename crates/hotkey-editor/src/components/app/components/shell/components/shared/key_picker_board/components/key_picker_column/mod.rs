pub mod components;
mod model;
mod view;

pub use view::KeyPickerColumnView;
mod style;

use components::key_picker_row::KeyPickerRow;
use dioxus::prelude::*;
use model::KeyPickerColumnModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn KeyPickerColumn(props: KeyPickerColumnModel) -> Element {
    let rows = props.rows;
    let on_pick = props.on_pick;
    rsx! {
        div {
            class: CLASS,
            for keys in rows {
                KeyPickerRow {
                    keys,
                    on_pick,
                }
            }
        }
    }
}

assert_component!(KeyPickerColumn);
