mod model;
mod style;
mod view;

pub use view::EmptyUnitPositionDetailView;

use dioxus::prelude::*;
use model::EmptyUnitPositionDetailModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EmptyUnitPositionDetail(props: EmptyUnitPositionDetailModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            p {


                {props.prompt}
            }
        }
    }
}

assert_component!(EmptyUnitPositionDetail);
