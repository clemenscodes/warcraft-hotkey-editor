pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastClose;
use components::error_toast_content::{ErrorToastContent, ErrorToastContentProps};
use components::error_toast_icon::ErrorToastIcon;
use dioxus::prelude::*;
pub use props::ErrorToastCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ErrorToastCard);

/// The error toast card: its tinted surface owning the alertdialog root, the
/// error icon and title, the description, and the close control. Presentational
/// only; the dispatcher builds its props from the toast record.
#[component]
pub fn ErrorToastCard(props: ErrorToastCardProps) -> Element {
    let content = ErrorToastContentProps::from(&props);
    let close = props.close;
    rsx! {
        div {
            class: CLASS,
            role: "alertdialog",
            "aria-modal": "false",
            tabindex: "0",
            ErrorToastIcon {}
            ErrorToastContent { ..content }
            ToastClose { ..close }
        }
    }
}
