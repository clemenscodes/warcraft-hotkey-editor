mod model;
mod view;

pub use view::UnitCardNameView;
mod style;

use dioxus::prelude::*;
use model::UnitCardNameModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCardName(props: UnitCardNameModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(UnitCardName);
