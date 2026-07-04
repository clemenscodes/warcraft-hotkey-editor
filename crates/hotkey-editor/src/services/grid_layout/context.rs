use dioxus::prelude::*;
use warcraft_keybinds::GridLayout;

/// Access the app-wide chosen [`GridLayout`] signal provided at the app root. Call
/// from a component or hook body (it is a hook). The selected layout is a global,
/// crate-wide concept — no single component owns it — so this accessor lives in
/// `services/`, not colocated with any component.
pub(crate) fn use_grid_layout() -> Signal<GridLayout> {
    use_context::<Signal<GridLayout>>()
}
