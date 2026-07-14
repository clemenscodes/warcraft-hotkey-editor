pub mod components;
mod model;
mod style;
mod view;

pub use view::HelpFooterView;

use components::help_dismiss::HelpDismiss;
use dioxus::prelude::*;
use model::HelpFooterModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpFooter(props: HelpFooterModel) -> Element {
    let on_dismiss = props.on_dismiss;
    rsx! {
        footer {
            class: CLASS,
            HelpDismiss {
                on_dismiss,
            }
        }
    }
}

assert_component!(HelpFooter);
