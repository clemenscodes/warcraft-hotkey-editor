mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(InventoryEmptySlot);

/// A vacant inventory cell.
#[component]
pub fn InventoryEmptySlot() -> Element {
    rsx! {
        div { class: CLASS, "—" }
    }
}
