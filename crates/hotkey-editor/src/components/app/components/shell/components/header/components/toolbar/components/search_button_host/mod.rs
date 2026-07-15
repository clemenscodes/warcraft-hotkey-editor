pub mod components;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::search_dialog::SearchDialog;
use components::search_button::SearchButton;
use dioxus::prelude::*;
use presentation::use_search_button;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchButtonHost() -> Element {
    let model = use_search_button();
    let aria_expanded = model.aria_expanded;
    let onclick = model.onclick;
    let open = model.open;
    let on_open_change = model.on_open_change;
    rsx! {
        div {
            class: CLASS,
            SearchButton {
                aria_expanded,
                onclick,
            }
            SearchDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(SearchButtonHost);
