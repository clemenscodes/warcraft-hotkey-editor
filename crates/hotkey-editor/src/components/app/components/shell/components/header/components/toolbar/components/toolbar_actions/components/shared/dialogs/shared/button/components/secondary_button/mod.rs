mod model;
mod view;

pub use view::SecondaryButtonView;
mod style;

use dioxus::prelude::*;
use model::SecondaryButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SecondaryButton(props: SecondaryButtonModel) -> Element {
    let onclick = props.onclick;
    let label = props.label;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
        }
    }
}

assert_component!(SecondaryButton);
