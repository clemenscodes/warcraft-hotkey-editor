pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::toast_description::{ToastDescription, ToastDescriptionProps};
use components::toast_title::{ToastTitle, ToastTitleProps};
use dioxus::prelude::*;
pub use props::ToastContentProps;
use style::CLASS;
assert_component!(ToastContent);

/// The text column of a toast: its title above its optional description.
#[component]
pub fn ToastContent(props: ToastContentProps) -> Element {
    let title_props = ToastTitleProps::from(&props);
    let description_props = ToastDescriptionProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            ToastTitle { ..title_props }
            ToastDescription { ..description_props }
        }
    }
}
