pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::unit_list_search_icon::UnitListSearchIcon;
use components::unit_list_search_input::UnitListSearchInput;
use style::CLASS;

pub use props::UnitListSearchProps;

assert_component!(UnitListSearch);

/// The unit list's search box: a magnifier icon (mobile only) over the query input.
#[component]
pub fn UnitListSearch(props: UnitListSearchProps) -> Element {
    let value = props.value;
    let placeholder = props.placeholder;
    let on_input = props.on_input;
    let on_keydown = props.on_keydown;
    rsx! {
        div { class: CLASS,
            UnitListSearchIcon {}
            UnitListSearchInput { value, placeholder, on_input, on_keydown }
        }
    }
}
