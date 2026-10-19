pub mod components;
mod model;
mod view;

pub use view::ToastOverlayView;
mod style;

use components::toast_list::ToastList;
use dioxus::prelude::*;
use model::ToastOverlayModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToastOverlay(props: ToastOverlayModel) -> Element {
    let toasts = props.toasts;
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            role: "region",
            "aria-label": "notifications",
            tabindex: "-1",
            ToastList {
                toasts,
                on_remove,
            }
        }
    }
}

assert_component!(ToastOverlay);
