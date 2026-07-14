mod model;
mod view;

pub use view::LinkMoveNameView;
mod style;

use dioxus::prelude::*;
use model::LinkMoveNameModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn LinkMoveName(props: LinkMoveNameModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(LinkMoveName);
