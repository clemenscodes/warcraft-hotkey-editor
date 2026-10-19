mod model;
mod presentation;
mod view;

pub use view::UnitListSearchInputView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::UnitListSearchInputModel;
use presentation::{UnitListSearchInputPresentation, use_unit_list_search_input};

#[component]
pub fn UnitListSearchInput(props: UnitListSearchInputModel) -> Element {
    let UnitListSearchInputPresentation {
        value,
        placeholder,
        on_input,
        on_keydown,
    } = use_unit_list_search_input(&props);
    rsx! {
        input {
            class: CLASS,
            r#type: "search",
            placeholder,
            value,
            oninput: move |event| on_input.call(event),
            onkeydown: move |event| on_keydown.call(event),
        }
    }
}

assert_component!(UnitListSearchInput);
