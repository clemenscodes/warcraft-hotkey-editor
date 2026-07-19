mod model;
mod view;

pub use view::RegenLabelView;
mod style;

use dioxus::prelude::*;
use model::RegenLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RegenLabel(props: RegenLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(RegenLabel);
