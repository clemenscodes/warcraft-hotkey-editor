pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescription;
use components::error_toast_title::{ErrorToastTitle, ErrorToastTitleProps};
use dioxus::prelude::*;
pub use props::ErrorToastContentProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ErrorToastContent);

/// The text column of a error toast: its error title above the optional description.
#[component]
pub fn ErrorToastContent(props: ErrorToastContentProps) -> Element {
    let title = ErrorToastTitleProps::from(&props);
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            ErrorToastTitle { ..title }
            ToastDescription { ..description }
        }
    }
}
