mod model;
mod view;

pub use view::RaceTabLabelView;
mod style;

use dioxus::prelude::*;
use model::RaceTabLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceTabLabel(props: RaceTabLabelModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(RaceTabLabel);
