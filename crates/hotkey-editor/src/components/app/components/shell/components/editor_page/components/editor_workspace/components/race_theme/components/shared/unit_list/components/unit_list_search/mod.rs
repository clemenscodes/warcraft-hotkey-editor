pub mod components;
mod props;
mod view;

pub use view::UnitListSearchView;
mod style;

use dioxus::prelude::*;

use components::unit_list_search_icon::UnitListSearchIcon;
use components::unit_list_search_input::UnitListSearchInput;
use style::CLASS;
use tw_macro::assert_component;

use props::UnitListSearchProps;

/// The unit list's search box: a magnifier icon (mobile only) over the query input.
#[component]
pub fn UnitListSearch(props: UnitListSearchProps) -> Element {
    let value = props.value;
    let placeholder = props.placeholder;
    let on_input = props.on_input;
    let on_keydown = props.on_keydown;
    rsx! {
        div {
            class: CLASS,
            UnitListSearchIcon {}
            UnitListSearchInput { value, placeholder, on_input, on_keydown }
        }
    }
}

assert_component!(UnitListSearch);
