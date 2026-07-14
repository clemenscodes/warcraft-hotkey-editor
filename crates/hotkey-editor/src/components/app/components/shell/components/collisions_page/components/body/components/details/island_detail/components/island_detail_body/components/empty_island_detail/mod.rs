mod model;
mod style;
mod view;

pub use view::EmptyIslandDetailView;

use dioxus::prelude::*;
use model::EmptyIslandDetailModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EmptyIslandDetail(props: EmptyIslandDetailModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            p {


                {props.prompt}
            }
        }
    }
}

assert_component!(EmptyIslandDetail);
