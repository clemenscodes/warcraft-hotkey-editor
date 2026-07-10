mod props;
mod style;

use dioxus::prelude::*;
pub use props::PlainSlotKeyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PlainSlotKey);

/// The plain gold key glyph shown when the slot's binding is not in conflict.
/// Presentational — the `SystemSlotKey` dispatcher renders it for the non-conflict
/// look.
#[component]
pub fn PlainSlotKey(props: PlainSlotKeyProps) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}
