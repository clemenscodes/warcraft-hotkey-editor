pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescription;
use components::warning_toast_title::{WarningToastTitle, WarningToastTitleProps};
use dioxus::prelude::*;
pub use props::WarningToastContentProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(WarningToastContent);

/// The text column of a warning toast: its warning title above the optional description.
#[component]
pub fn WarningToastContent(props: WarningToastContentProps) -> Element {
    let title = WarningToastTitleProps::from(&props);
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            WarningToastTitle { ..title }
            ToastDescription { ..description }
        }
    }
}
