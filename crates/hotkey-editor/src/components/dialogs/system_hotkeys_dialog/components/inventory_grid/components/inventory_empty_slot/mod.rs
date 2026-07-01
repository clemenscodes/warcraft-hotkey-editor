mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

assert_component!(InventoryEmptySlot);

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
