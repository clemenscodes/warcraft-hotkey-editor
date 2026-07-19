mod model;
mod view;

pub use view::FilterGroupLabelView;
mod style;

use dioxus::prelude::*;
use model::FilterGroupLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilterGroupLabel(props: FilterGroupLabelModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(FilterGroupLabel);
