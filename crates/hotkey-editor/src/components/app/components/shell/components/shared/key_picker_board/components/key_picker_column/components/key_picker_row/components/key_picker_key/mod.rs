pub mod components;
mod logic;
mod props;

use crate::components::app::components::shell::components::shared::key_picker_board::KeyWidth;
use components::narrow_key_slot::{NarrowKeySlot, NarrowKeySlotProps};
use components::wide_key_slot::{WideKeySlot, WideKeySlotProps};
use dioxus::prelude::*;
use logic::KeyPickerKeyPresentation;
pub use props::KeyPickerKeyProps;
use tw_macro::assert_component;

/// A single key on the picker board: an on-screen keyboard button that assigns its key
/// when clicked. It carries no look of its own — it is the dispatcher that derives the
/// key's width from its cell and renders the matching sizing slot (`NarrowKeySlot` xor
/// `WideKeySlot`). Each slot owns the key's width and fills it with the color leaf that
/// owns the look. The body only chooses the width; the color is chosen further down.
#[component]
pub fn KeyPickerKey(props: KeyPickerKeyProps) -> Element {
    let presentation = KeyPickerKeyPresentation::from(&props);
    match presentation.width {
        KeyWidth::Standard => {
            let narrow = NarrowKeySlotProps::from(&presentation);
            rsx! {
                NarrowKeySlot { ..narrow }
            }
        }
        KeyWidth::Wide => {
            let wide = WideKeySlotProps::from(&presentation);
            rsx! {
                WideKeySlot { ..wide }
            }
        }
    }
}

assert_component!(KeyPickerKey);
