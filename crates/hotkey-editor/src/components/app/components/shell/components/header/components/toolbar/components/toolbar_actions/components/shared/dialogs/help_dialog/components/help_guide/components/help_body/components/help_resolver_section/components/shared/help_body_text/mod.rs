mod model;
mod view;

pub use view::HelpBodyTextView;
mod style;

use dioxus::prelude::*;
use model::HelpBodyTextModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpBodyText(props: HelpBodyTextModel) -> Element {
    let text = props.text.clone();
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(HelpBodyText);
