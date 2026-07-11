mod model;
mod view;

pub use view::UnitNameView;
mod style;

use dioxus::prelude::*;
use model::UnitNameModel;
use style::CLASS;
use tw_macro::assert_component;

/// The unit's name, in gold Friz Quadrata.
#[component]
pub fn UnitName(props: UnitNameModel) -> Element {
    let text = props.text;
    rsx! {
        h2 {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(UnitName);
