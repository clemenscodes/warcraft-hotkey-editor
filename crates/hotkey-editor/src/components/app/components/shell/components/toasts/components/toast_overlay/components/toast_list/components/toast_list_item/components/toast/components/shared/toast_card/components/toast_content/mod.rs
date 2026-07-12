pub mod components;
mod model;
mod view;

pub use view::ToastContentView;
mod style;

use components::toast_description::ToastDescription;
use components::toast_title::ToastTitle;
use dioxus::prelude::*;
use model::ToastContentModel;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a toast: its title above the optional description. The title
/// tint comes from `--toast-title`, set by the severity wrapper.
#[component]
pub fn ToastContent(props: ToastContentModel) -> Element {
    let title = props.title;
    let description = props.description;
    rsx! {
        div {
            class: CLASS,
            ToastTitle { title }
            ToastDescription { description }
        }
    }
}

assert_component!(ToastContent);
