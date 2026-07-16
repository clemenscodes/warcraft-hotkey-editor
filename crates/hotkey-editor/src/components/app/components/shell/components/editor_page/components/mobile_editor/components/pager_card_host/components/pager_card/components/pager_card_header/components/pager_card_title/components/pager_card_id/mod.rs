mod model;
mod view;

pub use view::PagerCardIdView;
mod style;

use dioxus::prelude::*;
use model::PagerCardIdModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardId(props: PagerCardIdModel) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(PagerCardId);
