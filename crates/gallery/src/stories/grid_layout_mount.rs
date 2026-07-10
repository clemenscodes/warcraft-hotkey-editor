use dioxus::prelude::*;
use hotkey_editor::services::grid_layout::service::GridLayoutService;
use warcraft_keybinds::GridLayout;

/// App-specific story decorator: provides a [`GridLayoutService`] over the story's own
/// `grid_layout` signal, so a component that reaches for the app-wide layout service
/// (the layout editor) can render and mutate the selected layout standalone. The
/// presentational majority takes its layout slice as props and needs no such wrapper.
/// Lives with the stories, not in the generic gallery framework, which stays
/// domain-agnostic. Mirrors [`super::keys_mount::CustomKeysMount`].
#[component]
pub fn GridLayoutMount(grid_layout: Signal<GridLayout>, children: Element) -> Element {
    use_context_provider(|| GridLayoutService::new(grid_layout));
    rsx! {
        {children}
    }
}
