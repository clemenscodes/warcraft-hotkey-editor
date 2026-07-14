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

#[component]
pub fn EditableKeycap(props: EditableKeycapModel) -> Element {
    match props.state {
        EditableKeycapState::Editing => {
            let label = props.label.clone();
            rsx! {
                EditingKeycap {
                    label,
                }
            }
        }
        EditableKeycapState::Idle => {
            let label = props.label.clone();
            rsx! {
                IdleKeycap {
                    label,
                }
            }
        }
    }
}

assert_component!(EditableKeycap);
