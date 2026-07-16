mod model;
mod view;

pub use view::PagerSpacerView;
mod style;

use dioxus::prelude::*;
use model::PagerSpacerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerSpacer(props: PagerSpacerModel) -> Element {
    let height_px = props.height_px;
    let inline_height = format!("height: {height_px}px;");
    rsx! {
        div {
            class: CLASS,
            style: inline_height,
        }
    }
}

assert_component!(PagerSpacer);
