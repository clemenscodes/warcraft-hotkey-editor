pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescription;
use components::success_toast_title::SuccessToastTitle;
use dioxus::prelude::*;
use props::SuccessToastContentProps;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a success toast: its success title above the optional description.
#[component]
pub fn SuccessToastContent(props: SuccessToastContentProps) -> Element {
    let title = props.title;
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            SuccessToastTitle { title }
            ToastDescription { description }
        }
    }
}

assert_component!(SuccessToastContent);
