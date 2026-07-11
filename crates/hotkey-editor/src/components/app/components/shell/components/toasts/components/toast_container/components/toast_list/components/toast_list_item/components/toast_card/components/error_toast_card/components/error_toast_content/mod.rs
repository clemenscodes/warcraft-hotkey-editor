pub mod components;
mod model;
mod view;

pub use view::ErrorToastContentView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescription;
use components::error_toast_title::ErrorToastTitle;
use dioxus::prelude::*;
use model::ErrorToastContentModel;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a error toast: its error title above the optional description.
#[component]
pub fn ErrorToastContent(props: ErrorToastContentModel) -> Element {
    let title = props.title;
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            ErrorToastTitle { title }
            ToastDescription { description }
        }
    }
}

assert_component!(ErrorToastContent);
