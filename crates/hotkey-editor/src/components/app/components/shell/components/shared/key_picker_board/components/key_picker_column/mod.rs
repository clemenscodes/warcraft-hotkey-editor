pub mod components;
mod props;
mod view;

pub use view::KeyPickerColumnView;
mod style;

use components::key_picker_row::KeyPickerRow;
use dioxus::prelude::*;
use props::KeyPickerColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// One column of the picker board: a vertical stack of key rows. The letter picker
/// renders a single column; the system keyboard renders the main keyboard and the
/// numpad as two columns side by side.
#[component]
pub fn KeyPickerColumn(props: KeyPickerColumnProps) -> Element {
    let rows = props.rows;
    let on_pick = props.on_pick;
    rsx! {
        div { class: CLASS,
            for keys in rows {
                KeyPickerRow { keys, on_pick }
            }
        }
    }
}

assert_component!(KeyPickerColumn);
