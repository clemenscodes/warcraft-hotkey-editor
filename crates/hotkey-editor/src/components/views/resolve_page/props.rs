use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The Resolve page inputs: the loaded keys and the selected move-category
/// breadcrumb (backed by the `?entry=` URL parameter so the viewed section
/// deep-links and survives browser back/forward — mirroring the collisions page).
#[derive(Props, Clone, PartialEq)]
pub struct ResolvePageProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub selected_move_category: Signal<Option<String>>,
}
