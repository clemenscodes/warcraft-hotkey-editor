pub mod components;
mod model;
mod view;

pub use view::InfoDialogBodyView;
mod style;

use components::info_content::InfoContent;
use dioxus::prelude::*;
use model::InfoDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InfoDialogBody(props: InfoDialogBodyModel) -> Element {
    let intro = props.intro;
    let warning = props.warning;
    rsx! {
        div {
            class: CLASS,
            InfoContent {
                intro,
                warning,
            }
        }
    }
}

assert_component!(InfoDialogBody);
