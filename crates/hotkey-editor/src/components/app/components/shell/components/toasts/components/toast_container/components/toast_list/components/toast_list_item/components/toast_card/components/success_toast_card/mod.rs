pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastClose;
use components::success_toast_content::{SuccessToastContent, SuccessToastContentProps};
use components::success_toast_icon::SuccessToastIcon;
use dioxus::prelude::*;
pub use props::SuccessToastCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SuccessToastCard);

/// The success toast card: its tinted surface owning the alertdialog root, the
/// success icon and title, the description, and the close control. Presentational
/// only; the dispatcher builds its props from the toast record.
#[component]
pub fn SuccessToastCard(props: SuccessToastCardProps) -> Element {
    let content = SuccessToastContentProps::from(&props);
    let close = props.close;
    rsx! {
        div {
            class: CLASS,
            role: "alertdialog",
            "data-type": "success",
            "aria-modal": "false",
            tabindex: "0",
            SuccessToastIcon {}
            SuccessToastContent { ..content }
            ToastClose { ..close }
        }
    }
}
