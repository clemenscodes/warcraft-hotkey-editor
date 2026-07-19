mod model;
mod view;

pub use view::RaceScopeBackButtonView;
mod style;

use dioxus::prelude::*;
use model::RaceScopeBackButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceScopeBackButton(props: RaceScopeBackButtonModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            span {
                aria_hidden: "true",
                "◂"
            }
            "Back"
        }
    }
}

assert_component!(RaceScopeBackButton);
