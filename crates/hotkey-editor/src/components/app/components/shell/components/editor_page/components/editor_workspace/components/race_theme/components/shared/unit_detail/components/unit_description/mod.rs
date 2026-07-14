mod model;
mod view;

pub use view::UnitDescriptionView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::UnitDescriptionModel;

#[component]
pub fn UnitDescription(props: UnitDescriptionModel) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(UnitDescription);
