pub mod components;
mod model;
mod view;

pub use view::SystemSlotKeyView;

use components::conflict_slot_key::ConflictSlotKey;
use components::plain_slot_key::PlainSlotKey;
use dioxus::prelude::*;
use model::SystemSlotKeyModel;
use tw_macro::assert_component;

/// The bound-key glyph shown on a system hotkey slot. A pure dispatcher: from the
/// slot's conflict flag it renders the matching look — `PlainSlotKey` (gold) xor
/// `ConflictSlotKey` (danger-red). The glyph size is inherited from the parent size
/// container, so both looks are size-agnostic.
#[component]
pub fn SystemSlotKey(props: SystemSlotKeyModel) -> Element {
    match props.conflict {
        false => {
            let label = props.label.clone();
            rsx! {
                PlainSlotKey { label }
            }
        }
        true => {
            let label = props.label.clone();
            rsx! {
                ConflictSlotKey { label }
            }
        }
    }
}

assert_component!(SystemSlotKey);
