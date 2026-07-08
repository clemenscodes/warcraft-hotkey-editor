pub mod components;
mod hooks;
mod logic;
mod props;

use crate::components::app::components::shell::components::toasts::ToastType;
use components::error_toast_card::{ErrorToastCard, ErrorToastCardProps};
use components::info_toast_card::{InfoToastCard, InfoToastCardProps};
use components::success_toast_card::{SuccessToastCard, SuccessToastCardProps};
use components::warning_toast_card::{WarningToastCard, WarningToastCardProps};
use dioxus::prelude::*;
use hooks::use_toast_auto_dismiss;
pub use props::ToastCardProps;
use tw_macro::assert_component;
assert_component!(ToastCard);

/// A single toast, dispatched by its type. The surface tint, icon, and title all
/// differ per kind, so each kind is its own card component. This dispatcher carries
/// no markup of its own: it schedules the auto-dismiss, then renders the one card
/// its type selects. Each per-kind card owns its `role="alertdialog"` root and the
/// shared icon/content/close leaves.
#[component]
pub fn ToastCard(props: ToastCardProps) -> Element {
    use_toast_auto_dismiss(&props);
    let toast_type = props.record.toast_type();
    match toast_type {
        ToastType::Success => {
            let card = SuccessToastCardProps::from(&props);
            rsx! {
                SuccessToastCard { ..card }
            }
        }
        ToastType::Error => {
            let card = ErrorToastCardProps::from(&props);
            rsx! {
                ErrorToastCard { ..card }
            }
        }
        ToastType::Warning => {
            let card = WarningToastCardProps::from(&props);
            rsx! {
                WarningToastCard { ..card }
            }
        }
        ToastType::Info => {
            let card = InfoToastCardProps::from(&props);
            rsx! {
                InfoToastCard { ..card }
            }
        }
    }
}
