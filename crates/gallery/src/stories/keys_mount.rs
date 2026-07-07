use dioxus::prelude::*;
use hotkey_editor::services::customkeys::service::CustomKeysService;
use warcraft_keybinds::CustomKeys;

/// App-specific story decorator: provides a [`CustomKeysService`] over the
/// story's own `loaded_keys` signal, so a behaviour story can exercise a
/// component that mutates the document and see the result update. The
/// presentational majority needs no such wrapper — it takes its slice of the
/// document as props and renders directly. Lives with the stories, not in the
/// generic gallery framework, which stays domain-agnostic. This is the analogue
/// of a Storybook decorator, mirroring `Toasts` for the toast context.
#[component]
pub fn CustomKeysMount(loaded_keys: Signal<Option<CustomKeys>>, children: Element) -> Element {
    use_context_provider(|| CustomKeysService::new(loaded_keys));
    rsx! {
        {children}
    }
}
