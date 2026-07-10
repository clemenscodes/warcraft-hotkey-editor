mod props;
mod style;

use dioxus::prelude::*;
pub use props::ConflictSlotKeyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The danger-red key glyph shown when the slot's binding is in conflict.
/// Presentational — the `SystemSlotKey` dispatcher renders it for the conflict look.
#[component]
pub fn ConflictSlotKey(props: ConflictSlotKeyProps) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(ConflictSlotKey);
