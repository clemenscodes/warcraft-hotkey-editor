mod model;
mod view;

pub use view::HelpDismissView;
mod style;

use dioxus::prelude::*;
use model::HelpDismissModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpDismiss(props: HelpDismissModel) -> Element {
    let on_dismiss = props.on_dismiss;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick: on_dismiss,
            "Got it, don't show this again"
        }
    }
}

assert_component!(HelpDismiss);
