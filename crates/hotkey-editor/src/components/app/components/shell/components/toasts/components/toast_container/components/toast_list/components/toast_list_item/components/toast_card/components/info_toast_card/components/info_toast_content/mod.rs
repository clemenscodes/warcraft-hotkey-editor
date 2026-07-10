pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescription;
use components::info_toast_title::{InfoToastTitle, InfoToastTitleProps};
use dioxus::prelude::*;
pub use props::InfoToastContentProps;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a info toast: its info title above the optional description.
#[component]
pub fn InfoToastContent(props: InfoToastContentProps) -> Element {
    let title = InfoToastTitleProps::from(&props);
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            InfoToastTitle { ..title }
            ToastDescription { ..description }
        }
    }
}

assert_component!(InfoToastContent);
