pub mod components;
mod props;
mod state;

use components::editing_keycap::{EditingKeycap, EditingKeycapProps};
use components::idle_keycap::{IdleKeycap, IdleKeycapProps};
use dioxus::prelude::*;
use tw_macro::assert_component;

pub use props::EditableKeycapProps;
pub use state::EditableKeycapState;

/// The shared gold key-cap surface worn by both editable hotkey cells: the editor's
/// override key and the layout-grid tile. A pure dispatcher: from the cap's state it
/// renders the resting cap (`IdleKeycap`) or the pulsing one (`EditingKeycap`), each of
/// which owns its own classed root. It carries no class of its own. It is presentational:
/// the host owns size, focus, drag, and the click handler; the look-children only render
/// the look, so the gallery can render it with any glyph, radius, and pulse state.
#[component]
pub fn EditableKeycap(props: EditableKeycapProps) -> Element {
    match props.state {
        EditableKeycapState::Editing => {
            let look = EditingKeycapProps::from(&props);
            rsx! {
                EditingKeycap { ..look }
            }
        }
        EditableKeycapState::Idle => {
            let look = IdleKeycapProps::from(&props);
            rsx! {
                IdleKeycap { ..look }
            }
        }
    }
}

assert_component!(EditableKeycap);
