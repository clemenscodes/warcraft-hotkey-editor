pub mod components;
mod model;
mod view;

pub use view::EditableKeycapView;
mod state;

use components::editing_keycap::EditingKeycap;
use components::idle_keycap::IdleKeycap;
use dioxus::prelude::*;
use tw_macro::assert_component;

use model::EditableKeycapModel;
pub use state::EditableKeycapState;

/// The shared gold key-cap surface worn by both editable hotkey cells: the editor's
/// override key and the layout-grid tile. A pure dispatcher: from the cap's state it
/// renders the resting cap (`IdleKeycap`) or the pulsing one (`EditingKeycap`), each of
/// which owns its own classed root. It carries no class of its own. It is presentational:
/// the host owns size, focus, drag, and the click handler; the look-children only render
/// the look, so the gallery can render it with any glyph, radius, and pulse state.
#[component]
pub fn EditableKeycap(props: EditableKeycapModel) -> Element {
    match props.state {
        EditableKeycapState::Editing => {
            let label = props.label.clone();
            rsx! {
                EditingKeycap { label }
            }
        }
        EditableKeycapState::Idle => {
            let label = props.label.clone();
            rsx! {
                IdleKeycap { label }
            }
        }
    }
}

assert_component!(EditableKeycap);
