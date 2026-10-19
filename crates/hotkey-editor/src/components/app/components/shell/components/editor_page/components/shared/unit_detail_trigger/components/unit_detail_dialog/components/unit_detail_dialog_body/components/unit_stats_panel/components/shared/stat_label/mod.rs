mod model;
mod view;

pub use view::StatLabelView;
mod style;

use dioxus::prelude::*;
use model::StatLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn StatLabel(props: StatLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(StatLabel);
