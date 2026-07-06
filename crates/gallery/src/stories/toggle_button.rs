use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButton;

pub fn stories() -> Vec<Story> {
    vec![
        Story::new(
            "ToggleButton",
            "ToggleButton",
            "Active",
            toggle_button_active,
        ),
        Story::new(
            "ToggleButton",
            "ToggleButton",
            "Inactive",
            toggle_button_inactive,
        ),
        Story::new("ToggleButton", "ToggleButton", "Pair", toggle_button_pair),
    ]
}

// The button fills the box its parent hands it, so each story wraps it in a small
// sized flex box — the same role the mode / search / catalog groups play in the app.
fn toggle_button_active() -> Element {
    rsx! {
        div {
            style: "display:flex;width:18rem;height:3.5rem;",
            ToggleButton {
                label: "Melee",
                active: true,
                onclick: |_| {},
            }
        }
    }
}

fn toggle_button_inactive() -> Element {
    rsx! {
        div {
            style: "display:flex;width:18rem;height:3.5rem;",
            ToggleButton {
                label: "Campaign",
                active: false,
                onclick: |_| {},
            }
        }
    }
}

fn toggle_button_pair() -> Element {
    rsx! {
        div {
            style: "display:flex;gap:0.5rem;width:22rem;height:3.5rem;",
            ToggleButton {
                label: "Unit",
                active: true,
                onclick: |_| {},
            }
            ToggleButton {
                label: "Ability",
                active: false,
                onclick: |_| {},
            }
        }
    }
}
