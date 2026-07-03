pub mod components;
mod context;
mod hooks;
mod props;

use components::toast_container::{ToastContainer, ToastContainerProps};
use dioxus::prelude::*;
use hooks::use_toast_provider;

pub use context::{ToastOptions, ToastRecord, ToastType, Toasts, consume_toast, use_toast};
pub use props::ToastMountProps;

/// Provides the toast queue to its subtree and renders the fixed toast overlay.
/// Its own markup is pure layout glue — the app subtree plus the toast
/// container — so it carries no class of its own; every styled element below is
/// its own component.
#[component]
pub fn ToastMount(props: ToastMountProps) -> Element {
    let children = props.children;
    let model = use_toast_provider();
    let container_props = ToastContainerProps::from(&model);
    rsx! {
        {children}
        ToastContainer { ..container_props }
    }
}
