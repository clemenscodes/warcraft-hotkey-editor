mod model;
mod view;

pub use view::PagerCardNameView;
mod style;

use dioxus::prelude::*;
use model::PagerCardNameModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardName(props: PagerCardNameModel) -> Element {
    let name = props.name;
    rsx! {
        span {
            class: CLASS,
            {name}
        }
    }
}

assert_component!(PagerCardName);
