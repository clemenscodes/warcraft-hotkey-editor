mod model;
mod view;

pub use view::AbilityNameView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::AbilityNameModel;

#[component]
pub fn AbilityName(props: AbilityNameModel) -> Element {
    let text = props.text;
    rsx! {
        h3 {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AbilityName);
