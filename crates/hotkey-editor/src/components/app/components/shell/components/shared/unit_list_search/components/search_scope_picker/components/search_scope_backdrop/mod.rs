mod model;
mod view;

pub use view::SearchScopeBackdropView;
mod style;

use dioxus::prelude::*;
use model::SearchScopeBackdropModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchScopeBackdrop(props: SearchScopeBackdropModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            onclick,
        }
    }
}

assert_component!(SearchScopeBackdrop);
