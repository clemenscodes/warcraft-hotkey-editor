pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::toast_close::{ToastClose, ToastCloseProps};
use components::toast_content::{ToastContent, ToastContentProps};
use components::toast_icon::{ToastIcon, ToastIconProps};
use dioxus::prelude::*;
use hooks::use_toast_auto_dismiss;
use logic::ToastCardPresentation;
pub use props::ToastCardProps;
assert_component!(ToastCard);

/// A single toast: its type-tinted card, the type icon, the title/description
/// content, and the close button. Auto-dismisses after its duration unless
/// permanent.
#[component]
pub fn ToastCard(props: ToastCardProps) -> Element {
    use_toast_auto_dismiss(&props);
    let ToastCardPresentation { class, data_type } = ToastCardPresentation::from(&props);
    let icon_props = ToastIconProps::from(&props);
    let content_props = ToastContentProps::from(&props);
    let close_props = ToastCloseProps::from(&props);
    rsx! {
        div {
            class,
            role: "alertdialog",
            "data-type": data_type,
            "aria-modal": "false",
            tabindex: "0",
            ToastIcon { ..icon_props }
            ToastContent { ..content_props }
            ToastClose { ..close_props }
        }
    }
}
