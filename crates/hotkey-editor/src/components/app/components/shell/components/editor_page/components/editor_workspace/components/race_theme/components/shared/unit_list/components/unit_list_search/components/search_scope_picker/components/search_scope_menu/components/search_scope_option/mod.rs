pub mod components;
mod model;
mod view;

pub use view::SearchScopeOptionView;

use components::active_search_scope_option::ActiveSearchScopeOption;
use components::idle_search_scope_option::IdleSearchScopeOption;
use dioxus::prelude::*;
use model::SearchScopeOptionModel;
use tw_macro::assert_component;

#[component]
pub fn SearchScopeOption(props: SearchScopeOptionModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    if props.is_active {
        rsx! {
            ActiveSearchScopeOption {
                label,
                onclick,
            }
        }
    } else {
        rsx! {
            IdleSearchScopeOption {
                label,
                onclick,
            }
        }
    }
}

assert_component!(SearchScopeOption);
