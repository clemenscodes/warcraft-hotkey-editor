pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastClose;
use components::warning_toast_content::{WarningToastContent, WarningToastContentProps};
use components::warning_toast_icon::WarningToastIcon;
use dioxus::prelude::*;
pub use props::WarningToastCardProps;
use style::CLASS;
use tw_macro::assert_component;

/// The warning toast card: its tinted surface owning the alertdialog root, the
/// warning icon and title, the description, and the close control. Presentational
/// only; the dispatcher builds its props from the toast record.
#[component]
pub fn WarningToastCard(props: WarningToastCardProps) -> Element {
    let content = WarningToastContentProps::from(&props);
    let close = props.close;
    rsx! {
        div {
            class: CLASS,
            role: "alertdialog",
            "aria-modal": "false",
            tabindex: "0",
            WarningToastIcon {}
            WarningToastContent { ..content }
            ToastClose { ..close }
        }
    }
}

assert_component!(WarningToastCard);
