mod model;
mod style;
mod view;

pub use view::EmptyDetailView;

use dioxus::prelude::*;
use model::EmptyDetailModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EmptyDetail(props: EmptyDetailModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            p {


                {props.prompt}
            }
        }
    }
}

assert_component!(EmptyDetail);
