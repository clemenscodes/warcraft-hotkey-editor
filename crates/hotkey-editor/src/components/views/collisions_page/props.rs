use crate::services::navigation::app_view::CollisionKind;
use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, GridLayout};

/// The Collisions page inputs. Selection signals live in `app.rs` so they survive
/// leaving the page (a unit click → editor) and ride in the `?entry=` URL param —
/// one per kind, for per-tab memory.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionsPageProps {
    pub kind: CollisionKind,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub selected_island: Signal<Option<String>>,
    pub selected_hotkey_unit: Signal<Option<String>>,
    pub selected_unit_position: Signal<Option<String>>,
}
