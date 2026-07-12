pub mod components;
mod model;
mod view;

pub use view::InfoToastContentView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_overlay::components::toast_list::components::toast_list_item::components::toast::components::shared::toast_description::ToastDescription;
use components::info_toast_title::InfoToastTitle;
use dioxus::prelude::*;
use model::InfoToastContentModel;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a info toast: its info title above the optional description.
#[component]
pub fn InfoToastContent(props: InfoToastContentModel) -> Element {
    let title = props.title;
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            InfoToastTitle { title }
            ToastDescription { description }
        }
    }
}

assert_component!(InfoToastContent);
