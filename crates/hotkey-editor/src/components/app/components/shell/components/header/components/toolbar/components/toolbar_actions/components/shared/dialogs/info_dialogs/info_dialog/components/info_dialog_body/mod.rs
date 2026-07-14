pub mod components;
mod model;
mod view;

pub use view::InfoDialogBodyView;
mod style;

use components::info_actions::InfoActions;
use components::info_content::InfoContent;
use dioxus::prelude::*;
use model::InfoDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InfoDialogBody(props: InfoDialogBodyModel) -> Element {
    let intro = props.intro;
    let warning = props.warning;
    let primary_label = props.primary_label;
    let on_primary = props.on_primary;
    let on_cancel = props.on_cancel;
    rsx! {
        div {
            class: CLASS,
            InfoContent {
                intro,
                warning,
            }
            InfoActions {
                primary_label,
                on_primary,
                on_cancel,
            }
        }
    }
}

assert_component!(InfoDialogBody);
