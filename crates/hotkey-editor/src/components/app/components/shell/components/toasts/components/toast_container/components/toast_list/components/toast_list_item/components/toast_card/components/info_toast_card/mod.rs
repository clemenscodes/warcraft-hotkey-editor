pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastClose;
use components::info_toast_content::{InfoToastContent, InfoToastContentProps};
use components::info_toast_icon::InfoToastIcon;
use dioxus::prelude::*;
pub use props::InfoToastCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(InfoToastCard);

/// The info toast card: its tinted surface owning the alertdialog root, the
/// info icon and title, the description, and the close control. Presentational
/// only; the dispatcher builds its props from the toast record.
#[component]
pub fn InfoToastCard(props: InfoToastCardProps) -> Element {
    let content = InfoToastContentProps::from(&props);
    let close = props.close;
    rsx! {
        div {
            class: CLASS,
            role: "alertdialog",
            "aria-modal": "false",
            tabindex: "0",
            InfoToastIcon {}
            InfoToastContent { ..content }
            ToastClose { ..close }
        }
    }
}
