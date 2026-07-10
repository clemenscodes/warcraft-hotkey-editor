pub mod components;
mod logic;
mod props;
mod state;

use components::available_key::{AvailableKey, AvailableKeyProps};
use components::conflict_key::{ConflictKey, ConflictKeyProps};
use components::current_key::{CurrentKey, CurrentKeyProps};
use dioxus::prelude::*;
pub use props::ColorKeyProps;
pub use state::ColorKeyState;
use tw_macro::assert_component;

/// A picker key's color look, chosen from its state. A pure dispatcher that carries no
/// look of its own: from the key's visual state it renders the matching color
/// (`AvailableKey` xor `CurrentKey` xor `ConflictKey`), each of which owns its own
/// button, color styling, and conflict tooltip. Its width is owned by the slot that
/// renders it; this only chooses the color and fills the box it is given.
#[component]
pub fn ColorKey(props: ColorKeyProps) -> Element {
    match props.state {
        ColorKeyState::Available => {
            let available = AvailableKeyProps::from(&props);
            rsx! {
                AvailableKey { ..available }
            }
        }
        ColorKeyState::Current => {
            let current = CurrentKeyProps::from(&props);
            rsx! {
                CurrentKey { ..current }
            }
        }
        ColorKeyState::Conflict => {
            let conflict = ConflictKeyProps::from(&props);
            rsx! {
                ConflictKey { ..conflict }
            }
        }
    }
}

assert_component!(ColorKey);
