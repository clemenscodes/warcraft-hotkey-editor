mod model;
mod view;

pub use view::PlainSlotKeyView;
mod style;

use dioxus::prelude::*;
use model::PlainSlotKeyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The plain gold key glyph shown when the slot's binding is not in conflict.
/// Presentational — the `SystemSlotKey` dispatcher renders it for the non-conflict
/// look.
#[component]
pub fn PlainSlotKey(props: PlainSlotKeyModel) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(PlainSlotKey);
