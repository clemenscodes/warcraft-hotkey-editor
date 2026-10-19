mod model;
mod view;

pub use view::CollisionKindTabLabelView;
mod style;

use dioxus::prelude::*;
use model::CollisionKindTabLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionKindTabLabel(props: CollisionKindTabLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(CollisionKindTabLabel);
