pub mod components;
mod model;
mod view;

pub use view::SearchClearView;
mod style;

use components::search_clear_icon::SearchClearIcon;
use dioxus::prelude::*;
use model::SearchClearModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchClear(props: SearchClearModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Clear search",
            onclick,
            SearchClearIcon {}
        }
    }
}

assert_component!(SearchClear);
