pub mod components;
mod model;
mod presentation;
mod view;

pub use view::ToastView;

use crate::components::app::components::shell::components::toasts::ToastType;
use components::error_toast_card::ErrorToastCard;
use components::info_toast_card::InfoToastCard;
use components::success_toast_card::SuccessToastCard;
use components::warning_toast_card::WarningToastCard;
use dioxus::prelude::*;
use model::ToastModel;
use presentation::use_toast_auto_dismiss;
use tw_macro::assert_component;

#[component]
pub fn Toast(props: ToastModel) -> Element {
    use_toast_auto_dismiss(&props);
    let toast_type = props.record.toast_type();
    let record = props.record;
    let on_remove = props.on_remove;
    match toast_type {
        ToastType::Success => rsx! {
            SuccessToastCard {
                record,
                on_remove,
            }
        },
        ToastType::Error => rsx! {
            ErrorToastCard {
                record,
                on_remove,
            }
        },
        ToastType::Warning => rsx! {
            WarningToastCard {
                record,
                on_remove,
            }
        },
        ToastType::Info => rsx! {
            InfoToastCard {
                record,
                on_remove,
            }
        },
    }
}

assert_component!(Toast);
