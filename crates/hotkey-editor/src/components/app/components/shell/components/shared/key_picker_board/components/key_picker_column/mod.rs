pub mod components;
mod props;
mod style;

use components::key_picker_row::KeyPickerRow;
use dioxus::prelude::*;
pub use props::KeyPickerColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(KeyPickerColumn);

/// One column of the picker board: a vertical stack of key rows. The letter picker
/// renders a single column; the system keyboard renders the main keyboard and the
/// numpad as two columns side by side.
#[component]
pub fn KeyPickerColumn(props: KeyPickerColumnProps) -> Element {
    let rows = props.rows;
    rsx! {
        div { class: CLASS,
            for row in rows {
                KeyPickerRow { ..row }
            }
        }
    }
}
