pub mod components;
mod model;
mod view;

pub use view::WarningToastContentView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_overlay::components::toast_list::components::toast_list_item::components::toast::components::shared::toast_description::ToastDescription;
use components::warning_toast_title::WarningToastTitle;
use dioxus::prelude::*;
use model::WarningToastContentModel;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a warning toast: its warning title above the optional description.
#[component]
pub fn WarningToastContent(props: WarningToastContentModel) -> Element {
    let title = props.title;
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            WarningToastTitle { title }
            ToastDescription { description }
        }
    }
}

assert_component!(WarningToastContent);
