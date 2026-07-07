pub mod components;
mod props;

use components::toast_container::{ToastContainer, ToastContainerProps};
use dioxus::prelude::*;
use dioxus_kit::toast::use_toast_provider;

pub use dioxus_kit::toast::{
    ToastOptions, ToastRecord, ToastType, Toasts, consume_toast, use_toast,
};
pub use props::ToastsProps;

/// Provides the toast queue to its subtree and renders the fixed toast overlay.
/// Its own markup is pure layout glue — the app subtree plus the toast
/// container — so it carries no class of its own; every styled element below is
/// its own component.
use tw_macro::assert_component;
assert_component!(Toasts);
#[component]
pub fn Toasts(props: ToastsProps) -> Element {
    let children = props.children;
    let model = use_toast_provider();
    let container_props = ToastContainerProps::from(&model);
    rsx! {
        {children}
        ToastContainer { ..container_props }
    }
}
