mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(InventoryEmptySlot);

/// A vacant inventory cell.
#[component]
pub fn InventoryEmptySlot() -> Element {
    rsx! {
        div { class: CLASS, "—" }
    }
}
