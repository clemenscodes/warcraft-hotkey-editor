mod style;

use crate::components::app::components::shell::components::shared::search_dialog_body::SearchDialogBody;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitList() -> Element {
    rsx! {
        aside {
            class: CLASS,
            SearchDialogBody {}
        }
    }
}

assert_component!(UnitList);
