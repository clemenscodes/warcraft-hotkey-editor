pub mod components;
mod logic;
mod props;
mod state;

use components::available_key::{AvailableKey, AvailableKeyProps};
use components::conflict_key::{ConflictKey, ConflictKeyProps};
use components::current_key::{CurrentKey, CurrentKeyProps};
use dioxus::prelude::*;
use logic::KeyPickerKeyPresentation;
pub use props::KeyPickerKeyProps;
use state::KeyPickerKeyState;
use tw_macro::assert_component;
assert_component!(KeyPickerKey);

/// A single key on the picker board: an on-screen keyboard button that assigns its key
/// when clicked. It carries no look of its own — it is the dispatcher that derives the
/// key's visual state from its cell and renders the matching look component
/// (`AvailableKey` xor `CurrentKey` xor `ConflictKey`), each of which owns its own
/// button, styling, and conflict tooltip. The body only chooses which look to render.
#[component]
pub fn KeyPickerKey(props: KeyPickerKeyProps) -> Element {
    let presentation = KeyPickerKeyPresentation::from(&props);
    match presentation.state {
        KeyPickerKeyState::Available => {
            let available = AvailableKeyProps::from(&presentation);
            rsx! {
                AvailableKey { ..available }
            }
        }
        KeyPickerKeyState::Current => {
            let current = CurrentKeyProps::from(&presentation);
            rsx! {
                CurrentKey { ..current }
            }
        }
        KeyPickerKeyState::Conflict => {
            let conflict = ConflictKeyProps::from(&presentation);
            rsx! {
                ConflictKey { ..conflict }
            }
        }
    }
}
