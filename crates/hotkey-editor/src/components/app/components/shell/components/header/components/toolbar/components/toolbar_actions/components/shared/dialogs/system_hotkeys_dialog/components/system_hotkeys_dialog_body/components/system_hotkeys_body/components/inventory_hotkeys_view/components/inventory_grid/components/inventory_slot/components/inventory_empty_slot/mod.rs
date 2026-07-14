mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// A vacant inventory cell.
#[component]
pub fn InventoryEmptySlot() -> Element {
    rsx! {
        div {
            class: CLASS,
            "—"
        }
    }
}

assert_component!(InventoryEmptySlot);
