mod model;
mod style;
mod view;

pub use view::SearchDialogBodyView;

use dioxus::prelude::*;
use model::SearchDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchDialogBody(props: SearchDialogBodyModel) -> Element {
    let SearchDialogBodyModel {} = props;
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(SearchDialogBody);
