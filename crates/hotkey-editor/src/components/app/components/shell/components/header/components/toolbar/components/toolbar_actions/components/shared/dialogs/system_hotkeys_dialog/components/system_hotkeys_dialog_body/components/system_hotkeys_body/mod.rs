pub mod components;
mod presentation;
mod style;

use components::category_hotkeys::CategoryHotkeys;
use dioxus::prelude::*;
use presentation::{SystemHotkeysBodyPresentation, use_system_hotkeys_body};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SystemHotkeysBody() -> Element {
    let SystemHotkeysBodyPresentation { category } = use_system_hotkeys_body();
    rsx! {
        div {
            class: CLASS,
            CategoryHotkeys {
                category,
            }
        }
    }
}

assert_component!(SystemHotkeysBody);
