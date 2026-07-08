pub mod components;
mod hooks;
mod props;
mod style;

use crate::components::app::components::shell::components::toasts::ToastType;
use components::error_toast_card::ErrorToastCard;
use components::info_toast_card::InfoToastCard;
use components::success_toast_card::SuccessToastCard;
use components::warning_toast_card::WarningToastCard;
use dioxus::prelude::*;
use hooks::use_toast_auto_dismiss;
pub use props::ToastCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ToastCard);

/// A single toast. Auto-dismisses after its duration unless permanent, then routes
/// the toast's type to the matching per-kind card, each of which owns that type's
/// surface look.
#[component]
pub fn ToastCard(props: ToastCardProps) -> Element {
    use_toast_auto_dismiss(&props);
    let record = props.record;
    let on_remove = props.on_remove;
    let toast_type = record.toast_type();
    rsx! {
        div {
            class: CLASS,
            {
                match toast_type {
                    ToastType::Success => rsx! { SuccessToastCard { record, on_remove } },
                    ToastType::Error => rsx! { ErrorToastCard { record, on_remove } },
                    ToastType::Warning => rsx! { WarningToastCard { record, on_remove } },
                    ToastType::Info => rsx! { InfoToastCard { record, on_remove } },
                }
            }
        }
    }
}
