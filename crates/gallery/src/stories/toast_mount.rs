use dioxus::prelude::*;
use hotkey_editor::components::app::components::shell::components::toasts::{
    ToastContainer, ToastContainerProps, use_toast_provider,
};

/// App-specific story decorator: provides the toast queue over the story subtree and
/// renders the fixed toast overlay, so a story that fires toasts shows them. It is the
/// analogue of [`super::keys_mount::CustomKeysMount`] for the toast context — an
/// app-specific decorator that lives with the stories, not in the domain-agnostic
/// gallery framework. It does for a story exactly what the app's `Shell` does for the
/// running app.
#[component]
pub fn ToastMount(children: Element) -> Element {
    let model = use_toast_provider();
    let container = ToastContainerProps::from(&model);
    rsx! {
        {children}
        ToastContainer { ..container }
    }
}
